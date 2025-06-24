use std::io;

pub mod command;

const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';
const BACKLASH_QUOTE: char = '\\';
// const ESCAPED_DOUBLE_QUOTE: char = '\"';
const WHITESPACE: char = ' ';

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
    let mut curr_token = String::new();
    let mut tokens = vec![];
    let mut skip_next = false;
    let mut is_single_quote = false;
    let mut is_double_quote = false;

    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let ch = chars[i];
        let next_ch = chars.get(i + 1);

        match ch {
            _ if skip_next => {
                curr_token.push(ch);
                skip_next = false;
            }

            DOUBLE_QUOTE => {
                if is_single_quote {
                    curr_token.push(ch);
                } else {
                    is_double_quote = !is_double_quote;
                }
            }

            SINGLE_QUOTE => {
                if !is_double_quote {
                    is_single_quote = !is_single_quote;
                } else {
                    curr_token.push(ch);
                }
            }

            WHITESPACE => {
                if is_double_quote || is_single_quote || skip_next {
                    curr_token.push(ch);
                    continue;
                }

                if !curr_token.is_empty() {
                    tokens.push(curr_token);
                    curr_token = String::new();
                }
            }

            BACKLASH_QUOTE => {
                if is_single_quote {
                    curr_token.push(ch);
                } else if is_double_quote {
                    match next_ch {
                        Some(&c) => {
                            if c == BACKLASH_QUOTE || c == DOUBLE_QUOTE {
                                skip_next = true;
                            } else {
                                curr_token.push(ch);
                            }
                        }
                        None => {}
                    }
                } else {
                    skip_next = true;
                }
            }

            _ => curr_token.push(ch),
        }
    }

    if !curr_token.is_empty() {
        tokens.push(curr_token);
    }

    tokens
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
