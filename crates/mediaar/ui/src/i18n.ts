import { FluentBundle, FluentResource } from "@fluent/bundle";
import enCommon from "@locales/en/common.ftl?raw";
import enDesktop from "@locales/en/desktop.ftl?raw";
import esCommon from "@locales/es/common.ftl?raw";
import esDesktop from "@locales/es/desktop.ftl?raw";

export const AVAILABLE = ["en", "es"] as const;
export type LocaleTag = (typeof AVAILABLE)[number];

const SOURCES: Record<LocaleTag, string[]> = {
  en: [enCommon, enDesktop],
  es: [esCommon, esDesktop],
};

const bundles = new Map<LocaleTag, FluentBundle>();

function loadBundle(tag: LocaleTag): FluentBundle {
  const cached = bundles.get(tag);
  if (cached) {
    return cached;
  }

  const bundle = new FluentBundle(tag, { useIsolating: false });
  for (const source of SOURCES[tag]) {
    const errors = bundle.addResource(new FluentResource(source));
    if (errors.length > 0) {
      console.warn(`fluent resource warnings (${tag})`, errors);
    }
  }
  bundles.set(tag, bundle);
  return bundle;
}

export function negotiate(raw: string | null | undefined): LocaleTag {
  if (!raw) {
    return "en";
  }
  const normalized = raw.trim().replaceAll("_", "-").split(".")[0]?.split("@")[0]?.toLowerCase();
  if (!normalized) {
    return "en";
  }
  if ((AVAILABLE as readonly string[]).includes(normalized)) {
    return normalized as LocaleTag;
  }
  const primary = normalized.split("-")[0] ?? "en";
  if ((AVAILABLE as readonly string[]).includes(primary)) {
    return primary as LocaleTag;
  }
  return "en";
}

export function cycle(current: string): LocaleTag {
  const tag = negotiate(current);
  const idx = AVAILABLE.indexOf(tag);
  return AVAILABLE[(idx + 1) % AVAILABLE.length] ?? "en";
}

export function systemLocale(): LocaleTag {
  if (typeof navigator === "undefined") {
    return "en";
  }
  return negotiate(navigator.language);
}

type Args = Record<string, string | number | Date>;

export function t(tag: string, id: string, args?: Args): string {
  const bundle = loadBundle(negotiate(tag));
  const message = bundle.getMessage(id);
  if (!message?.value) {
    return id;
  }
  const errors: Error[] = [];
  const value = bundle.formatPattern(message.value, args, errors);
  if (errors.length > 0) {
    console.warn(`fluent format warnings (${tag}/${id})`, errors);
  }
  return value;
}

export function tAttr(tag: string, id: string, attr: string, args?: Args): string {
  const bundle = loadBundle(negotiate(tag));
  const message = bundle.getMessage(id);
  const attribute = message?.attributes.get(attr);
  if (!attribute) {
    return `${id}.${attr}`;
  }
  const errors: Error[] = [];
  const value = bundle.formatPattern(attribute, args, errors);
  if (errors.length > 0) {
    console.warn(`fluent format warnings (${tag}/${id}.${attr})`, errors);
  }
  return value;
}

export function formatSessionDate(tag: string): string {
  return new Intl.DateTimeFormat(negotiate(tag), {
    month: "long",
    day: "numeric",
    year: "numeric",
  }).format(new Date());
}

export function showcaseArgs(tag: string, name = "Alex"): Args {
  return {
    name,
    count: 3,
    gigabytes: 1.5,
    date: formatSessionDate(tag),
  };
}
