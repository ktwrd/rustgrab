use std::path::PathBuf;
use std::env;

pub fn main() {
    build_fltk();
}

pub fn build_fltk() {
    println!("cargo:rerun-if-changed=src/gui/config_ui.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/gui/config_ui.fl", out_path.join("config_ui.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
}