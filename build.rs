use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/leetcode");
    let dest_path = PathBuf::from("src/leetcode.rs");

    let mut mod_declarations = String::new();

    let mut files = fs::read_dir(src_dir)
        .expect("Error")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    files.sort();

    // Iterate over the .rs files in the directory
    for path in files {
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                mod_declarations.push_str(&format!("pub mod {};\n", file_stem));
            }
        }
    }
    fs::write(dest_path, mod_declarations).unwrap();
}
