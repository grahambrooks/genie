pub fn shell() -> String {
    let shell_env = std::env::var("SHELL").unwrap();
    let shell = shell_env.split('/').last().unwrap();
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
        std::env::set_var("SHELL", "/bin/bash");
        assert_eq!(shell(), "bash");
        std::env::remove_var("SHELL");
    }

    fn test_shell_cmd() {
        std::env::set_var("SHELL", "cmd.exe");
        assert_eq!(shell(), "cmd.exe");
        std::env::remove_var("SHELL");
    }
}

