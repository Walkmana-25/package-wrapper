use thiserror::Error;
use std::process::{self, Command};

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_check_root() {
        use std::process::{Output, Command};

        let username: Output = Command::new("whoami")
            .output()
            .unwrap();

        let username : String = String::from_utf8(username.stdout).unwrap();

        match username == "root" {
            true => assert!(check_root()),
            false => assert!(!check_root()),
            
        }
    }

    #[test]
    fn test_runcmd(){
        let run_result: Result<(), anyhow::Error> =
                run_cmd(vec!["/bin/touch".to_string(), "/tmp/rust-test".to_string()]);
        println!("run_result:{:?}", run_result);

        let test_result =  Path::new("/tmp/rust-test").exists();
        println!("test_result:{:?}", test_result);
        fs::remove_file("/tmp/rust-test").ok();

        assert!(test_result);
        assert!(run_result.is_ok());


    }
    #[test]
    fn test_rootcmd(){

        let run_result: Result<(), anyhow::Error> =
                run_cmd_root(vec![format!("test {} = `whoami`", whoami::username())]);

        if "root" == whoami::username() {
            assert!(run_result.is_ok());
        } else {
            assert!(run_result.is_err());
        }

    }


}

pub fn check_root() -> bool {
    let username = whoami::username();
    
    username == "root"
}

pub fn run_cmd(cmd: Vec<String>) -> Result<(), anyhow::Error> {

    let mut command = process::Command::new("/bin/bash")
        .arg("-c")
        .arg(cmd.join(" "))
        .stdin(process::Stdio::inherit())
        .spawn()?;



    command.wait()?;

    Ok(())
}

pub fn run_cmd_root(cmd: Vec<String>) -> Result<(), anyhow::Error> {

    if !check_root() {
        return Err(PWError::NotRootUserError.into());
    }


    run_cmd(cmd)?;

    Ok(())
}

/// check which package manager run in this system.
pub fn select_package_manager() -> Result<String, anyhow::Error>{

    let package_list = [
        "pacman", "apt"
    ];

    for p in package_list {
        let output = Command::new("/usr/bin/which").arg(p).output().unwrap();

        if String::from_utf8(output.stdout).unwrap() != "" {
            return Ok(p.to_string());
        }


    }

    Err(PWError::PackageManagerError.into())

}




#[derive(Error, Debug)]
pub enum PWError {
    #[error("Supported Package Manager is not found.")]
    PackageManagerError,
    #[error("you cannot perform this operation unless you are root.")]
    NotRootUserError
}



