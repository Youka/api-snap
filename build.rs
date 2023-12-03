// Reference: https://doc.rust-lang.org/cargo/reference/build-scripts.html

use std::{
    env::var as env_var,
    path::PathBuf,
    process::Command
};

fn main() {
    // Build paths
    let manifest_dir = env_var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory should always be predefined!");
    let third_party_dir = PathBuf::from(manifest_dir).join("third-party");
    let download_file = third_party_dir.join("download.sh");

    // Download third-party files
    let command_status = Command::new("sh")
        .current_dir(third_party_dir)
        .arg(download_file.file_name().expect("PathBuf should contain the filename!"))
        .status()
        .expect("Command failed. Linux tools needed!");
    if !command_status.success() {
        panic!("Command failed with status: {}", command_status.code().expect("Command has no exit code?!"));
    }

    // Configure cargo
    println!("cargo:rerun-if-changed={}", download_file.to_string_lossy());
}
