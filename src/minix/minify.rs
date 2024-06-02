use regex::Regex;

use crate::{
    minix::utils::Utils,
    minix::regex::RegExp,
};

pub struct Minify;

impl Minify {

    pub fn js(code: &str) -> String {
        let code = Utils::preserve_strings(code);
        let code = Utils::remove_single_line_comments(&code);
        let code = Regex::new(RegExp::MIN_JS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(&code.trim(), "");
        let code = Regex::new(RegExp::MIN_JS_STRING_LITERAL).unwrap().replace_all(&code, "$0");
        let code = Regex::new(RegExp::MIN_JS_REMOVE_OPERATORS_KEYWORDS).unwrap().replace_all(&code, "$1");
        let code = Regex::new(RegExp::MIN_JS_REMOVE_SPACES).unwrap().replace_all(&code, "$1"); // (BROKEN)
    
        let code = Regex::new(
            &format!(r"\b({})\b", RegExp::MIN_JS_KEYWORDS)
        ).unwrap().replace_all(
            &code, "$0"
        );
    
        let code = Regex::new(RegExp::MIN_JS_DUPLICATE_SPACES).unwrap().replace_all(&code, " ");
        let code = Regex::new(RegExp::MIN_JS_LOGICAL_OPERATORS).unwrap().replace_all(&code, "||");
        let code = Regex::new(RegExp::MIN_JS_WHITESPACE_TRIM).unwrap().replace_all(&code, "$1");
        let code = Regex::new(RegExp::MIN_JS_REMOVE_SPACES_AFTER_PAREN_REGEX).unwrap().replace_all(&code, "(");
    
        let code = Regex::new(RegExp::MIN_JS_DOUBLE_QUOTED_STRING).unwrap().replace_all(
            &code, |caps: &regex::Captures| {
                let inner = &caps[0][1..caps[0].len() - 1];
                format!("\"{}\"", inner.replace("\\\"", "\""))
            }
        );

        let code = Utils::remove_empty_lines(&code);
        let code = Utils::remove_line_break_after_semicolon(&code);
    
        code.to_string()
    }

    pub fn css(code: &str) -> String {
        let css = Regex::new(RegExp::MIN_CSS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(code, "");
        let css = Regex::new(RegExp::MIN_CSS_REMOVE_WHITESPACE).unwrap().replace_all(&css, " ");
        let css = Regex::new(RegExp::MIN_CSS_REMOVE_SPACES).unwrap().replace_all(&css, "$1");
        css.to_string()
    }
   
}