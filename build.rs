use std::process::Command;

fn main() {
    // Get git commit SHA
    let git_sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string())
        .trim()
        .to_string();

    // Get build timestamp in ISO 8601 format
    let build_time = chrono::Utc::now().to_rfc3339();

    // Get build host
    let build_host = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    // Export as environment variables for compile time
    println!("cargo:rustc-env=BUILD_GIT_SHA={git_sha}");
    println!("cargo:rustc-env=BUILD_TIMESTAMP={build_time}");
    println!("cargo:rustc-env=BUILD_HOST={build_host}");

    // Rerun if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
