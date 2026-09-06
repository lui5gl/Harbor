export type RuntimeVersionParts = {
  number: string;
  channel: string;
};

export type RuntimeChannel = "current" | "lts" | "security" | "active" | "eol";

export function parseRuntimeVersion(version: string): RuntimeVersionParts {
  const match = version.match(/^(.*) \((.*)\)$/);
  if (!match) return { number: version, channel: "" };
  return { number: match[1], channel: match[2] };
}

export function cleanRuntimeVersion(version: string): string {
  return parseRuntimeVersion(version).number.trimStart().replace(/^v/, "");
}

export function formatRuntimeChannel(channel: string): string {
  return channel.replace("LTS - ", "LTS · ");
}

export function getRuntimeChannelClass(channel: string): RuntimeChannel {
  if (channel.startsWith("EOL")) return "eol";
  if (channel.startsWith("LTS")) return "lts";
  if (channel.startsWith("Security")) return "security";
  if (channel.startsWith("Active")) return "active";
  return "current";
}
