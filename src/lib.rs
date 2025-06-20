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
    let mut curr_open = Vec::new();
    let mut curr_token = String::new();
    let mut tokens = vec![];
    let mut skip_next = false;

    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let ch = chars[i];
        let is_within_single = curr_open.last() == Some(&SINGLE_QUOTE);
        let is_within_double = curr_open.last() == Some(&DOUBLE_QUOTE);
        let next_ch = chars.get(i + 1);

        if skip_next {
            skip_next = false;
            continue;
        }

        match ch {
            DOUBLE_QUOTE if !is_within_single => {
                if is_within_double {
                    if curr_token.chars().last() == Some(BACKLASH_QUOTE) {
                        curr_token.pop();
                    } else {
                        println!("running..{}", ch);

                        curr_open.pop();
                    }
                } else {
                    curr_open.push(ch);
                }
            }
            SINGLE_QUOTE if !is_within_double => {
                if is_within_single {
                    if curr_token.chars().last() == Some(BACKLASH_QUOTE) {
                        skip_next = true;
                        curr_token.pop();
                    } else {
                        curr_token.push(ch);
                    }
                } else {
                    curr_open.push(ch);
                }
            }

            WHITESPACE | '\t' | '\n' => {
                if is_within_double || is_within_single {
                    curr_token.push(ch);
                } else if !curr_token.is_empty() {
                    tokens.push(curr_token.clone());
                    curr_token.clear();
                }
            }

            BACKLASH_QUOTE if is_within_double => {
                skip_next = true;
            }

            BACKLASH_QUOTE if !is_within_single => match next_ch {
                Some(&c) => {
                    if c.is_whitespace() {
                        skip_next = true;
                        curr_token.push_str(" ");
                    } else if c == DOUBLE_QUOTE || c == SINGLE_QUOTE {
                        skip_next = true;
                        curr_token.push(c);
                    } else {
                        curr_token.push(c);
                        skip_next = true;
                    }
                }
                None => {}
            },

            _ => curr_token.push(ch),
        }
    }

    if !curr_token.is_empty() {
        tokens.push(curr_token);
    }

    // println!("{:?}", tokens);

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
