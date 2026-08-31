use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
use winreg::RegKey;

const SERVICE_NAME: &str = "Harbor";
const ACCOUNT_NAME: &str = "environment-profiles";
const HARBOR_MANAGED_ENVIRONMENT_KEYS: &str = "HARBOR_MANAGED_ENVIRONMENT_KEYS";
const HARBOR_ACTIVE_PROFILE: &str = "HARBOR_ACTIVE_PROFILE";
const HARBOR_PROFILE_IS_PRODUCTION: &str = "HARBOR_PROFILE_IS_PRODUCTION";
const PROFILE_BLOCK_START: &str = "# >>> Harbor environment profile >>>";
const PROFILE_BLOCK_END: &str = "# <<< Harbor environment profile <<<";

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariable {
    pub id: u64,
    pub key: String,
    pub value: String,
}

#[derive(Clone, Deserialize, Serialize)]
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
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretsConfiguration {
    pub projects: Vec<SecretProject>,
    pub active_environment_id: Option<u64>,
    #[serde(default)]
    managed_environment_variables: Vec<ManagedEnvironmentVariable>,
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
}

pub fn load() -> Result<SecretsConfiguration, String> {
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
            })
        }
        Err(keyring::Error::NoEntry) => Ok(SecretsConfiguration::default()),
        Err(error) => Err(format!(
            "Unable to load secret profiles from the system credential store: {error}"
        )),
    }
}

pub fn save(mut configuration: SecretsConfiguration) -> Result<(), String> {
    configuration.managed_environment_variables = load()
        .map(|existing| existing.managed_environment_variables)
        .unwrap_or_default();
    validate(&configuration)?;
    write(&configuration)
}

pub fn activate_powershell_profile(profile_id: u64) -> Result<(), String> {
    let mut configuration = load()?;
    validate(&configuration)?;
    let profile = configuration
        .projects
        .iter()
        .flat_map(|project| project.environments.iter())
        .find(|environment| environment.id == profile_id)
        .cloned()
        .ok_or_else(|| "The selected profile does not exist".to_string())?;

    publish_user_environment(&profile, &mut configuration.managed_environment_variables)?;
    install_powershell_profile()?;
    configuration.active_environment_id = Some(profile.id);
    write(&configuration)
}

fn credential_entry() -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|error| format!("Unable to access the system credential store: {error}"))
}

fn write(configuration: &SecretsConfiguration) -> Result<(), String> {
    let contents = serde_json::to_vec(configuration).map_err(|error| {
        format!("Unable to prepare secret profiles for secure storage: {error}")
    })?;
    credential_entry()?.set_secret(&contents).map_err(|error| {
        format!("Unable to save secret profiles in the system credential store: {error}")
    })
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
                {
                    return Err(format!(
                        "Variable {key} in {name} may only use letters, numbers, and underscores"
                    ));
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
        match &managed_variable.previous_value {
            Some(value) => environment.set_value(&managed_variable.key, value),
            None => {
                let _ = environment.delete_value(&managed_variable.key);
                Ok(())
            }
        }
        .map_err(|error| {
            format!(
                "Unable to restore the user environment variable {}: {error}",
                managed_variable.key
            )
        })?;
    }

    let previous_values = managed_variables
        .iter()
        .map(|variable| (variable.key.to_uppercase(), variable.previous_value.clone()))
        .collect::<HashMap<_, _>>();
    let mut next_managed_variables = Vec::with_capacity(active_variables.len());
    for variable in &profile.secrets {
        let key = variable.key.trim();
        let key_uppercase = key.to_uppercase();
        let previous_value = previous_values
            .get(&key_uppercase)
            .cloned()
            .unwrap_or_else(|| environment.get_value::<String, _>(key).ok());
        environment
            .set_value(key, &variable.value)
            .map_err(|error| {
                format!("Unable to publish the user environment variable {key}: {error}")
            })?;
        next_managed_variables.push(ManagedEnvironmentVariable {
            key: key.to_owned(),
            previous_value,
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
