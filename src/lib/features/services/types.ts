export type ServiceId = "php-web" | "nodejs";

export type VersionChannel = "Active" | "Security" | "EOL" | "LTS" | "Current" | "Unknown";

export type VersionItem = {
  raw: string;
  version: string;
  channel: string;
  isInstalled: boolean;
};

export type PhpStackProfile = {
  id: string;
  name: string;
  phpVersion: string;
  apacheVersion: string;
  isDefault?: boolean;
};

export type PhpWebStackState = {
  activePhpVersion: string | null;
  installedPhpVersions: string[];
  availablePhpVersions: string[];
  activeApacheVersion: string | null;
  installedApacheVersions: string[];
  availableApacheVersions: string[];
  isPhpRunning: boolean;
  fastCgiAddress: string;
  wwwPath: string;
};

export type NodeRuntimeState = {
  activeNodeVersion: string | null;
  installedNodeVersions: string[];
  availableNodeVersions: string[];
};

export function parseVersionString(raw: string): { version: string; channel: string } {
  const match = raw.match(/^(.*?)(?:\s+\((.*?)\))?$/);
  if (!match) {
    return { version: raw.trim().replace(/^v/, ""), channel: "" };
  }
  return {
    version: (match[1] ?? raw).trim().replace(/^v/, ""),
    channel: (match[2] ?? "").trim()
  };
}

export function cleanVersion(raw: string): string {
  return parseVersionString(raw).version;
}

export function getCompatibleApacheVersions(
  phpVersion: string | null,
  availableApache: string[]
): { recommended: string[]; all: string[]; compatibilityNote: string } {
  if (!phpVersion) {
    return {
      recommended: availableApache.slice(0, 3),
      all: availableApache,
      compatibilityNote: "Select a PHP version to verify Apache FastCGI binary compatibility."
    };
  }

  const clean = cleanVersion(phpVersion);
  const major = clean.split(".")[0];

  // PHP 8.x on Windows uses VS16/VS17 toolchains matching Apache 2.4.x VS17/VS16 binaries from ApacheLounge.
  if (major === "8") {
    return {
      recommended: availableApache.filter((v) => v.startsWith("2.4.")).slice(0, 4),
      all: availableApache,
      compatibilityNote: `PHP ${clean} (VS16/VS17 x64) is fully compatible with Apache 2.4.x via FastCGI (127.0.0.1:9070).`
    };
  }

  return {
    recommended: availableApache.slice(0, 3),
    all: availableApache,
    compatibilityNote: `PHP ${clean} is configured to run via FastCGI on 127.0.0.1:9070 with Apache 2.4.`
  };
}
