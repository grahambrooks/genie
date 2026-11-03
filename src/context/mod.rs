pub fn shell() -> String {
    let shell_env = std::env::var("SHELL").unwrap();
    let shell = shell_env.split('/').next_back().unwrap();
    shell.to_string()
}

pub fn os_type_and_version() -> String {
    let os = os_info::get();
    let os_name = os.os_type();
    let os_version = os.version();
    let os = format!("{} {}", os_name, os_version);
    os
}

#[cfg(test)]
mod tests {
    use super::*;

    // By default rust runs tests in parallel, so we need to run them sequentially because we are setting and removing environment variables
    #[test]
    fn test_shell_detection() {
        test_shell_bash();
        test_shell_cmd();
    }
    fn test_shell_bash() {
        // SAFETY: test-only; std serialises its own env access and only this
        // test touches SHELL.
        unsafe { std::env::set_var("SHELL", "/bin/bash") };
        assert_eq!(shell(), "bash");
        // SAFETY: test-only; std serialises its own env access and only this
        // test touches SHELL.
        unsafe { std::env::remove_var("SHELL") };
    }

    fn test_shell_cmd() {
        // SAFETY: test-only; std serialises its own env access and only this
        // test touches SHELL.
        unsafe { std::env::set_var("SHELL", "cmd.exe") };
        assert_eq!(shell(), "cmd.exe");
        // SAFETY: test-only; std serialises its own env access and only this
        // test touches SHELL.
        unsafe { std::env::remove_var("SHELL") };
    }
}
