import type { PhpStackProfile } from "../types";
import { cleanVersion } from "../types";

const PROFILE_STORAGE_KEY = "harbor_php_stack_profiles";
const DEFAULT_PROFILE_ID = "default-stack";
const DEFAULT_PROFILE_NAME = "Default Web Stack";
const DERIVED_PROFILE_PREFIX = "stack-php-";
const DERIVED_PROFILE_NAME_PREFIX = "PHP ";
const DERIVED_PROFILE_NAME_SUFFIX = " Environment";

export function createPhpStackProfiles(): {
  readonly profiles: PhpStackProfile[];
  load: () => void;
  save: (profiles: PhpStackProfile[]) => void;
  ensureDefaults: (phpVersions: string[], apacheVersions: string[]) => void;
} {
  let profiles = $state<PhpStackProfile[]>([]);

  function load() {
    if (typeof window === "undefined") return;
    const stored = localStorage.getItem(PROFILE_STORAGE_KEY);
    if (!stored) return;
    try {
      const parsed: unknown = JSON.parse(stored);
      if (Array.isArray(parsed)) profiles = parsed as PhpStackProfile[];
    } catch {
      localStorage.removeItem(PROFILE_STORAGE_KEY);
    }
  }

  function save(nextProfiles: PhpStackProfile[]) {
    profiles = nextProfiles;
    if (typeof window !== "undefined") {
      localStorage.setItem(PROFILE_STORAGE_KEY, JSON.stringify(nextProfiles));
    }
  }

  function ensureDefaults(phpVersions: string[], apacheVersions: string[]) {
    if (profiles.length > 0 || (phpVersions.length === 0 && apacheVersions.length === 0)) return;
    const phpVersion = cleanVersion(phpVersions[0] ?? "");
    const apacheVersion = cleanVersion(apacheVersions[0] ?? "");
    if (!phpVersion || !apacheVersion) return;
    const initial: PhpStackProfile[] = [
      {
        id: DEFAULT_PROFILE_ID,
        name: DEFAULT_PROFILE_NAME,
        phpVersion,
        apacheVersion,
        isDefault: true,
      },
    ];

    phpVersions.forEach((rawVersion) => {
      const version = cleanVersion(rawVersion);
      if (version === phpVersion) return;
      initial.push({
        id: `${DERIVED_PROFILE_PREFIX}${version.replace(/\./g, "-")}`,
        name: `${DERIVED_PROFILE_NAME_PREFIX}${version}${DERIVED_PROFILE_NAME_SUFFIX}`,
        phpVersion: version,
        apacheVersion,
      });
    });

    save(initial);
  }

  return {
    get profiles() {
      return profiles;
    },
    load,
    save,
    ensureDefaults,
  };
}
