// use clap::CommandFactory;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// --preserve-env
    /// Passing '-E' sets 'short_preserve_env' to true, 'preserve_env_list' stays empty
    #[test]
    fn short_preserve_env() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("-E");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("preserve_env: true,"))
            .stdout(predicate::str::contains("preserve_env_list: []"));
    }

    /// Passing '--preserve-env' sets 'short_preserve_env' to true, 'preserve_env_list' stays empty
    #[test]
    fn preserve_env_witout_var() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("--preserve-env");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("preserve_env: true,"))
            .stdout(predicate::str::contains("preserve_env_list: []"));
    }

    /// Passing '-E' with a variable fails
    #[test]
    fn short_preserve_env_with_var_fails() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("-E=variable");
        cmd.assert().failure().stderr(predicate::str::contains(
            "error: unexpected argument \'-=\' foun",
        ));
    }

    /// Passing '--preserve-env' with an argument fills 'preserve_env_list', 'short_preserve_env' stays 'false'
    #[test]
    fn preserve_env_with_var() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("--preserve-env=some_argument");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(
                "preserve_env_list: [\"some_argument\"]",
            ))
            .stdout(predicate::str::contains("preserve_env: false,"));
    }

    /// Passing '--preserve-env' with several arguments fills 'preserve_env_list', 'short_preserve_env' stays 'false'
    #[test]
    fn preserve_env_with_several_vars() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("--preserve-env=some_argument,another_argument,a_third_one");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(
                "preserve_env_list: [\"some_argument\", \"another_argument\", \"a_third_one\"]",
            ))
            .stdout(predicate::str::contains("preserve_env: false,"));
    }

    /// Catch env variable that is given without hyphens in 'VAR=value' form in env_var_list.
    /// external_args stay empty.
    #[test]
    fn env_variable() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("ENV=with_a_value");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(
                "env_var_list: [(\"ENV\", \"with_a_value\")]",
            ))
            .stdout(predicate::str::contains("external_args: []"));
    }

    /// Catch several env variablse that are given without hyphens in 'VAR=value' form in env_var_list.
    /// external_args stay empty.
    #[test]
    fn several_env_variables() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("ENV=with_a_value")
            .arg("another_var=otherval")
            .arg("more=this_is_a_val");
        cmd
            .assert()
            .success()
            .stdout(predicate::str::contains("env_var_list: [(\"ENV\", \"with_a_value\"), (\"another_var\", \"otherval\"), (\"more\", \"this_is_a_val\")]"))
            .stdout(predicate::str::contains("external_args: []"));
    }

    /// Mix env variables and trailing arguments that just pass through sudo
    /// Divided by hyphens.
    #[test]
    fn mix_env_variables_with_trailing_args_divided_by_hyphens() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("env=var")
            .arg("--")
            .arg("external=args")
            .arg("something");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(
                "env_var_list: [(\"env\", \"var\")]",
            ))
            .stdout(predicate::str::contains(
                "external_args: [\"external=args\", \"something\"]",
            ));
    }

    /// Mix env variables and trailing arguments that just pass through sudo
    /// Divided by known flag.
    // Currently panics.
    #[test]
    #[should_panic(expected = "env_var_list: [(\"env\", \"var\"), (\"external\", \"args\")]")]
    fn mix_env_variables_with_trailing_args_divided_by_known_flag() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("env=var")
            .arg("-b")
            .arg("external=args")
            .arg("something");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(
                "env_var_list: [(\"env\", \"var\")]",
            ))
            .stdout(predicate::str::contains(
                "external_args: [\"external=args\", \"something\"]",
            ))
            .stdout(predicate::str::contains("background: true"));
    }
    /// Catch trailing arguments that just pass through sudo
    /// but look like a known flag.
    #[test]
    fn trailing_args_followed_by_known_flag() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("trailing")
            .arg("args")
            .arg("followed_by")
            .arg("known_flag")
            .arg("-b");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("background: false,"))
            .stdout(predicate::str::contains(
                "external_args: [\"trailing\", \"args\", \"followed_by\", \"known_flag\", \"-b\"]",
            ));
    }

    /// Catch trailing arguments that just pass through sudo
    /// but look like a known flag, divided by hyphens.
    #[test]
    fn trailing_args_hyphens_known_flag() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("--")
            .arg("trailing")
            .arg("args")
            .arg("followed_by")
            .arg("known_flag")
            .arg("-b");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("background: false,"))
            .stdout(predicate::str::contains(
                "external_args: [\"trailing\", \"args\", \"followed_by\", \"known_flag\", \"-b\"]",
            ));
    }

    /// Flags that exclude each other
    #[test]
    fn remove_and_reset_timestamp_exclusion() {
        let mut cmd = std::process::Command::cargo_bin("sudo").unwrap();
        cmd.arg("-k").arg("-K");
        cmd.assert().failure().stderr(predicate::str::contains(
            "the argument '--reset-timestamp' cannot be used with '--remove-timestamp'",
        ));
    }
}
