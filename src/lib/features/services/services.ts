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
    description: "Local web server",
    iconPath: siApache.path,
    versions: [],
    installedVersions: []
  },
  {
    name: "PHP",
    description: "PHP runtime",
    iconPath: siPhp.path,
    versions: [],
    installedVersions: []
  },
  {
    name: "Node.js",
    description: "JavaScript runtime",
    iconPath: siNodedotjs.path,
    versions: [],
    installedVersions: []
  }
];
