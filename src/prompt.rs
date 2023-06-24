pub(crate) fn build(args: &mut Vec<String>) -> String {
    if !args.is_empty() {
        args.remove(0); // Remove the first element without shifting the others
    }
    return args.join(" ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prompt() {
        let mut args: Vec<String> = vec!["".to_string(), "hello".to_string(), "world".to_string()];
        let prompt = build(&mut args);
        assert_eq!(prompt, "hello world");
    }
}
