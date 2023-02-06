// use assert_cmd::prelude::*; // Add methods on commands
// use predicates::prelude::*;
use sudo_cli::SudoOptions;

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;

    /// --preserve-env
    /// Passing '-E' sets 'short_preserve_env' to true, 'preserve_env_list' stays empty
    #[test]
    fn short_preserve_env() {
        let cmd = SudoOptions::parse_from_args(
            vec![String::from("sudo"), String::from("-E")].into_iter(),
        );
        assert_eq!(cmd.preserve_env, true);
        assert_eq!(cmd.preserve_env_list, Vec::<String>::new()); //empty vec, .stdout(predicate::str::contains("preserve_env_list: []"));
    }

    /// Passing '--preserve-env' sets 'short_preserve_env' to true, 'preserve_env_list' stays empty
    #[test]
    fn preserve_env_witout_var() {
        let cmd = SudoOptions::parse_from_args(
            vec![String::from("sudo"), String::from("--preserve-env")].into_iter(),
        );
        assert_eq!(cmd.preserve_env, true);
        assert_eq!(cmd.preserve_env_list, Vec::<String>::new());
    }

    /// Passing '-E' with a variable fails
    #[test]
    #[should_panic]
    fn short_preserve_env_with_var_fails() {
        SudoOptions::parse_from_args(
            vec![String::from("sudo"), String::from("-E=variable")].into_iter(),
        );

        // let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        // cmd.arg("-E=variable");
        // cmd.assert().failure().stderr(predicate::str::contains(
        //     "error: unexpected argument \'-=\' foun",
        // ));
    }

    /// Passing '--preserve-env' with an argument fills 'preserve_env_list', 'short_preserve_env' stays 'false'
    #[test]
    fn preserve_env_with_var() {
        let cmd = SudoOptions::parse_from_args(
            vec![
                String::from("sudo"),
                String::from("--preserve-env=some_argument"),
            ]
            .into_iter(),
        );
        assert_eq!(cmd.preserve_env_list, vec!["some_argument"]);
        assert_eq!(cmd.preserve_env, false);
    }

    /// Passing '--preserve-env' with several arguments fills 'preserve_env_list', 'short_preserve_env' stays 'false'
    #[test]
    fn preserve_env_with_several_vars() {
        let cmd = SudoOptions::parse_from_args(
            vec![
                String::from("sudo"),
                String::from("--preserve-env=some_argument,another_argument,a_third_one"),
            ]
            .into_iter(),
        );
        assert_eq!(
            cmd.preserve_env_list,
            vec!["some_argument", "another_argument", "a_third_one"]
        );
        assert_eq!(cmd.preserve_env, false);
    }

    /// Catch env variable that is given without hyphens in 'VAR=value' form in env_var_list.
    /// external_args stay empty.
    #[test]
    fn env_variable() {
        let cmd = SudoOptions::parse_from_args(
            vec![String::from("sudo"), String::from("ENV=with_a_value")].into_iter(),
        );
        assert_eq!(
            cmd.env_var_list,
            vec![("ENV".to_owned(), "with_a_value".to_owned())]
        );
        assert_eq!(cmd.external_args, Vec::<String>::new());
    }

    /// Catch several env variablse that are given without hyphens in 'VAR=value' form in env_var_list.
    /// external_args stay empty.
    #[test]
    fn several_env_variables() {
        let cmd = SudoOptions::parse_from_args(
            vec![
                String::from("sudo"),
                String::from("ENV=with_a_value"),
                String::from("another_var=otherval"),
                String::from("more=this_is_a_val"),
            ]
            .into_iter(),
        );
        assert_eq!(
            cmd.env_var_list,
            vec![
                ("ENV".to_owned(), "with_a_value".to_owned()),
                ("another_var".to_owned(), "otherval".to_owned()),
                ("more".to_owned(), "this_is_a_val".to_owned())
            ]
        );
        assert_eq!(cmd.external_args, Vec::<String>::new());
    }

    /// Mix env variables and trailing arguments that just pass through sudo
    /// Divided by hyphens.
    #[test]
    fn mix_env_variables_with_trailing_args_divided_by_hyphens() {
        let cmd = SudoOptions::parse_from_args(
            vec![
                String::from("sudo"),
                String::from("env=var"),
                String::from("--"),
                String::from("external=args"),
                String::from("something"),
            ]
            .into_iter(),
        );
        assert_eq!(cmd.env_var_list, vec![("env".to_owned(), "var".to_owned())]);
        assert_eq!(cmd.external_args, vec!["external=args", "something"]);
    }

    /// Mix env variables and trailing arguments that just pass through sudo
    /// Divided by known flag.
    // Currently panics.
    #[test]
    fn mix_env_variables_with_trailing_args_divided_by_known_flag() {
        let cmd = SudoOptions::parse_from_args(
            vec![
                String::from("sudo"),
                String::from("-b"),
                String::from("external=args"),
                String::from("something"),
            ]
            .into_iter(),
        );
        println!("cmd: {:?}", cmd);
        assert_eq!(
            cmd.env_var_list,
            vec![("external".to_owned(), "args".to_owned())]
        );
        assert_eq!(cmd.external_args, vec!["something"]);
        assert_eq!(cmd.background, true);
    }

    /// Catch trailing arguments that just pass through sudo
    /// but look like a known flag.
    #[test]
    fn trailing_args_followed_by_known_flag() {
        let cmd = SudoOptions::parse_from_args(
            vec![
                String::from("sudo"),
                String::from("args"),
                String::from("followed_by"),
                String::from("known_flag"),
                String::from("-b"),
            ]
            .into_iter(),
        );
        assert_eq!(cmd.background, false);
        assert_eq!(
            cmd.external_args,
            vec!["args", "followed_by", "known_flag", "-b"]
        );
    }

    /// Catch trailing arguments that just pass through sudo
    /// but look like a known flag, divided by hyphens.
    #[test]
    fn trailing_args_hyphens_known_flag() {
        let cmd = SudoOptions::parse_from_args(
            vec![
                String::from("sudo"),
                String::from("--"),
                String::from("trailing"),
                String::from("args"),
                String::from("followed_by"),
                String::from("known_flag"),
                String::from("-b"),
            ]
            .into_iter(),
        );
        assert_eq!(cmd.background, false);
        assert_eq!(
            cmd.external_args,
            vec!["trailing", "args", "followed_by", "known_flag", "-b"]
        );
    }

    /// Flags that exclude each other
    #[test]
    #[should_panic]
    fn remove_and_reset_timestamp_exclusion() {
        // let cmd = SudoOptions::parse_from_args(vec![String::from("sudo"), String::from("-k"), String::from("-K")].into_iter());
        SudoOptions::parse_from_args(
            vec!["sudo", "--reset-timestamp", "--reboot-timestamp"]
                .into_iter()
                .map(String::from),
        );
        // let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        // cmd.arg("-k").arg("-K");
        // cmd.assert().failure().stderr(predicate::str::contains(
        //     "the argument '--reset-timestamp' cannot be used with '--remove-timestamp'",
        // ));
    }
}
