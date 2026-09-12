### Shared Mediaar strings (English)
### Loaded by both desktop and TUI bundles.

## Terms (reusable fragments)
-brand-name = Mediaar
-theme-latte = Latte
-theme-mocha = Mocha

## Brand
app-brand = { -brand-name }

## Theme toggle — selectors + attributes
theme-toggle =
    .label =
        { $theme ->
            [latte] { -theme-latte }
           *[mocha] { -theme-mocha }
        }
    .hint =
        { $theme ->
            [latte] Light
           *[mocha] Dark
        }
    .aria-label =
        Switch to { $next ->
            [latte] { -theme-latte }
           *[mocha] { -theme-mocha }
        } theme

## Language toggle — selectors + attributes
lang-toggle =
    .label =
        { $lang ->
            [en] English
            [es] Español
           *[other] { $lang }
        }
    .hint = Language
    .aria-label = Switch language

## Fluent showcase (shared demos)
showcase-heading = Fluent showcase
showcase-intro = Shared .ftl resources power both surfaces; each also loads its own.

# Placeables + terms
showcase-placeable = Hello, { $name } — welcome to { -brand-name }.

# Plural categories
showcase-plural =
    { $count ->
        [0] No titles in the library yet.
        [one] One title in the library.
       *[other] { $count } titles in the library.
    }

# NUMBER builtin
showcase-number = Free space estimate: { NUMBER($gigabytes, minimumFractionDigits: 1) } GB.

# Date placeable (formatted by the host via Intl / locale-aware APIs)
showcase-date = Session date: { $date }.
