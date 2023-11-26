
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_gen_cmd() {

        let cmd_arch : Result<Ok<Vec<String>>, Err<String> = gen_cmd(
            package_manager = "pacman",
            packages = vec!["package1", "package2"],
            yes_all = Option(None),
        );

        assert_eq!(cmd_arch, Result.Ok(vec!["pacman", "-S", "package1", "package2"]));





    }
}
