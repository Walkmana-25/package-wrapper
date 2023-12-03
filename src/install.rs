use std::collections::HashMap;

use anyhow::Error;

use crate::libs::PWError;



#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_gen_cmd() {

        let gen_cmd = gen_cmd(
            &"pacman".to_string(),
            &vec!["package1".to_string(), "package2".to_string()],
            false,
            ModeSelect::Install
        );
        let cmd_arch : Result<Vec<String>, Error> = gen_cmd;

        assert_eq!(cmd_arch.ok(), std::option::Option::Some(vec!["pacman".to_string(), "-S".to_string(), "package1".to_string(), "package2".to_string()]));
    }
}

struct Cmd {
    command: Vec<String>,
    yes_all: String,
}

pub enum ModeSelect {
    Install,
    Remove
}

pub fn gen_cmd<'a>(
    package_manager: &'a String,
    packages: &'a Vec<String>,
    yes_all: bool,
    mode: ModeSelect
    ) -> Result<Vec<String>, Error > {

    let mut cmd = HashMap::new();

    match mode {
        ModeSelect::Install => {
            cmd.insert("pacman".to_string(), Cmd {
                command: vec!["pacman".to_string(), "-S".to_string()],
                yes_all: "--noconfirm".to_string(),
            });
    
            cmd.insert("apt".to_string(), Cmd {
                command: vec!["apt-get".to_string(), "install".to_string()],
                yes_all: "-y".to_string(),
            });
        },
        ModeSelect::Remove => {
            cmd.insert("pacman".to_string(), Cmd {
                command: vec!["pacman".to_string(), "-R".to_string()],
                yes_all: "--noconfirm".to_string(),
            });
    
            cmd.insert("apt".to_string(), Cmd {
                command: vec!["apt-get".to_string(), "remove".to_string()],
                yes_all: "-y".to_string(),
            });
        }
    
    }

    let package_manager_list = ["apt".to_string(), "pacman".to_string()];

    #[allow(unused)]
    match package_manager_list.binary_search(package_manager) {
        Ok(i) => {
            let space = " ".to_string();
            let current_cmd = cmd.get(package_manager).unwrap();
            let confirm_cmd = if yes_all {
                &current_cmd.yes_all
            } else {
                &space
            };
            let mut command : Vec<String> = current_cmd.command.clone();

            if yes_all {
                command.insert(1, current_cmd.yes_all.clone());
            }

            command.append(&mut packages.clone());

            Ok(command)




        },
        Err(e) => {
            Err(PWError::PackageManagerError.into())

        }

    }

    }







