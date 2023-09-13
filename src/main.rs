use std::fs;
use std::process::Command;
use std::string::String;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("for script in /etc/update-motd.d/*; do $script; done")
        .output()
        .expect("Failed to execute scripts");

    let motd_content: String = String::from_utf8_lossy(&output.stdout).into_owned();

    fs::write("/etc/motd", motd_content.as_bytes()).expect("Unable to write to /etc/motd");
}
