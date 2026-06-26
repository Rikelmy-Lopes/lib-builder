use std::process::Output;

pub fn format_output(output: Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

pub fn is_build_successful(output: &str) -> bool {
    output.contains("BUILD SUCCESSFUL")
}

pub fn is_7zip_successful(output: &str) -> bool {
    output.contains("Delete data from archive: 1 file")
}
