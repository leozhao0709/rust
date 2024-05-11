use std::{env, process::Command};

#[tokio::main]
async fn main() {
    let exe_path = env::current_exe().expect("Failed to get the executable path");

    // Get the directory containing the executable
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get the directory of the executable"); // Debug or Release folder

    let jar_path = exe_dir.join("../../resources/pdf-highlighter-extractor-0.0.1-SNAPSHOT.jar");
    // get it from debug folder to root resource folder
    let jar_path = jar_path.to_str().expect("invalid jar path");

    // blocking_way(jar_path);

    let output = tokio::process::Command::new("java")
        .args(["-jar", jar_path]) // Ensure you can capture stdout
        .spawn()
        .expect("Failed to spawn child process")
        .wait_with_output()
        .await
        .expect("Failed to wait on child");

    if output.status.success() {
        println!("{:?}", String::from_utf8(output.stdout));
    } else {
        println!("{:?}", String::from_utf8(output.stderr));
    }
}

fn blocking_way(jar_path: &str) {
    let result = Command::new("java")
        .args(["-jar", jar_path])
        .output()
        .map_err(|err| err.to_string())
        .and_then(|output| {
            if output.status.success() {
                // String::from_utf8(output.stdout).expect("failed to running")
                String::from_utf8(output.stdout).map_err(|err| err.to_string())
            } else {
                String::from_utf8(output.stderr).map_err(|err| err.to_string())
            }
        });
    println!("{:#?}", result);
}
