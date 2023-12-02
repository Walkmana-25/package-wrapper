use thiserror::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use std::fs;

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
                runcmd(vec!["touch /tmp/rust-test".to_string()]);

        let test_result =  fs::metadata("/tmp/rust-test").is_ok();
        
        fs::remove_file("/tmp/rust-test").ok();

        assert!(test_result);
        assert!(run_result.is_ok());


    }


}

pub fn check_root() -> bool {
    let username = whoami::username();
    
    username == "root"
}


#[derive(Error, Debug)]
pub enum PWError {
    #[error("Supported Package Manager is not found.")]
    PackageManagerError
}



