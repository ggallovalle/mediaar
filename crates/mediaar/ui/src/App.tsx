import { createSignal } from "solid-js";
import "./App.css";

type Theme = "latte" | "mocha";

const STORAGE_KEY = "mediaar-theme";

function readStoredTheme(): Theme {
  if (typeof window === "undefined") {
    return "mocha";
  }
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === "latte" || stored === "mocha") {
    return stored;
  }
  return window.matchMedia("(prefers-color-scheme: light)").matches
    ? "latte"
    : "mocha";
}

function applyTheme(theme: Theme) {
  document.documentElement.classList.remove("latte", "mocha");
  document.documentElement.classList.add(theme);
  document.documentElement.dataset.theme = theme;
}

export default function App() {
  const initial = readStoredTheme();
  applyTheme(initial);
  const [theme, setTheme] = createSignal<Theme>(initial);

  function toggleTheme() {
    setTheme((current) => {
      const next: Theme = current === "mocha" ? "latte" : "mocha";
      localStorage.setItem(STORAGE_KEY, next);
      applyTheme(next);
      return next;
    });
  }

  return (
    <main class="welcome">
      <div class="welcome__atmosphere" aria-hidden="true" />
      <header class="welcome__header">
        <p class="welcome__brand">Mediaar</p>
        <button
          type="button"
          class="welcome__theme-toggle"
          onClick={toggleTheme}
          aria-label={`Switch to ${theme() === "mocha" ? "Latte" : "Mocha"} theme`}
        >
          <span class="welcome__theme-label">{theme() === "mocha" ? "Mocha" : "Latte"}</span>
          <span class="welcome__theme-hint">
            {theme() === "mocha" ? "Dark" : "Light"}
          </span>
        </button>
      </header>

      <section class="welcome__hero">
        <h1 class="welcome__title">Welcome</h1>
        <p class="welcome__lede">
          Manage your media from a calm desktop workspace styled with Catppuccin.
        </p>
      </section>
    </main>
  );
}
