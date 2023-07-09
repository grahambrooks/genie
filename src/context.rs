// Read the shell from the SHELL environment variable. and return the name of the shell app
pub fn shell() -> String {
    let shell = std::env::var("SHELL").unwrap();
    let shell = shell.split("/").last().unwrap();
    shell.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell() {
        assert_eq!(shell(), "zsh");
    }

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


// function to read the operating system name and version for mac, linux and windows
pub fn os() -> String {
    let os = os_info::get();
    let os_name = os.os_type();
    let os_version = os.version();
    let os = format!("{} {}", os_name, os_version);
    os
}