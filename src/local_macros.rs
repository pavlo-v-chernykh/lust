macro_rules! t_number {
    ($val:expr, $span:expr) => (::lexer::Token::Number { val: $val, span: $span, });
}

macro_rules! t_string {
    ($val:expr, $span:expr) => (::lexer::Token::String { val: $val.to_string(), span: $span, });
}

macro_rules! t_symbol {
    ($val:expr, $span:expr) => (::lexer::Token::Symbol { val: $val.to_string(), span: $span, });
}

macro_rules! t_keyword {
    ($val:expr, $span:expr) => (::lexer::Token::Keyword { val: $val.to_string(), span: $span, });
}

macro_rules! t_list_start {
    ($span:expr) => (::lexer::Token::ListStart { span: $span });
}

macro_rules! t_list_end {
    ($span:expr) => (::lexer::Token::ListEnd { span: $span });
}

macro_rules! t_vec_start {
    ($span:expr) => (::lexer::Token::VecStart { span: $span });
}

macro_rules! t_vec_end {
    ($span:expr) => (::lexer::Token::VecEnd { span: $span });
}

macro_rules! t_quote {
    ($span:expr) => (::lexer::Token::Quote { span: $span });
}

macro_rules! t_unquote {
    ($span:expr) => (::lexer::Token::Unquote { span: $span });
}

macro_rules! t_unquote_splicing {
    ($span:expr) => (::lexer::Token::UnquoteSplicing { span: $span });
}

macro_rules! span {
    ($start_line:expr, $start_col:expr, $end_line:expr, $end_col:expr) => (
        ::lexer::Span::new($start_line, $start_col, $end_line, $end_col);
    )
}

macro_rules! e_number {
    ($e:expr) => (::ast::Expr::Number($e))
}

macro_rules! e_bool {
    ($e:expr) => (::ast::Expr::Bool($e))
}

macro_rules! e_string {
    ($e:expr) => (::ast::Expr::String($e.to_string()))
}

macro_rules! e_symbol {
    ($e:expr) => (::ast::Expr::Symbol($e.to_string()))
}

macro_rules! e_keyword {
    ($e:expr) => (::ast::Expr::Keyword($e.to_string()))
}

macro_rules! e_list {
    ($($e:expr),*) => (::ast::Expr::List(vec![$($e),*]))
}

macro_rules! e_vec {
    ($($e:expr),*) => (::ast::Expr::Vec(vec![$($e),*]))
}

macro_rules! e_def {
    ($sym:expr, $e:expr) => (::ast::Expr::Def {
        sym: $sym.to_string(),
        expr: Box::new($e),
    })
}

macro_rules! e_fn {
    ([$($params:expr),*], [$($e:expr),*]) => (::ast::Expr::Fn {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

macro_rules! e_macro {
    ([$($params:expr),*], [$($e:expr),*]) => (::ast::Expr::Macro {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

macro_rules! e_call {
    ($name:expr, $($arg:expr),*) => (::ast::Expr::Call {
        name: $name.to_string(),
        args: vec![$($arg),*],
    })
}

macro_rules! e_let {
    ([$($s:expr, $e:expr),*], $($body:expr),*) => (::ast::Expr::Let {
        bindings: vec![$($s, $e),*],
        body: vec![$($body),*],
    })
}
