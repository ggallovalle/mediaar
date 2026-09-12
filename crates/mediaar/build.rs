use std::path::PathBuf;
use std::process::Command;

fn main() {
    ensure_frontend_dist();
    tauri_build::build()
}

fn ensure_frontend_dist() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let ui_dir = manifest_dir.join("ui");
    let index = ui_dir.join("dist").join("index.html");

    println!("cargo:rerun-if-changed={}", ui_dir.join("src").display());
    println!("cargo:rerun-if-changed={}", ui_dir.join("index.html").display());
    println!("cargo:rerun-if-changed={}", ui_dir.join("package.json").display());
    println!("cargo:rerun-if-changed={}", index.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("locales").display()
    );
    // Also watch individual locale files for incremental builds.
    for locale in ["en", "es"] {
        for resource in ["common", "desktop", "tui"] {
            println!(
                "cargo:rerun-if-changed={}",
                manifest_dir
                    .join("locales")
                    .join(locale)
                    .join(format!("{resource}.ftl"))
                    .display()
            );
        }
    }

    if index.is_file() || std::env::var_os("MEDIAAR_SKIP_UI_BUILD").is_some() {
        return;
    }

    let pnpm = which("pnpm").unwrap_or_else(|| PathBuf::from("pnpm"));
    let status = Command::new(&pnpm)
        .args(["install", "--frozen-lockfile"])
        .current_dir(&ui_dir)
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            // Retry without frozen lockfile for fresh checkouts.
            let retry = Command::new(&pnpm)
                .arg("install")
                .current_dir(&ui_dir)
                .status()
                .unwrap_or(status);
            if !retry.success() {
                panic!(
                    "frontend dependencies missing and `pnpm install` failed in {}\n\
                     build the UI first (`pnpm --dir crates/mediaar/ui build`) \
                     or set MEDIAAR_SKIP_UI_BUILD=1 when dist/ is already present",
                    ui_dir.display()
                );
            }
        }
        Err(err) => {
            panic!(
                "frontend dist missing at {} and could not run pnpm: {err}\n\
                 install pnpm/node, or vendor ui/dist before building",
                index.display()
            );
        }
    }

    let build = Command::new(&pnpm)
        .arg("build")
        .current_dir(&ui_dir)
        .status()
        .expect("failed to spawn pnpm build");

    assert!(
        build.success() && index.is_file(),
        "frontend build failed; expected {}",
        index.display()
    );
}

fn which(bin: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).find_map(|dir| {
            let path = dir.join(bin);
            path.is_file().then_some(path)
        })
    })
}
