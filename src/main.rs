use std::process::Command;
use std::fs;

fn main() {
    match check_osquery_installed() {
        Ok(installed) => {
            if installed {
                println!("osquery is already installed.");
            } else {
                println!("osquery is not installed. Installing...");
                match install_osquery() {
                    Ok(_) => println!("osquery installed successfully."),
                    Err(e) => println!("Failed to install osquery: {}", e),
                }
            }
        }
        Err(e) => println!("Error checking osquery installation: {}", e),
    }
}

fn check_osquery_installed() -> Result<bool, String> {
    let osqueryi_result = Command::new("which").arg("osqueryi").output();
    let osqueryctl_result = Command::new("which").arg("osqueryctl").output();

    if osqueryi_result.is_err() || osqueryctl_result.is_err() {
        return Err("Failed to execute 'which' command.".to_string());
    }

    let osqueryi_installed = osqueryi_result.unwrap().status.success();
    let osqueryctl_installed = osqueryctl_result.unwrap().status.success();

    Ok(osqueryi_installed && osqueryctl_installed)
}

fn install_osquery() -> Result<(), String> {
    let version = "5.11.0";
    let architecture = if cfg!(target_arch = "x86_64") { "x86_64" } else if cfg!(target_arch = "aarch64") { "aarch64" } else { return Err("Unsupported architecture".to_string()); };

    // Determine Linux distribution (Debian-based or Fedora-based)
    let (package_type, package_manager) = if cfg!(target_os = "linux") {
        match fs::metadata("/etc/debian_version") {
            Ok(_) => ("deb", "apt-get"),
            Err(_) => ("rpm", "dnf"),
        }
    } else {
        return Err("Unsupported operating system".to_string());
    };

    let install_url = match package_type {
        "deb" => format!(
            "https://github.com/osquery/osquery/releases/download/{}/osquery_{}-1.linux_{}.deb",
            version, version, architecture.replace("x86_64", "amd64").replace("aarch64", "arm64")
        ),
        "rpm" => format!(
            "https://github.com/osquery/osquery/releases/download/{}/osquery-{}-1.linux.{}.rpm",
            version, version, architecture
        ),
        _ => unreachable!(),
    };

    let package_path = format!("/tmp/osquery.{}", package_type);

    // Check for wget or curl
    let downloader = if Command::new("wget").output().is_ok() {
        "wget"
    } else if Command::new("curl").output().is_ok() {
        "curl"
    } else {
        return Err("Neither wget nor curl is installed.".to_string());
    };

    let download_result = if downloader == "wget" {
        Command::new("wget").args(&["-O", &package_path, &install_url]).status()
    } else {
        Command::new("curl").args(&["-L", &install_url, "-o", &package_path]).status()
    };

    if download_result.is_err() || !download_result.unwrap().success() {
        return Err("Failed to download the osquery package.".to_string());
    }

    let install_status = Command::new("sudo").args(&[package_manager, "install", "-y", &package_path]).status();
    if install_status.is_err() || !install_status.unwrap().success() {
        return Err("Failed to install osquery package.".to_string());
    }

    // Attempt to clean up the downloaded package regardless of installation success
    let _ = fs::remove_file(&package_path);

    Ok(())
}
