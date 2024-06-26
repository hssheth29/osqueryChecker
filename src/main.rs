use std::process::{Command, exit, Stdio};
use std::fs;

fn main() {
    // Check for root privileges at the start of the program
    if !is_root() {
        eprintln!("This program requires sudo privileges. Please run it as root.");
        exit(1);
    }

    match check_osquery_installed() {
        Ok(installed) => {
            if installed {
                // Silent if already installed, or change to log level if desired
                println!("osquery is already installed.");
            } else {
                println!("osquery is not installed. Installing..."); // Make silent
                match install_osquery() {
                    Ok(_) => {
                        // Silent success, or log success if desired
                        println!("osquery installed successfully.");
                    },
                    Err(e) => eprintln!("Failed to install osquery: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error checking osquery installation: {}", e),
    }
}

fn is_root() -> bool {
    if let Ok(output) = Command::new("id")
        .arg("-u")
        .output() {
        if let Ok(uid) = String::from_utf8(output.stdout) {
            if let Ok(uid) = uid.trim().parse::<u32>() {
                return uid == 0;
            }
        }
    }
    false
}

fn check_osquery_installed() -> Result<bool, String> {
    let osqueryi_result = Command::new("which")
        .arg("osqueryi")
        .stdout(Stdio::null()) // Suppress output
        .output();
    let osqueryctl_result = Command::new("which")
        .arg("osqueryctl")
        .stdout(Stdio::null()) // Suppress output
        .output();

    if osqueryi_result.is_err() || osqueryctl_result.is_err() {
        return Err("Failed to execute 'which' command.".to_string());
    }

    let osqueryi_installed = osqueryi_result.unwrap().status.success();
    let osqueryctl_installed = osqueryctl_result.unwrap().status.success();

    Ok(osqueryi_installed && osqueryctl_installed)
}

fn install_osquery() -> Result<(), String> {
    let version = "5.11.0";
    let architecture = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        return Err("Unsupported architecture".to_string());
    };

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
            version,
            version,
            architecture.replace("x86_64", "amd64").replace("aarch64", "arm64")
        ),
        "rpm" => format!(
            "https://github.com/osquery/osquery/releases/download/{}/osquery-{}-1.linux.{}.rpm",
            version,
            version,
            architecture
        ),
        _ => unreachable!(),
    };

    let package_path = format!("/tmp/osquery.{}", package_type);

    let downloader = if Command::new("wget")
        .stdout(Stdio::null()) // Check silently
        .output().is_ok() {
        "wget"
    } else if Command::new("curl")
        .stdout(Stdio::null()) // Check silently
        .output().is_ok() {
        "curl"
    } else {
        return Err("Neither wget nor curl is installed.".to_string());
    };

    let download_result = if downloader == "wget" {
        Command::new("wget")
            .args(&["-O", &package_path, &install_url])
            .stdout(Stdio::null()) // Suppress output
            .status()
    } else {
        Command::new("curl")
            .args(&["-L", &install_url, "-o", &package_path])
            .stdout(Stdio::null()) // Suppress output
            .status()
    };

    if download_result.is_err() || !download_result.unwrap().success() {
        return Err("Failed to download the osquery package.".to_string());
    }

    let install_status = Command::new(package_manager)
        .arg("install")
        .arg("-y")
        .arg(&package_path)
        .stdout(Stdio::null()) // Suppress output during installation
        .status();
    if install_status.is_err() || !install_status.unwrap().success() {
        return Err("Failed to install osquery package.".to_string());
    }

    // Attempt to clean up the downloaded package regardless of installation success
    let _ = fs::remove_file(&package_path);

    Ok(())
}
