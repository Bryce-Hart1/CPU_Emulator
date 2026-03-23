src/main.rs           ← handles native vs web entry points, don't touch
src/lib.rs            ← wires main.rs to app.rs for wasm, don't touch
.github/workflows/    ← CI/CD for packaging and GitHub Pages, keep it
assets/sw.js          ← service worker for web builds, keep it
index.html            ← web shell, keep it (just rename eframe_template → your crate name)
Cargo.toml            ← keep structure, just edit the metadata fields
rust-toolchain.toml   ← pins your Rust version, keep it
Trunk.toml            ← web build config, keep it
