use regex::Regex;

use crate::core::regex::RegExp;

pub struct Utils;

impl Utils {

    pub fn preserve_strings(&self, code: &str) -> String {
        let mut result = String::new();
        let mut is_in_string = false;
        let mut string_char = ' ';

        for (_, c) in code.chars().enumerate() {
            if !is_in_string {
                if c == '\'' || c == '"' || c == '`' {
                    is_in_string = true;
                    string_char = c;
                }

                result.push(c);

            } else if is_in_string {
                result.push(c);

                if c == string_char {
                    is_in_string = false;
                }
            }
        }

        result
    }

    pub fn remove_single_line_comments(&self, code: &str) -> String {
        let re = Regex::new(RegExp::MIN_JS_SINGLE_LINE_COMMENT_REGEX).unwrap();
        let mut result = String::new();
    
        for line in code.lines() {
            if !line.contains("://") {
                let new_line = re.replace(line, "");
                result.push_str(&new_line);
            } else {
                result.push_str(line);
            }

            result.push('\n');
        }
    
        result
    }

    pub fn remove_empty_lines(&self, code: &str) -> String {
        let re = Regex::new(RegExp::MIN_JS_EMPTY_LINES_REGEX).unwrap();
        let result = re.replace_all(code, "");
        result.into_owned()
    }
    
    pub fn remove_line_break_after_semicolon(&self, code: &str) -> String {
        let re = Regex::new(RegExp::MIN_JS_LINE_END_SEMICOLON_REGEX).unwrap();
        let result = re.replace_all(code, ";");
        result.into_owned()
    }

}