use std::env;

fn main(){
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(r"cargo:rustc-link-search=native={}/../sampl_dll/x64/Debug",dir);
    println!("cargo:rustc-link-lib=dylib={}","sampl_dll");
}