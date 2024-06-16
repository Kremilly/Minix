pub struct RegExp;

impl RegExp {
    
    pub const MIN_JS_REMOVE_MULTI_LINE_COMMENT: &'static str = r"/\*.*?\*/";
    pub const MIN_JS_STRING_LITERAL: &'static str = r#""(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\.|[^`\\])*`"#;
    pub const MIN_JS_EMPTY_LINES_REGEX: &'static str = r"(?m)^\s*$\n?";
    pub const MIN_JS_SINGLE_LINE_COMMENT_REGEX: &'static str = r"//.*$";
    pub const MIN_JS_LINE_END_SEMICOLON_REGEX: &'static str = r";\n";
    pub const MIN_JS_REMOVE_SPACES: &'static str = r"\s*([=\{\)\]\}])\s*";
    pub const MIN_JS_DUPLICATE_SPACES: &'static str = r"\s{2,}";
    pub const MIN_JS_LOGICAL_OPERATORS: &'static str = r"\s*\|\|\s*";
    pub const MIN_JS_DOUBLE_QUOTED_STRING: &'static str = r#""(?:\\.|[^"\\])*""#;
    pub const MIN_JS_WHITESPACE_TRIM: &'static str = r#"\s*([,+{}:])\s*"#;
    pub const MIN_JS_REMOVE_SPACES_AFTER_PAREN_REGEX: &'static str = r#"\(\s+"#;
    pub const MIN_JS_KEYWORDS: &'static str = r"var|let|const|if|else|for|while|function|return|class|new";
    pub const MIN_JS_REMOVE_OPERATORS_KEYWORDS: &'static str = r"\s*(=|\{|\}|:|,|\[|\]|\(|\)|<|>|\+|\-|\*|/|&|\||\^|!|~|\?|;|\.\.\.)\s*";

    pub const MIN_CSS_REMOVE_MULTI_LINE_COMMENT: &'static str = r"/\*.*?\*/";
    pub const MIN_CSS_REMOVE_WHITESPACE: &'static str = r"\s+";
    pub const MIN_CSS_REMOVE_SPACES: &'static str = r"\s*([{}:;,>])\s*";
   
}
