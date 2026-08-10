import { siApache, siNodedotjs, siPhp } from "simple-icons";

export type ServiceDefinition = {
  name: string;
  description: string;
  iconPath: string;
  versions: string[];
  installedVersions: string[];
};

export const serviceDefinitions: ServiceDefinition[] = [
  {
    name: "Apache",
    description: "Web server",
    iconPath: siApache.path,
    versions: [],
    installedVersions: []
  },
  {
    name: "PHP",
    description: "Server-side scripting language",
    iconPath: siPhp.path,
    versions: [],
    installedVersions: []
  },
  {
    name: "Node.js",
    description: "Runtime environment",
    iconPath: siNodedotjs.path,
    versions: [],
    installedVersions: []
  }
];
