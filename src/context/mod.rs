use os_info;

// Read the shell from the SHELL environment variable. and return the name of the shell app
pub fn shell() -> String {
    let shell_env = std::env::var("SHELL").unwrap();
    let shell = shell_env.split('/').last().unwrap();
    shell.to_string()
}

pub fn os() -> String {
    let os = os_info::get();
    let os_name = os.os_type();
    let os_version = os.version();
    let os = format!("{} {}", os_name, os_version);
    os
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_bash() {
        std::env::set_var("SHELL", "/bin/bash");
        assert_eq!(shell(), "bash");
        std::env::remove_var("SHELL");
    }

    #[test]
    fn test_shell_cmd() {
        std::env::set_var("SHELL", "cmd.exe");
        assert_eq!(shell(), "cmd.exe");
        std::env::remove_var("SHELL");
    }
}

