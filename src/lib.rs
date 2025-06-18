use std::io;

use shlex::Shlex;

pub mod command;

const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';

pub fn find_cmd_in_path(cmd: &str, path: &[String]) -> Option<String> {
    path.iter()
        .map(|path| read_dir(path))
        .filter_map(Result::ok)
        .flatten()
        .find(|path| path.ends_with(&format!("/{}", &cmd)))
}

pub fn read_dir(path: &str) -> io::Result<Vec<String>> {
    let entries = std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    Ok(entries)
}

pub fn read_path_env() -> Vec<String> {
    let path = std::env::var("PATH");

    match path {
        Ok(path) => path
            .split(":")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>(),
        Err(_) => Vec::new(),
    }
}

pub fn parse_input(input: &str) -> Vec<String> {
    let lexer = Shlex::new(&input);
    let args = lexer.collect();

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_single_word() {
        let input = "hello";
        let expected = vec!["hello".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_parse_input_multiple_words() {
        let input = "hello world";
        let expected = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_single_quote_literal() {
        let input = "'hello'";
        let expected = vec!["hello".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_single_quote_literal_with_whitespace() {
        let input = "'hello world'";
        let expected = vec!["hello world".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_single_quote_literal_with_multiple_words() {
        let input = "'hello world'";
        let expected = vec!["hello world".to_string()];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_single_quote_literal_with_multiple_words_and_whitespace() {
        let input = "'hello     world' 'example''script' shell''test";
        let expected = vec!["shell     world hello example test script".to_string()];
        assert_eq!(parse_input(input), expected);
    }
}
