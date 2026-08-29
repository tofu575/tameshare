use std::path::Path;

/// keg-onlyのlibpqをRust linkerから参照可能にする。
fn main() {
    println!("cargo:rerun-if-env-changed=PQ_LIB_DIR");

    if let Ok(path) = std::env::var("PQ_LIB_DIR") {
        println!("cargo:rustc-link-search=native={path}");
        return;
    }

    // Homebrew keeps keg-only libpq outside the default linker search path.
    // Linux package managers normally install it in a standard path.
    for path in ["/opt/homebrew/opt/libpq/lib", "/usr/local/opt/libpq/lib"] {
        if Path::new(path).is_dir() {
            println!("cargo:rustc-link-search=native={path}");
            break;
        }
    }
}
