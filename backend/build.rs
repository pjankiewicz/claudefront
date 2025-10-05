use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/types/");

    // Generate OpenAPI spec at build time
    // Note: This is a placeholder - the actual spec is generated at runtime
    // We'll export it via an endpoint and use it for TypeScript generation

    let output_dir = Path::new("../frontend/src/generated");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).ok();
    }

    println!("Build script executed - OpenAPI spec will be generated at runtime");
}
