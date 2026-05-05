fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("msvc") {
        println!("cargo:rustc-link-arg=/OUT:taxipatch.asi");
    } else if target.contains("windows-gnu") {
        println!("cargo:rustc-link-arg=-o");
        println!("cargo:rustc-link-arg=taxipatch.asi");
    }
}
