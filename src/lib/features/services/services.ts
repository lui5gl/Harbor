import { siApache, siNodedotjs, siPhp } from "simple-icons";

export type ServiceDefinition = {
  name: string;
  description: string;
  iconPath: string;
  versions: string[];
  installedVersions: string[];
};

const nodeVersions = [
  "v24.4.1 (Current)",
  "v24.3.0 (Current)",
  "v22.17.1 (LTS)",
  "v22.16.0 (LTS)",
  "v22.15.1 (LTS)",
  "v20.19.4 (LTS)",
  "v20.19.3 (LTS)",
  "v20.18.3 (LTS)",
  "v18.20.8 (LTS)",
  "v18.20.7 (LTS)"
];

const apacheVersions = ["2.4.63", "2.4.62", "2.4.59"];
const phpVersions = ["8.4.10", "8.3.25", "8.2.29"];

export const serviceDefinitions: ServiceDefinition[] = [
  {
    name: "Apache",
    description: "Web server",
    iconPath: siApache.path,
    versions: apacheVersions,
    installedVersions: ["2.4.63"]
  },
  {
    name: "PHP",
    description: "Server-side scripting language",
    iconPath: siPhp.path,
    versions: phpVersions,
    installedVersions: ["8.4.10"]
  },
  {
    name: "Node.js",
    description: "Runtime environment",
    iconPath: siNodedotjs.path,
    versions: nodeVersions,
    installedVersions: ["v24.4.1 (Current)"]
  }
];
