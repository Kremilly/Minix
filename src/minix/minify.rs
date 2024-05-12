use regex::Regex;

pub struct Minify;

impl Minify {
    
    const MIN_JS_REMOVE_WHITESPACE: &'static str = r" +";
    const MIN_JS_REMOVE_SINGLE_LINE_COMMENT: &'static str = r"//.*?(?:\n|$)";
    const MIN_JS_REMOVE_MULTI_LINE_COMMENT: &'static str = r"/\*.*?\*/";
    const MIN_JS_REMOVE_STRINGS: &'static str = r#""[^"\\]*(?:\\.[^"\\]*)*""#;
    const MIN_JS_REMOVE_OPERATORS_KEYWORDS: &'static str = r"(==|===|!==|!=|\+|-|\*|/|&&|\|\||\(|\)|\{|}|=|;)";
    const MIN_JS_REMOVE_SPACES: &'static str = r"\s*([=\{\)\]\}])\s*";
    const MIN_JS_KEYWORDS: &'static str = r"\b(if|else|for|while|do|switch|case|break|continue|return|function|var|let|const)\b";
    const MIN_JS_DUPLICATE_SPACES: &'static str = r"\s{2,}";
    const MIN_JS_LOGICAL_OPERATORS: &'static str = r"\s*\|\|\s*";
    const MIN_JS_DOUBLE_QUOTED_STRING: &'static str = r#""(?:\\.|[^"\\])*""#;
    const MIN_JS_WHITESPACE_TRIM: &'static str = r#"\s*([;,+{}:])\s*"#;

    const MIN_CSS_REMOVE_MULTI_LINE_COMMENT: &'static str = r"/\*.*?\*/";
    const MIN_CSS_REMOVE_WHITESPACE: &'static str = r"\s+";
    const MIN_CSS_REMOVE_SPACES: &'static str = r"\s*([{}:;,>])\s*";

    pub fn js(code: &str) -> String {
        let code = Regex::new(Self::MIN_JS_REMOVE_WHITESPACE).unwrap().replace_all(code, " ");
        let code = Regex::new(Self::MIN_JS_REMOVE_SINGLE_LINE_COMMENT).unwrap().replace_all(&code, "");
        let code = Regex::new(Self::MIN_JS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(&code.trim(), "");
        let code = Regex::new(Self::MIN_JS_REMOVE_STRINGS).unwrap().replace_all(&code, "\"\"");
        let code = Regex::new(Self::MIN_JS_REMOVE_OPERATORS_KEYWORDS).unwrap().replace_all(&code, "$1");
        let code = Regex::new(Self::MIN_JS_REMOVE_SPACES).unwrap().replace_all(&code, "$1");

        let code = Regex::new(
            &format!(r"\b({})\b", Self::MIN_JS_KEYWORDS)
        ).unwrap().replace_all(
            &code, " $1 "
        );

        let code = Regex::new(Self::MIN_JS_DUPLICATE_SPACES).unwrap().replace_all(&code, " ");
        let code = Regex::new(Self::MIN_JS_LOGICAL_OPERATORS).unwrap().replace_all(&code, "||");
        let code = Regex::new(Self::MIN_JS_WHITESPACE_TRIM).unwrap().replace_all(&code, "$1");

        let code = Regex::new(Self::MIN_JS_DOUBLE_QUOTED_STRING).unwrap().replace_all(
            &code, |caps: &regex::Captures| {
                let inner = &caps[0][1..caps[0].len() - 1];
                format!("\"{}\"", inner.replace("\\\"", "\""))
            }
        );

        code.to_string()
    }

    pub fn css(code: &str) -> String {
        let css = Regex::new(Self::MIN_CSS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(code, "");
        let css = Regex::new(Self::MIN_CSS_REMOVE_WHITESPACE).unwrap().replace_all(&css, " ");
        let css = Regex::new(Self::MIN_CSS_REMOVE_SPACES).unwrap().replace_all(&css, "$1");
        css.to_string()
    }
   
}
