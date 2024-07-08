use std::env;

fn main(){
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let deb_rel = env::var("PROFILE").unwrap();    
    println!(r"cargo:rustc-link-search=native={}/target/{}",dir,deb_rel);
    println!("cargo:rustc-link-lib=dylib={}","sampl_dll");
}