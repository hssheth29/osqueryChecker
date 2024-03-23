use std::process::Command;

fn main() {
    if !check_osquery_installed() {
        println!("osquery is not installed. Installing...");
        install_osquery();
    } else {
        println!("osquery is already installed.");
    }
}

fn check_osquery_installed() -> bool {
    // Check osqueryi
    let osqueryi_output = Command::new("which")
        .arg("osqueryi")
        .output()
        .expect("Failed to execute 'which' for osqueryi");

    let osqueryi_installed = osqueryi_output.status.success() &&
        String::from_utf8_lossy(&osqueryi_output.stdout).trim().ends_with("osqueryi");

    // Check osqueryctl
    let osqueryctl_output = Command::new("which")
        .arg("osqueryctl")
        .output()
        .expect("Failed to execute 'which' for osqueryctl");

    let osqueryctl_installed = osqueryctl_output.status.success() &&
        String::from_utf8_lossy(&osqueryctl_output.stdout).trim().ends_with("osqueryctl");

    osqueryi_installed && osqueryctl_installed
}

fn install_osquery() {
    let version = "5.11.0";
    let architecture = if cfg!(target_arch = "x86_64") { "x86_64" } else if cfg!(target_arch = "aarch64") { "aarch64" } else { panic!("Unsupported architecture"); };
    let install_url = format!("https://github.com/osquery/osquery/releases/download/{}/osquery-5.11.0-1.linux.{}.rpm", version, architecture);
    let package_path = "/tmp/osquery.rpm";

    // Download the package
    let status = Command::new("curl")
        .args(&["-L", &install_url, "-o", package_path])
        .status()
        .expect("Failed to download osquery package");

    if !status.success() {
        panic!("Failed to download the osquery package.");
    }

    // Install the package
    let status = Command::new("sudo")
        .args(&["dnf", "install", "-y", package_path])
        .status()
        .expect("Failed to install osquery package");

    if !status.success() {
        println!("Note: osquery may already be installed or another error occurred.");
    }

    // Clean up downloaded package
    let _ = std::fs::remove_file(package_path);
}
