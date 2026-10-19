use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_EXPAND_SZ, REG_SZ};
use winreg::RegKey;

static SECRETS_LOCK: Mutex<()> = Mutex::new(());
const SERVICE_NAME: &str = "Harbor";
const ACCOUNT_NAME: &str = "environment-profiles";
const VAULT_FILE: &str = "secrets.vault";
const VAULT_KEY_ACCOUNT: &str = "vault-master-key";
const VAULT_MAGIC: &[u8; 4] = b"HBRV";
const VAULT_VERSION: u8 = 1;
const VAULT_AAD: &[u8] = b"Harbor secrets vault v1";
const HARBOR_MANAGED_ENVIRONMENT_KEYS: &str = "HARBOR_MANAGED_ENVIRONMENT_KEYS";
const HARBOR_ACTIVE_PROFILE: &str = "HARBOR_ACTIVE_PROFILE";
const HARBOR_PROFILE_IS_PRODUCTION: &str = "HARBOR_PROFILE_IS_PRODUCTION";
const PROFILE_BLOCK_START: &str = "# >>> Harbor environment profile >>>";
const PROFILE_BLOCK_END: &str = "# <<< Harbor environment profile <<<";

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariable {
    pub id: u64,
    pub key: String,
    pub value: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentProfile {
    pub id: u64,
    pub name: String,
    pub is_production: bool,
    pub secrets: Vec<EnvironmentVariable>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretProject {
    pub id: u64,
    pub name: String,
    pub environments: Vec<EnvironmentProfile>,
}

#[derive(Clone, Deserialize, Serialize)]
struct ManagedEnvironmentVariable {
    key: String,
    previous_value: Option<String>,
    #[serde(default)]
    previous_expandable: bool,
}

#[derive(Clone, Deserialize, Serialize)]
struct RegistrySnapshot {
    key: String,
    bytes: Option<Vec<u8>>,
    expandable: bool,
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretsConfiguration {
    pub projects: Vec<SecretProject>,
    pub active_environment_id: Option<u64>,
    #[serde(default)]
    pub revision: u64,
    #[serde(default)]
    managed_environment_variables: Vec<ManagedEnvironmentVariable>,
    #[serde(default, skip_serializing)]
    pending_restore: Option<Vec<RegistrySnapshot>>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredSecretsConfiguration {
    #[serde(default)]
    projects: Vec<SecretProject>,
    #[serde(default)]
    active_environment_id: Option<u64>,
    #[serde(default)]
    profiles: Vec<EnvironmentProfile>,
    #[serde(default)]
    active_profile_id: Option<u64>,
    #[serde(default)]
    managed_environment_variables: Vec<ManagedEnvironmentVariable>,
    #[serde(default)]
    revision: u64,
    #[serde(default)]
    pending_restore: Option<Vec<RegistrySnapshot>>,
}

pub fn load() -> Result<SecretsConfiguration, String> {
    let _guard = SECRETS_LOCK.lock().map_err(|_| "Secrets storage is unavailable")?;
    recover_configuration()
}

fn vault_path() -> std::path::PathBuf {
    crate::runtime_paths::harbor_root().join("config").join(VAULT_FILE)
}

fn master_key_entry() -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, VAULT_KEY_ACCOUNT)
        .map_err(|error| format!("Unable to access the system credential store: {error}"))
}

fn get_or_create_master_key() -> Result<[u8; 32], String> {
    let entry = master_key_entry()?;
    match entry.get_secret() {
        Ok(secret) => {
            if secret.len() == 32 {
                let mut key = [0u8; 32];
                key.copy_from_slice(&secret);
                Ok(key)
            } else {
                Err("Vault master key in system credential store has invalid length".to_owned())
            }
        }
        Err(keyring::Error::NoEntry) => {
            let mut key = [0u8; 32];
            getrandom::fill(&mut key).map_err(|e| format!("Entropy error generating vault master key: {e}"))?;
            entry.set_secret(&key).map_err(|e| format!("Unable to save vault master key in system credential store: {e}"))?;
            Ok(key)
        }
        Err(error) => Err(format!("Unable to load vault master key: {error}")),
    }
}

fn encrypt_vault(payload: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    use aes_gcm::aead::{Aead, KeyInit, Payload};
    use aes_gcm::{Aes256Gcm, Nonce};

    let mut nonce_bytes = [0u8; 12];
    getrandom::fill(&mut nonce_bytes).map_err(|e| format!("Entropy failure: {e}"))?;
    let nonce = Nonce::from(nonce_bytes);
    let cipher = Aes256Gcm::new(key.into());

    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: payload,
                aad: VAULT_AAD,
            },
        )
        .map_err(|error| format!("Failed to encrypt vault payload: {error}"))?;

    let mut result = Vec::with_capacity(4 + 1 + 12 + ciphertext.len());
    result.extend_from_slice(VAULT_MAGIC);
    result.push(VAULT_VERSION);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

fn decrypt_vault(raw: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    use aes_gcm::aead::{Aead, KeyInit, Payload};
    use aes_gcm::{Aes256Gcm, Nonce};

    if raw.len() < 4 + 1 + 12 + 16 {
        return Err("Vault file is corrupted or too short".to_owned());
    }
    if &raw[0..4] != VAULT_MAGIC {
        return Err("Invalid vault file format".to_owned());
    }
    if raw[4] != VAULT_VERSION {
        return Err(format!("Unsupported vault version: {}", raw[4]));
    }
    let nonce_bytes: [u8; 12] = raw[5..17].try_into().map_err(|_| "Invalid nonce".to_string())?;
    let nonce = Nonce::from(nonce_bytes);
    let ciphertext = &raw[17..];

    let cipher = Aes256Gcm::new(key.into());
    cipher
        .decrypt(
            &nonce,
            Payload {
                msg: ciphertext,
                aad: VAULT_AAD,
            },
        )
        .map_err(|_| "Vault authentication failed: wrong master key or corrupted data".to_owned())
}

fn read_configuration() -> Result<SecretsConfiguration, String> {
    let path = vault_path();
    if path.exists() {
        let raw = fs::read(&path).map_err(|e| format!("Unable to read secrets vault: {e}"))?;
        let key = get_or_create_master_key()?;
        let decrypted = decrypt_vault(&raw, &key)?;
        let stored: StoredSecretsConfiguration = serde_json::from_slice(&decrypted)
            .map_err(|error| format!("Unable to parse saved secret profiles: {error}"))?;

        if !stored.projects.is_empty() || stored.profiles.is_empty() {
            return Ok(SecretsConfiguration {
                projects: stored.projects,
                active_environment_id: stored.active_environment_id,
                managed_environment_variables: stored.managed_environment_variables,
                revision: stored.revision,
                pending_restore: stored.pending_restore,
            });
        }

        return Ok(SecretsConfiguration {
            projects: vec![SecretProject {
                id: 1,
                name: "General".to_string(),
                environments: stored.profiles,
            }],
            active_environment_id: stored.active_profile_id,
            managed_environment_variables: stored.managed_environment_variables,
            revision: stored.revision,
            pending_restore: stored.pending_restore,
        });
    }

    let entry = credential_entry()?;
    match entry.get_secret() {
        Ok(contents) => {
            let stored: StoredSecretsConfiguration = serde_json::from_slice(&contents)
                .map_err(|error| format!("Unable to read saved secret profiles: {error}"))?;

            if !stored.projects.is_empty() || stored.profiles.is_empty() {
                return Ok(SecretsConfiguration {
                    projects: stored.projects,
                    active_environment_id: stored.active_environment_id,
                    managed_environment_variables: stored.managed_environment_variables,
                    revision: stored.revision,
                    pending_restore: stored.pending_restore,
                });
            }

            // Existing profile-only installations become a single project without losing secrets.
            Ok(SecretsConfiguration {
                projects: vec![SecretProject {
                    id: 1,
                    name: "General".to_string(),
                    environments: stored.profiles,
                }],
                active_environment_id: stored.active_profile_id,
                managed_environment_variables: stored.managed_environment_variables,
                revision: stored.revision,
                pending_restore: stored.pending_restore,
            })
        }
        Err(keyring::Error::NoEntry) => Ok(SecretsConfiguration::default()),
        Err(error) => Err(format!(
            "Unable to load secret profiles from the system credential store: {error}"
        )),
    }
}

pub fn save(mut configuration: SecretsConfiguration) -> Result<u64, String> {
    let _guard = SECRETS_LOCK.lock().map_err(|_| "Secrets storage is unavailable")?;
    let previous = recover_configuration()?;
    prepare_save(&mut configuration, &previous)?;
    validate(&configuration)?;
    write(&configuration)?;
    Ok(configuration.revision)
}

fn prepare_save(configuration: &mut SecretsConfiguration, previous: &SecretsConfiguration) -> Result<(), String> {
    if configuration.revision != previous.revision {
        return Err("Secrets changed in another window. Reload before saving; your edits have not been written.".to_owned());
    }
    if let Some(active_id) = previous.active_environment_id {
        let find_active = |configuration: &SecretsConfiguration| {
            configuration.projects.iter().flat_map(|project| &project.environments)
                .find(|environment| environment.id == active_id).cloned()
        };
        if find_active(configuration) != find_active(previous) {
            return Err("Deactivate the active environment before editing or deleting it.".to_owned());
        }
    }
    configuration.active_environment_id = previous.active_environment_id;
    configuration.managed_environment_variables = previous.managed_environment_variables.clone();
    configuration.pending_restore = None;
    configuration.revision = previous.revision.checked_add(1).ok_or("Secrets revision limit reached")?;
    Ok(())
}

pub fn activate_powershell_profile(profile_id: u64) -> Result<(), String> {
    let _guard = SECRETS_LOCK.lock().map_err(|_| "Secrets storage is unavailable")?;
    let mut configuration = recover_configuration()?;
    validate(&configuration)?;
    let profile = configuration
        .projects
        .iter()
        .flat_map(|project| project.environments.iter())
        .find(|environment| environment.id == profile_id)
        .cloned()
        .ok_or_else(|| "The selected profile does not exist".to_string())?;

    install_powershell_profile()?;
    let previous = configuration.clone();
    let snapshot = capture_environment(&configuration, Some(&profile))?;
    let mut journal = previous.clone();
    journal.pending_restore = Some(snapshot.clone());
    write(&journal)?;
    if let Err(error) = publish_user_environment(&profile, &mut configuration.managed_environment_variables) {
        return rollback_environment(&previous, &snapshot, error);
    }
    configuration.active_environment_id = Some(profile.id);
    configuration.revision = configuration.revision.checked_add(1).ok_or("Secrets revision limit reached")?;
    if let Err(error) = write(&configuration) {
        return rollback_environment(&previous, &snapshot, error);
    }
    Ok(())
}

pub fn deactivate() -> Result<(), String> {
    let _guard = SECRETS_LOCK.lock().map_err(|_| "Secrets storage is unavailable")?;
    let previous = recover_configuration()?;
    let snapshot = capture_environment(&previous, None)?;
    let mut journal = previous.clone();
    journal.pending_restore = Some(snapshot.clone());
    write(&journal)?;
    let mut configuration = previous.clone();
    let result = (|| {
        let environment = user_environment_key()?;
        for variable in &configuration.managed_environment_variables {
            restore_managed_variable(&environment, variable)?;
        }
        for key in metadata_keys() {
            delete_environment_value(&environment, key)?;
        }
        configuration.active_environment_id = None;
        configuration.managed_environment_variables.clear();
        configuration.revision = configuration.revision.checked_add(1).ok_or("Secrets revision limit reached")?;
        write(&configuration)
    })();
    match result {
        Ok(()) => Ok(()),
        Err(error) => rollback_environment(&previous, &snapshot, error),
    }
}

pub fn open_terminal(profile_id: u64, terminal_executable: &str) -> Result<(), String> {
    let _guard = SECRETS_LOCK.lock().map_err(|_| "Secrets storage is unavailable")?;
    let configuration = recover_configuration()?;
    validate(&configuration)?;
    let profile = configuration.projects.iter().flat_map(|project| &project.environments)
        .find(|environment| environment.id == profile_id)
        .ok_or("The selected environment does not exist")?;
    let (executable, terminal_kind) = match terminal_executable {
        "powershell" => {
            let system_root = std::env::var_os("SystemRoot").ok_or("Unable to locate Windows PowerShell")?;
            (Path::new(&system_root).join("System32/WindowsPowerShell/v1.0/powershell.exe"), "powershell")
        }
        "git-bash" => {
            let candidates = [
                std::env::var_os("ProgramFiles").map(|root| Path::new(&root).join("Git/bin/bash.exe")),
                std::env::var_os("ProgramFiles(x86)").map(|root| Path::new(&root).join("Git/bin/bash.exe")),
                std::env::var_os("LOCALAPPDATA").map(|root| Path::new(&root).join("Programs/Git/bin/bash.exe")),
            ];
            let path_candidate = std::env::var_os("PATH").into_iter()
                .flat_map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
                .map(|directory| directory.join("bash.exe"))
                .find(|path| path.is_file());
            let executable = candidates.into_iter().flatten().find(|path| path.is_file())
                .or(path_candidate)
                .ok_or("Git Bash was not found. Select its bash.exe using the custom terminal option.")?;
            (executable, "bash")
        }
        custom_path if !custom_path.trim().is_empty() => (Path::new(custom_path).to_path_buf(), "custom"),
        _ => return Err("Choose a terminal executable in Harbor settings.".to_owned()),
    };
    if terminal_kind == "custom" && !executable.is_file() {
        return Err("The configured terminal executable was not found.".to_owned());
    }
    let mut command = Command::new(executable);
    if terminal_kind == "powershell" {
        command.args(["-NoLogo", "-NoProfile", "-NoExit"]);
    } else if terminal_kind == "bash" {
        command.args(["--login", "-i"]);
    }
    command.creation_flags(0x00000010);
    for variable in &configuration.managed_environment_variables {
        command.env_remove(&variable.key);
        if let Some(previous) = &variable.previous_value {
            command.env(&variable.key, previous);
        }
    }
    for key in metadata_keys() { command.env_remove(key); }
    for variable in &profile.secrets { command.env(variable.key.trim(), &variable.value); }
    command.env(HARBOR_ACTIVE_PROFILE, &profile.name);
    command.env(HARBOR_PROFILE_IS_PRODUCTION, if profile.is_production { "1" } else { "0" });
    if let Some(home) = std::env::var_os("USERPROFILE") { command.current_dir(home); }
    let mut child = command.spawn().map_err(|error| format!("Unable to open the isolated PowerShell session: {error}"))?;
    std::thread::spawn(move || { let _ = child.wait(); });
    Ok(())
}

fn metadata_keys() -> [&'static str; 3] {
    [HARBOR_MANAGED_ENVIRONMENT_KEYS, HARBOR_ACTIVE_PROFILE, HARBOR_PROFILE_IS_PRODUCTION]
}

fn delete_environment_value(environment: &RegKey, key: &str) -> Result<(), String> {
    match environment.delete_value(key) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("Unable to remove environment variable {key}: {error}")),
    }
}

fn capture_environment(configuration: &SecretsConfiguration, profile: Option<&EnvironmentProfile>) -> Result<Vec<RegistrySnapshot>, String> {
    let environment = user_environment_key()?;
    let mut keys: HashSet<String> = configuration.managed_environment_variables.iter().map(|variable| variable.key.to_uppercase()).collect();
    keys.extend(metadata_keys().map(str::to_owned));
    if let Some(profile) = profile {
        keys.extend(profile.secrets.iter().map(|variable| variable.key.trim().to_uppercase()));
    }
    keys.into_iter().map(|key| {
        let raw = match environment.get_raw_value(&key) {
            Ok(raw) if raw.vtype == REG_SZ || raw.vtype == REG_EXPAND_SZ => Some(raw),
            Ok(_) => return Err(format!("Variable {key} has an unsupported Windows registry type; it was not changed")),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(format!("Unable to read environment variable {key}: {error}")),
        };
        Ok(RegistrySnapshot { key, expandable: raw.as_ref().is_some_and(|value| value.vtype == REG_EXPAND_SZ), bytes: raw.map(|value| value.bytes) })
    }).collect()
}

fn restore_snapshot(snapshot: &[RegistrySnapshot]) -> Result<(), String> {
    let environment = user_environment_key()?;
    for value in snapshot {
        if let Some(bytes) = &value.bytes {
            environment.set_raw_value(&value.key, &winreg::RegValue {
                bytes: bytes.clone(), vtype: if value.expandable { REG_EXPAND_SZ } else { REG_SZ },
            }).map_err(|error| format!("Unable to restore environment variable {}: {error}", value.key))?;
        } else {
            delete_environment_value(&environment, &value.key)?;
        }
    }
    Ok(())
}

fn rollback_environment(previous: &SecretsConfiguration, snapshot: &[RegistrySnapshot], error: String) -> Result<(), String> {
    restore_snapshot(snapshot).and_then(|_| write(previous)).map_err(|recovery| format!("{error}. Recovery is pending: {recovery}. Reopen Secrets to retry recovery."))?;
    Err(error)
}

fn recover_configuration() -> Result<SecretsConfiguration, String> {
    let mut configuration = read_configuration()?;
    if let Some(snapshot) = &configuration.pending_restore {
        restore_snapshot(snapshot)?;
        configuration.pending_restore = None;
        write(&configuration)?;
    }
    Ok(configuration)
}

fn credential_entry() -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|error| format!("Unable to access the system credential store: {error}"))
}

fn write(configuration: &SecretsConfiguration) -> Result<(), String> {
    let mut stored = serde_json::to_value(configuration).map_err(|error| error.to_string())?;
    if let Some(snapshot) = &configuration.pending_restore {
        stored["pendingRestore"] = serde_json::to_value(snapshot).map_err(|error| error.to_string())?;
    }
    let contents = serde_json::to_vec(&stored).map_err(|error| {
        format!("Unable to prepare secret profiles for secure storage: {error}")
    })?;

    let key = get_or_create_master_key()?;
    let encrypted = encrypt_vault(&contents, &key)?;

    let path = vault_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Unable to create vault directory: {e}"))?;
    }
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, &encrypted)
        .map_err(|error| format!("Unable to write secrets vault: {error}"))?;
    fs::rename(&temp_path, &path)
        .map_err(|error| format!("Unable to commit secrets vault: {error}"))?;

    if let Ok(entry) = credential_entry() {
        let _ = entry.delete_credential();
    }
    Ok(())
}

fn validate(configuration: &SecretsConfiguration) -> Result<(), String> {
    let mut project_ids = HashSet::new();
    let mut project_names = HashSet::new();
    let mut environment_ids = HashSet::new();

    for project in &configuration.projects {
        let project_name = project.name.trim();
        if project_name.is_empty() || project_name.len() > 80 {
            return Err("Each project must have a name between 1 and 80 characters".to_string());
        }
        if !project_ids.insert(project.id) {
            return Err("Project identifiers must be unique".to_string());
        }
        if !project_names.insert(project_name.to_lowercase()) {
            return Err("Project names must be unique".to_string());
        }

        let mut environment_names = HashSet::new();
        for environment in &project.environments {
            let name = environment.name.trim();
            if name.is_empty() || name.len() > 80 {
                return Err("Each environment must have a name between 1 and 80 characters".to_string());
            }
            if !environment_ids.insert(environment.id) {
                return Err("Environment identifiers must be unique".to_string());
            }
            if !environment_names.insert(name.to_lowercase()) {
                return Err(format!("Environment names in {project_name} must be unique"));
            }

            let mut variable_ids = HashSet::new();
            let mut variable_keys = HashSet::new();
            for variable in &environment.secrets {
                let key = variable.key.trim();
                if key.is_empty() || key.len() > 256 {
                    return Err(format!(
                        "Each variable in {name} must have a key between 1 and 256 characters"
                    ));
                }
                if !key
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '_')
                    || key.starts_with(|character: char| character.is_ascii_digit())
                {
                    return Err(format!(
                        "Variable {key} in {name} must start with a letter or underscore and use only letters, numbers, and underscores"
                    ));
                }
                if key.to_ascii_uppercase().starts_with("HARBOR_") {
                    return Err("Variable names beginning with HARBOR_ are reserved".to_string());
                }
                if variable.value.contains('\0') {
                    return Err(format!(
                        "Variable {key} in {name} cannot contain a null character"
                    ));
                }
                if !variable_ids.insert(variable.id) {
                    return Err(format!("Variable identifiers in {name} must be unique"));
                }
                if !variable_keys.insert(key.to_uppercase()) {
                    return Err(format!("Variable keys in {name} must be unique"));
                }
            }
        }
    }

    if let Some(active_environment_id) = configuration.active_environment_id {
        if !environment_ids.contains(&active_environment_id) {
            return Err("The active environment must exist".to_string());
        }
    }
    Ok(())
}

fn publish_user_environment(
    profile: &EnvironmentProfile,
    managed_variables: &mut Vec<ManagedEnvironmentVariable>,
) -> Result<(), String> {
    let environment = user_environment_key()?;
    let active_variables = profile
        .secrets
        .iter()
        .map(|variable| (variable.key.trim().to_uppercase(), variable))
        .collect::<HashMap<_, _>>();

    for managed_variable in managed_variables
        .iter()
        .filter(|variable| !active_variables.contains_key(&variable.key.to_uppercase()))
    {
        restore_managed_variable(&environment, managed_variable)?;
    }

    let previous_values = managed_variables
        .iter()
        .map(|variable| (variable.key.to_uppercase(), (variable.previous_value.clone(), variable.previous_expandable)))
        .collect::<HashMap<_, _>>();
    let mut next_managed_variables = Vec::with_capacity(active_variables.len());
    for variable in &profile.secrets {
        let key = variable.key.trim();
        let key_uppercase = key.to_uppercase();
        let (previous_value, previous_expandable) = previous_values
            .get(&key_uppercase)
            .cloned()
            .unwrap_or_else(|| (environment.get_value::<String, _>(key).ok(), environment.get_raw_value(key).is_ok_and(|raw| raw.vtype == REG_EXPAND_SZ)));
        environment
            .set_value(key, &variable.value)
            .map_err(|error| {
                format!("Unable to publish the user environment variable {key}: {error}")
            })?;
        next_managed_variables.push(ManagedEnvironmentVariable {
            key: key.to_owned(),
            previous_value,
            previous_expandable,
        });
    }

    environment
        .set_value(
            HARBOR_MANAGED_ENVIRONMENT_KEYS,
            &profile
                .secrets
                .iter()
                .map(|variable| variable.key.trim())
                .collect::<Vec<_>>()
                .join(","),
        )
        .map_err(|error| format!("Unable to publish Harbor environment metadata: {error}"))?;
    environment
        .set_value(HARBOR_ACTIVE_PROFILE, &profile.name.trim().to_owned())
        .map_err(|error| format!("Unable to publish the active Harbor profile: {error}"))?;
    environment
        .set_value(
            HARBOR_PROFILE_IS_PRODUCTION,
            &if profile.is_production {
                "1".to_owned()
            } else {
                "0".to_owned()
            },
        )
        .map_err(|error| format!("Unable to publish Harbor profile metadata: {error}"))?;
    *managed_variables = next_managed_variables;
    Ok(())
}

fn user_environment_key() -> Result<RegKey, String> {
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)
        .map_err(|error| format!("Unable to access the user environment registry: {error}"))
}

fn restore_managed_variable(environment: &RegKey, variable: &ManagedEnvironmentVariable) -> Result<(), String> {
    match &variable.previous_value {
        Some(value) => environment.set_raw_value(&variable.key, &winreg::RegValue {
            bytes: value.encode_utf16().chain(Some(0)).flat_map(u16::to_le_bytes).collect(),
            vtype: if variable.previous_expandable { REG_EXPAND_SZ } else { REG_SZ },
        }).map_err(|error| format!("Unable to restore variable {}: {error}", variable.key)),
        None => delete_environment_value(environment, &variable.key),
    }
}

fn install_powershell_profile() -> Result<(), String> {
    let user_profile = std::env::var_os("USERPROFILE")
        .ok_or_else(|| "Unable to find the current Windows user profile".to_string())?;
    let documents_directory = Path::new(&user_profile).join("Documents");
    let block = managed_powershell_block();
    for directory in ["WindowsPowerShell", "PowerShell"] {
        let profile_path = documents_directory
            .join(directory)
            .join("Microsoft.PowerShell_profile.ps1");
        install_managed_profile_block(&profile_path, &block)?;
    }
    Ok(())
}

fn managed_powershell_block() -> String {
    format!("{PROFILE_BLOCK_START}\r\n$harborProfileName = [Environment]::GetEnvironmentVariable('{HARBOR_ACTIVE_PROFILE}', 'User')\r\n$harborManagedKeys = [Environment]::GetEnvironmentVariable('{HARBOR_MANAGED_ENVIRONMENT_KEYS}', 'User')\r\nif ($harborManagedKeys) {{\r\n  foreach ($harborKey in ($harborManagedKeys -split ',')) {{\r\n    $harborValue = [Environment]::GetEnvironmentVariable($harborKey, 'User')\r\n    if ($null -ne $harborValue) {{ Set-Item -Path \"Env:$harborKey\" -Value $harborValue }}\r\n  }}\r\n}}\r\nif ($harborProfileName) {{\r\n  $harborIsProduction = [Environment]::GetEnvironmentVariable('{HARBOR_PROFILE_IS_PRODUCTION}', 'User') -eq '1'\r\n  if ($harborIsProduction) {{\r\n    Write-Host \"Harbor > [!] $harborProfileName profile loaded\" -ForegroundColor Yellow\r\n  }} else {{\r\n    Write-Host \"Harbor > $harborProfileName profile loaded\" -ForegroundColor Cyan\r\n  }}\r\n}}\r\n{PROFILE_BLOCK_END}\r\n")
}

fn install_managed_profile_block(path: &Path, block: &str) -> Result<(), String> {
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory).map_err(|error| {
            format!("Unable to create the PowerShell profile directory: {error}")
        })?;
    }
    let existing = if path.exists() {
        fs::read_to_string(path)
            .map_err(|error| format!("Unable to read the PowerShell profile: {error}"))?
    } else {
        String::new()
    };
    let contents = replace_managed_block(existing.trim_start_matches('\u{feff}'), block)?;
    let mut utf8_contents = vec![0xEF, 0xBB, 0xBF];
    utf8_contents.extend_from_slice(contents.as_bytes());
    fs::write(path, utf8_contents)
        .map_err(|error| format!("Unable to update the PowerShell profile: {error}"))
}

fn replace_managed_block(existing: &str, block: &str) -> Result<String, String> {
    let Some(start) = existing.find(PROFILE_BLOCK_START) else {
        return Ok(if existing.trim().is_empty() {
            block.to_owned()
        } else {
            format!("{existing}\r\n\r\n{block}")
        });
    };
    let end = existing[start..]
        .find(PROFILE_BLOCK_END)
        .map(|offset| start + offset + PROFILE_BLOCK_END.len())
        .ok_or_else(|| "The existing Harbor PowerShell profile block is incomplete".to_string())?;
    let mut updated = String::with_capacity(existing.len() + block.len());
    updated.push_str(&existing[..start]);
    updated.push_str(block);
    updated.push_str(&existing[end..]);
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn configuration_with_key(key: &str) -> SecretsConfiguration {
        SecretsConfiguration {
            projects: vec![SecretProject {
                id: 1,
                name: "Project".to_owned(),
                environments: vec![EnvironmentProfile {
                    id: 2,
                    name: "Development".to_owned(),
                    is_production: false,
                    secrets: vec![EnvironmentVariable {
                        id: 3,
                        key: key.to_owned(),
                        value: "test-value".to_owned(),
                    }],
                }],
            }],
            ..SecretsConfiguration::default()
        }
    }

    #[test]
    fn rejects_stale_saves_and_preserves_native_activation() {
        let mut previous = configuration_with_key("API_KEY");
        previous.revision = 4;
        previous.active_environment_id = Some(2);
        let mut incoming = configuration_with_key("API_KEY");
        assert!(prepare_save(&mut incoming, &previous).is_err());
        incoming.revision = 4;
        prepare_save(&mut incoming, &previous).unwrap();
        assert_eq!(incoming.revision, 5);
        assert_eq!(incoming.active_environment_id, Some(2));
        incoming.revision = 4;
        incoming.projects.clear();
        assert!(prepare_save(&mut incoming, &previous).is_err());
    }

    #[test]
    fn recovery_journal_is_not_exposed_to_frontend() {
        let mut configuration = configuration_with_key("API_KEY");
        configuration.pending_restore = Some(vec![RegistrySnapshot { key: "PRIVATE".to_owned(), bytes: Some(vec![1, 2]), expandable: false }]);
        let serialized = serde_json::to_value(&configuration).unwrap();
        assert!(serialized.get("pendingRestore").is_none());
    }

    #[test]
    fn validates_portable_variable_names() {
        for key in ["API_KEY", "_TOKEN", "DATABASE_URL"] {
            assert!(validate(&configuration_with_key(key)).is_ok());
        }
        for key in ["1TOKEN", "API-KEY", "API KEY", ""] {
            assert!(validate(&configuration_with_key(key)).is_err());
        }
    }

    #[test]
    fn vault_round_trips_large_payloads_above_credential_limit() {
        let mut key = [0u8; 32];
        getrandom::fill(&mut key).unwrap();
        // 100 KB payload - significantly exceeds the 2560 byte Windows Credential Manager limit
        let large_payload = vec![0x42u8; 102_400];
        let encrypted = encrypt_vault(&large_payload, &key).expect("encryption succeeds");
        let decrypted = decrypt_vault(&encrypted, &key).expect("decryption succeeds");
        assert_eq!(decrypted, large_payload);
    }

    #[test]
    fn vault_rejects_tampered_data_and_wrong_key() {
        let mut key1 = [0u8; 32];
        let mut key2 = [0u8; 32];
        getrandom::fill(&mut key1).unwrap();
        getrandom::fill(&mut key2).unwrap();
        let payload = b"critical-secrets-payload";
        let mut encrypted = encrypt_vault(payload, &key1).unwrap();

        // Wrong key must fail authenticated decryption
        assert!(decrypt_vault(&encrypted, &key2).is_err());

        // Bit tampering in ciphertext must fail
        let last_idx = encrypted.len() - 1;
        encrypted[last_idx] ^= 0x01;
        assert!(decrypt_vault(&encrypted, &key1).is_err());

        // Invalid header format must fail
        encrypted[0] = b'X';
        assert!(decrypt_vault(&encrypted, &key1).is_err());
    }

    #[test]
    fn rejects_reserved_metadata_names_case_insensitively() {
        for key in [HARBOR_ACTIVE_PROFILE, "harbor_custom", HARBOR_MANAGED_ENVIRONMENT_KEYS] {
            assert!(validate(&configuration_with_key(key)).is_err());
        }
    }

    #[test]
    fn replaces_only_the_managed_powershell_block() {
        let existing = format!("before\r\n{PROFILE_BLOCK_START}\r\nold\r\n{PROFILE_BLOCK_END}\r\nafter");
        let updated = replace_managed_block(&existing, "replacement").unwrap();
        assert_eq!(updated, "before\r\nreplacement\r\nafter");
        assert!(replace_managed_block(PROFILE_BLOCK_START, "replacement").is_err());
    }
}
