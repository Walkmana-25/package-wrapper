use thiserror::Error;

#[cfg(test)]
mod tests {
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



