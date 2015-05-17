macro_rules! t_number {
    ($val:expr, $span:expr) => (::lexer::Token::Number { val: $val, span: $span, });
}

macro_rules! t_string {
    ($val:expr, $span:expr) => (::lexer::Token::String { val: $val.to_string(), span: $span, });
}

macro_rules! t_symbol {
    ($name:expr, $span:expr) => (::lexer::Token::Symbol {
        ns: None,
        name: $name.to_string(),
        span: $span,
    });
    ($ns:expr, $name:expr, $span:expr) => (::lexer::Token::Symbol {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
        span: $span,
    });
}

macro_rules! t_keyword {
    ($name:expr, $span:expr) => (::lexer::Token::Keyword {
        ns: None,
        name: $name.to_string(),
        span: $span,
    });
    ($ns:expr, $name:expr, $span:expr) => (::lexer::Token::Keyword {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
        span: $span,
    });
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

macro_rules! t_syntax_quote {
    ($span:expr) => (::lexer::Token::SyntaxQuote { span: $span });
}

macro_rules! span {
    ($start_line:expr, $start_col:expr, $end_line:expr, $end_col:expr) => (
        ::lexer::Span::new($start_line, $start_col, $end_line, $end_col);
    )
}

#[macro_export]
macro_rules! n_number {
    ($e:expr) => ($crate::Node::Number($crate::nodes::Number::new($e)))
}

#[macro_export]
macro_rules! n_bool {
    ($e:expr) => ($crate::Node::Bool($crate::nodes::Bool::new($e)))
}

#[macro_export]
macro_rules! n_string {
    ($e:expr) => ($crate::Node::String($e.to_string()))
}

#[macro_export]
macro_rules! n_symbol {
    ($name:expr) => ($crate::Node::Symbol(
        $crate::nodes::Symbol::new(None, $name.to_string())
    ));
    ($ns:expr, $name:expr) => ($crate::Node::Symbol(
        $crate::nodes::Symbol::new($ns, $name.to_string())
    ));
}

#[macro_export]
macro_rules! n_keyword {
    ($name:expr) => ($crate::Node::Keyword {
        ns: None,
        name: $name.to_string(),
    });
    ($ns:expr, $name:expr) => ($crate::Node::Keyword {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
    });
}

#[macro_export]
macro_rules! n_list {
    ($($e:expr),*) => ($crate::Node::List(vec![$($e),*]))
}

#[macro_export]
macro_rules! n_vec {
    ($($e:expr),*) => ($crate::Node::Vec(vec![$($e),*]))
}

#[macro_export]
macro_rules! n_def {
    ($sym:expr, $e:expr) => ($crate::Node::Def {
        sym: $sym.to_string(),
        expr: Box::new($e),
    })
}

#[macro_export]
macro_rules! n_fn {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Node::Fn {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! n_macro {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Node::Macro {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

#[macro_export]
macro_rules! n_call {
    ($name:expr, $($arg:expr),*) => ($crate::Node::Call {
        ns: None,
        name: $name.to_string(),
        args: vec![$($arg),*],
    });
    ($ns:expr, $name:expr, $($arg:expr),*) => ($crate::Node::Call {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
        args: vec![$($arg),*],
    });
}

#[macro_export]
macro_rules! n_let {
    ([$($s:expr, $e:expr),*], $($body:expr),*) => ($crate::Node::Let {
        bindings: vec![$($s, $e),*],
        body: vec![$($body),*],
    })
}

#[macro_export]
macro_rules! n_alias {
    ($ns:expr, $name:expr) => ($crate::Node::Alias {
        ns: $ns.to_string(),
        name: $name.to_string(),
    });
}

#[macro_export]
macro_rules! is_file {
    ($md:expr) => ($md.map(|s| s.is_file()).unwrap_or(false))
}

#[macro_export]
macro_rules! is_file_exists {
    ($md:expr) => ($md.is_ok())
}
