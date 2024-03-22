use std::process::Command;

fn main() {
    let osquery_installed = check_osquery_installed();


    println!("osquery is {} available.", if osquery_installed { "" } else { "not " });
}

fn check_osquery_installed() -> bool {
    let output = Command::new("osqueryi")
        .arg("--version")
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute 'osqueryi' version"));

    output.status.success()
}

