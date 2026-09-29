//! Embeds every model under src/models/props and src/models/people into the
//! executable (the game is a single .exe), generating a table of
//! (virtual path, bytes) that the asset loader registers at startup.

use std::fmt::Write;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/models");
    let mut out = String::from("pub static MODEL_FILES: &[(&str, &[u8])] = &[\n");
    for sub in ["props", "people"] {
        let dir = root.join(sub);
        println!("cargo:rerun-if-changed={}", dir.display());
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        let mut files: Vec<_> = rd.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_file()).collect();
        files.sort();
        for f in files {
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            if !(name.ends_with(".glb") || name.ends_with(".json")) {
                continue;
            }
            println!("cargo:rerun-if-changed={}", f.display());
            writeln!(out, "    (\"{}/{}\", include_bytes!(r\"{}\")),", sub, name, f.display()).unwrap();
        }
    }
    out.push_str("];\n");
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("embedded_models.rs");
    std::fs::write(dest, out).unwrap();
}
