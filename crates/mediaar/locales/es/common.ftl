### Cadenas compartidas de Mediaar (español)
### Cargadas por los bundles de escritorio y TUI.

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
            [latte] Claro
           *[mocha] Oscuro
        }
    .aria-label =
        Cambiar al tema { $next ->
            [latte] { -theme-latte }
           *[mocha] { -theme-mocha }
        }

## Language toggle — selectors + attributes
lang-toggle =
    .label =
        { $lang ->
            [en] English
            [es] Español
           *[other] { $lang }
        }
    .hint = Idioma
    .aria-label = Cambiar idioma

## Fluent showcase (shared demos)
showcase-heading = Demostración de Fluent
showcase-intro = Recursos .ftl compartidos alimentan ambas superficies; cada una carga los suyos.

# Placeables + terms
showcase-placeable = Hola, { $name } — bienvenido a { -brand-name }.

# Plural categories
showcase-plural =
    { $count ->
        [0] Aún no hay títulos en la biblioteca.
        [one] Un título en la biblioteca.
       *[other] { $count } títulos en la biblioteca.
    }

# NUMBER builtin
showcase-number = Espacio libre estimado: { NUMBER($gigabytes, minimumFractionDigits: 1) } GB.

# Date placeable (formatted by the host via Intl / locale-aware APIs)
showcase-date = Fecha de sesión: { $date }.
