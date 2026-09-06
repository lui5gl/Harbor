import { en } from "./locales/en";
import { es } from "./locales/es";

export type Locale = "es" | "en";

type Dictionary = {
  readonly [key: string]: string | Dictionary;
};

const STORAGE_KEY = "harbor-settings";
const dictionaries: Record<Locale, Dictionary> = { es, en };

function readInitialLocale(): Locale {
  if (typeof window === "undefined") return "es";

  try {
    const stored = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "{}");
    return stored.locale === "en" ? "en" : "es";
  } catch {
    return "es";
  }
}

export const i18n = $state({ locale: readInitialLocale() });

export function setLocale(nextLocale: Locale) {
  i18n.locale = nextLocale;
}

export function persistLocale(nextLocale: Locale) {
  setLocale(nextLocale);

  if (typeof window === "undefined") return;

  try {
    const stored = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "{}");
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ ...stored, locale: nextLocale }));
  } catch {
    // The interface remains usable if local persistence fails.
  }
}

export function t(key: string): string {
  const value = key.split(".").reduce<unknown>((current, segment) => {
    if (!current || typeof current !== "object") return undefined;
    return (current as Record<string, unknown>)[segment];
  }, dictionaries[i18n.locale]);

  return typeof value === "string" ? value : key;
}
