export const RuntimeService = {
  PHP: "PHP",
  Apache: "Apache",
  Node: "Node.js",
} as const;

export type RuntimeService = (typeof RuntimeService)[keyof typeof RuntimeService];

export const RuntimeServiceCommand = {
  getCatalog: {
    [RuntimeService.PHP]: "get_php_versions",
    [RuntimeService.Apache]: "get_apache_versions",
    [RuntimeService.Node]: "get_node_versions",
  },
  install: {
    [RuntimeService.PHP]: "install_php",
    [RuntimeService.Apache]: "install_apache",
    [RuntimeService.Node]: "install_node",
  },
  activate: {
    [RuntimeService.PHP]: "set_active_php_version",
    [RuntimeService.Apache]: "set_active_apache_version",
    [RuntimeService.Node]: "set_active_node_version",
  },
} as const;

export const RuntimeTiming = {
  catalogCacheMs: 30 * 60 * 1000,
  mockInstallDelayMs: 500,
  quickTrayStatusRefreshMs: 4_000,
} as const;
