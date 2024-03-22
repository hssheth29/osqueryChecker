use std::process::Command;

fn main() {
    let osquery_installed = check_osquery_installed();

    println!("osquery is {}available.", if osquery_installed { "" } else { "not " });
}

fn check_osquery_installed() -> bool {
    let output = Command::new("which")
        .arg("osqueryi")
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}