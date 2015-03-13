macro_rules! t_number {
    ($val:expr, $span:expr) => ($crate::lexer::Token::Number { val: $val, span: $span, });
}

macro_rules! t_string {
    ($val:expr, $span:expr) => ($crate::lexer::Token::String { val: $val.to_string(), span: $span, });
}

macro_rules! t_symbol {
    ($val:expr, $span:expr) => ($crate::lexer::Token::Symbol { val: $val.to_string(), span: $span, });
}

macro_rules! t_list_start {
    ($span:expr) => ($crate::lexer::Token::ListStart { span: $span });
}

macro_rules! t_list_end {
    ($span:expr) => ($crate::lexer::Token::ListEnd { span: $span });
}

macro_rules! t_quote {
    ($span:expr) => ($crate::lexer::Token::Quote { span: $span });
}

macro_rules! t_unquote {
    ($span:expr) => ($crate::lexer::Token::Unquote { span: $span });
}

macro_rules! span {
    ($start_line:expr, $start_col:expr, $end_line:expr, $end_col:expr) => (
        $crate::lexer::Span::new($start_line, $start_col, $end_line, $end_col);
    )
}
