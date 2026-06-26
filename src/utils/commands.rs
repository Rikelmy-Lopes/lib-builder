use crate::config::constants::{CREATE_NO_WINDOW_FLAG, FILE_TO_DELETE};
use std::{
    os::windows::process::CommandExt,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

pub fn spawn_ant_build(ant_path: &PathBuf, source_project_path: &str) -> Child {
    Command::new(ant_path)
        .arg("-q")
        .arg("-f")
        .arg(source_project_path)
        .arg("clean")
        .arg("jar")
        /* .arg("-version") */
        .creation_flags(CREATE_NO_WINDOW_FLAG)
        .env_remove("ANT_HOME")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Falha ao executar o Apache Ant. Verifique se o caminho está correto.")
}

pub fn spawn_7zip(seven_zip_path: &PathBuf, build_file_path: &str) -> Child {
    Command::new(seven_zip_path)
        .arg("d")
        .arg(build_file_path)
        .arg(FILE_TO_DELETE)
        /* .arg("-version") */
        .creation_flags(CREATE_NO_WINDOW_FLAG)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Falha ao executar o 7zip.")
}

pub fn kill_process(pid: &u32) -> bool {
    let result = Command::new("taskkill")
        .args(["/f", "/pid", &pid.to_string(), "/t"])
        .creation_flags(CREATE_NO_WINDOW_FLAG)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                println!("Success killing process with pid: {}", pid);
                true
            } else {
                println!("Failed killing process with pid: {}", pid);
                //println!("{}", format_output(output));
                false
            }
        }
        Err(e) => {
            println!("Error killing process with pid: {}", pid);
            println!("{}", e);
            false
        }
    }
}
