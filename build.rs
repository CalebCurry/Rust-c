fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=linked-list");
}
