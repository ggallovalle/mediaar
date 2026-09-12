# Mediaar

Manage your media from one binary: desktop app (`mediaar desktop`) and TUI (`mediaar tui`).

## Install

User-level install (no root). Places the binary, man page, zsh completion, and a desktop launcher entry under `~/.local`:

```sh
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh
```

Preview without writing files:

```sh
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh -s -- --dry-run
```

Pin a version or override the install location:

```sh
MEDIAAR_VERSION=0.1.0 sh <(curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh)
# or
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | \
  MEDIAAR_INSTALL_PATH="$HOME/.local/bin/mediaar" sh
```

After install:

- Ensure `~/.local/bin` is on your `PATH`
- For zsh completions, add once: `fpath=($HOME/.local/share/zsh/site-functions $fpath)` then `compinit`
- Open **Mediaar** from your app launcher, or run `mediaar desktop` / `mediaar tui`

### Other install methods

```sh
cargo binstall mediaar
# or
cargo install mediaar --locked
```

These install the binary only (no man page, zsh completion, or desktop entry). Prefer the curl script for a full user setup.

## Uninstall

Removes the user-level install created by `install.sh`:

```sh
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/uninstall.sh | sh
```

Preview:

```sh
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/uninstall.sh | sh -s -- --dry-run
```

## What gets installed

| Artifact | Path |
| --- | --- |
| Binary | `~/.local/bin/mediaar` |
| Man page | `~/.local/share/man/man1/mediaar.1` |
| Zsh completion | `~/.local/share/zsh/site-functions/_mediaar` |
| Desktop entry | `~/.local/share/applications/mediaar.desktop` |
| Icons | `~/.local/share/icons/...` |

## License

MIT
