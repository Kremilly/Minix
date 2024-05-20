pub struct RegExp;

impl RegExp {
    
    pub const MIN_JS_REMOVE_WHITESPACE: &'static str = r" +";
    pub const MIN_JS_REMOVE_SINGLE_LINE_COMMENT: &'static str = r"//.*?(?:\n|$)";
    pub const MIN_JS_REMOVE_MULTI_LINE_COMMENT: &'static str = r"/\*.*?\*/";
    pub const MIN_JS_STRING_LITERAL: &'static str = r#""(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\.|[^`\\])*`"#;
    pub const MIN_JS_REMOVE_OPERATORS_KEYWORDS: &'static str = r"(==|===|!==|!=|\+|-|\*|/|&&|\|\||\(|\)|\{|}|=|;)";
    pub const MIN_JS_REMOVE_SPACES: &'static str = r"\s*([=\{\)\]\}])\s*";
    pub const MIN_JS_KEYWORDS: &'static str = r"\b(if|else|for|while|do|switch|case|break|continue|return|function|var|let|pub const)\b";
    pub const MIN_JS_DUPLICATE_SPACES: &'static str = r"\s{2,}";
    pub const MIN_JS_LOGICAL_OPERATORS: &'static str = r"\s*\|\|\s*";
    pub const MIN_JS_DOUBLE_QUOTED_STRING: &'static str = r#""(?:\\.|[^"\\])*""#;
    pub const MIN_JS_WHITESPACE_TRIM: &'static str = r#"\s*([;,+{}:])\s*"#;
    pub const MIN_JS_REMOVE_SPACES_AFTER_PAREN_REGEX: &'static str = r#"\(\s+"#;

    pub const MIN_CSS_REMOVE_MULTI_LINE_COMMENT: &'static str = r"/\*.*?\*/";
    pub const MIN_CSS_REMOVE_WHITESPACE: &'static str = r"\s+";
    pub const MIN_CSS_REMOVE_SPACES: &'static str = r"\s*([{}:;,>])\s*";
   
}
