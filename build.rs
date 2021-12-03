use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=ri_package_1.1/ri");
    std::env::set_current_dir("ri_package_1.1/ri/src")?;

    let mut cpp_files = vec![];
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
            if filename.ends_with(".cpp") {
                cpp_files.push(filename.to_owned());
            }
        }
    }

    cc::Build::new()
        .cpp(true)
        .flag("-w") // Disable warnings
        .includes(["../include", "../include/VG", "../include/EGL"])
        .flag("-fpermissive")
        .flag("-DEGLAPI=")
        .files(cpp_files)
        .compile("EGL");

    /*println!("cargo:warning={:?}", Command::new("/usr/bin/env")
        .arg("/bin/sh")
        .arg("-c")
        .arg("$CXX -w -I ../include -I ../include/EGL -I ../include/VG *.cpp -shared -o libEGL.so -fpermissive -DEGLAPI= -fPIC")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?);

    let static_lib_filename = "libEGL.so";
    let out_dir_path = PathBuf::from(env::var("OUT_DIR")?);
    fs::copy(static_lib_filename, out_dir_path.join(static_lib_filename))?;
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir_path.to_str().unwrap()
    );*/

    Ok(())
}
