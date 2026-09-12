import {
  action,
  createOptimistic,
  createSignal,
  isPending,
  Loading,
  refresh,
  Show,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import {
  cycle,
  negotiate,
  showcaseArgs,
  systemLocale,
  t,
  tAttr,
  type LocaleTag,
} from "./i18n";
import "./App.css";

type Theme = "latte" | "mocha";

type LocaleInfo = {
  locale: string;
  available: string[];
  fromCli: boolean;
};

const THEME_KEY = "mediaar-theme";
const LANG_KEY = "mediaar-lang";

function readStoredTheme(): Theme {
  if (typeof window === "undefined") {
    return "mocha";
  }
  const stored = localStorage.getItem(THEME_KEY);
  if (stored === "latte" || stored === "mocha") {
    return stored;
  }
  return window.matchMedia("(prefers-color-scheme: light)").matches
    ? "latte"
    : "mocha";
}

function readCachedLang(): LocaleTag {
  if (typeof window === "undefined") {
    return "en";
  }
  const stored = localStorage.getItem(LANG_KEY);
  return stored ? negotiate(stored) : systemLocale();
}

async function fetchLocale(): Promise<LocaleTag> {
  try {
    const info = await invoke<LocaleInfo>("get_locale");
    const tag = negotiate(info.locale);
    localStorage.setItem(LANG_KEY, tag);
    return tag;
  } catch {
    return readCachedLang();
  }
}

async function persistLocale(tag: LocaleTag): Promise<LocaleTag> {
  localStorage.setItem(LANG_KEY, tag);
  try {
    const info = await invoke<LocaleInfo>("set_locale", { locale: tag });
    return negotiate(info.locale);
  } catch {
    return tag;
  }
}

export default function App() {
  const [theme, setTheme] = createSignal<Theme>(readStoredTheme());

  // Async locale is a computation (Solid 2); Loading owns first-ready UI.
  const [lang, setLang] = createOptimistic(() => fetchLocale());

  function toggleTheme() {
    setTheme((current) => {
      const next: Theme = current === "mocha" ? "latte" : "mocha";
      localStorage.setItem(THEME_KEY, next);
      return next;
    });
  }

  const toggleLang = action(function* () {
    const next = cycle(lang());
    setLang(next);
    yield persistLocale(next);
    refresh(lang);
  });

  return (
    <Loading
      fallback={
        <main class={`welcome ${theme()}`}>
          <p class="welcome__showcase-status" aria-live="polite">
            …
          </p>
        </main>
      }
    >
      <main class={`welcome ${theme()}`} lang={lang()}>
        <div class="welcome__atmosphere" aria-hidden="true" />
        <header class="welcome__header">
          <p class="welcome__brand">{t(lang(), "app-brand")}</p>
          <div class="welcome__controls">
            <button
              type="button"
              class="welcome__toggle"
              onClick={toggleLang}
              disabled={isPending(() => lang())}
              aria-label={tAttr(lang(), "lang-toggle", "aria-label", {
                lang: lang(),
              })}
            >
              <span class="welcome__toggle-label">
                {tAttr(lang(), "lang-toggle", "label", { lang: lang() })}
              </span>
              <span class="welcome__toggle-hint">
                {tAttr(lang(), "lang-toggle", "hint")}
              </span>
            </button>
            <button
              type="button"
              class="welcome__toggle"
              onClick={toggleTheme}
              aria-label={tAttr(lang(), "theme-toggle", "aria-label", {
                theme: theme(),
                next: theme() === "mocha" ? "latte" : "mocha",
              })}
            >
              <span class="welcome__toggle-label">
                {tAttr(lang(), "theme-toggle", "label", { theme: theme() })}
              </span>
              <span class="welcome__toggle-hint">
                {tAttr(lang(), "theme-toggle", "hint", { theme: theme() })}
              </span>
            </button>
          </div>
        </header>

        <section class="welcome__hero">
          <h1 class="welcome__title">{t(lang(), "welcome-title")}</h1>
          <p class="welcome__lede">{t(lang(), "welcome-lede")}</p>
        </section>

        <section class="welcome__showcase" aria-labelledby="fluent-showcase">
          <h2 id="fluent-showcase" class="welcome__showcase-title">
            {t(lang(), "showcase-heading")}
          </h2>
          <p class="welcome__showcase-intro">{t(lang(), "showcase-intro")}</p>
          <ul class="welcome__showcase-list">
            <li>{t(lang(), "showcase-placeable", showcaseArgs(lang()))}</li>
            <li>{t(lang(), "showcase-plural", showcaseArgs(lang()))}</li>
            <li>{t(lang(), "showcase-number", showcaseArgs(lang()))}</li>
            <li>{t(lang(), "showcase-date", showcaseArgs(lang()))}</li>
            <li>{t(lang(), "showcase-nested", showcaseArgs(lang()))}</li>
          </ul>
          <Show when={isPending(() => lang())}>
            <p class="welcome__showcase-status" aria-live="polite">
              …
            </p>
          </Show>
        </section>
      </main>
    </Loading>
  );
}
