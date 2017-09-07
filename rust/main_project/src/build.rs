// provides linking path (-L)
fn main() {
    println!("cargo:rustc-link-search=../staticlib");
}