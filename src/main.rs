use std::process::Command;
use regex::Regex;

fn main() {
    let osquery_installed = check_osquery_installed();
    println!("osquery is {}available.", if osquery_installed { "" } else { "not " });
}

fn check_osquery_installed() -> bool {
    let output = Command::new("which")
        .arg("osqueryi")
        .output()
        .expect("Failed to execute 'which'");

    if !output.status.success() {
        return false;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"/\w+").unwrap(); // Simple regex to check for a path
    re.is_match(&output_str)
}



