/// Integration test
/// Build the foo package. Deploy the foo package.

#[cfg(test)]
mod helpers_test;

mod tests {
    use std::{
        fs,
        io::{self, Write},
        path::PathBuf,
    };

    use crate::helpers_test;

    #[test]
    fn build_and_deploy_foo_package() {
        let mut cmd = helpers_test::get_remiz_command();
        cmd.args([
            "-v",
            "build",
            "tests/test_build_package/foo_package/foo.toml",
            "-g",
            "tests/configuration.toml",
        ]);

        // Execute the command
        let output = cmd
            // .current_dir(&package_config.path.parent().unwrap())
            .output()
            .expect("Failed to run remiz build.");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        // Assert that command did not failed
        let return_code = output.status.code().unwrap();
        assert_eq!(return_code, 0);

        // Test that the .pack file has been successfully built
        let path_to_pack = PathBuf::from("tests/packages/my_project/my_project_v1.2.3.pack")
            .canonicalize()
            .unwrap();

        // Deploy packages
        deploy_foo_package(&path_to_pack);

        // Remove .pack file
        fs::remove_file(&path_to_pack).unwrap();
    }

    fn deploy_foo_package(path_to_pack: &PathBuf) {
        let mut cmd = helpers_test::get_remiz_command();
        cmd.args([
            "-v",
            "deploy",
            &path_to_pack.display().to_string(),
            "-g",
            "tests/configuration.toml",
            "--env",
            "staging",
        ]);

        // Execute the command
        let output = cmd
            // .current_dir(&package_config.path.parent().unwrap())
            .output()
            .expect("Failed to run remiz deploy.");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        // Assert that command did not failed
        let return_code = output.status.code().unwrap();
        assert_eq!(return_code, 0);

        // Test that the .pack file has been successfully deployed
    }
}
