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
    ($e:expr) => ($crate::Node::String($crate::nodes::String::new($e.to_string())))
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
    ($name:expr) => ($crate::Node::Keyword(
        $crate::nodes::Keyword::new(None, $name.to_string())
    ));
    ($ns:expr, $name:expr) => ($crate::Node::Keyword(
        $crate::nodes::Keyword::new($ns, $name.to_string())
    ));
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
    ($name:expr, $e:expr) => ($crate::Node::Def($crate::nodes::Def::new(
        $crate::nodes::Symbol::new(None, $name.to_string()),
        $e,
    )));
    ($ns:expr, $name:expr, $e:expr) => ($crate::Node::Def($crate::nodes::Def::new(
        $crate::nodes::Symbol::new(Some($ns.to_string()), $name.to_string()),
        $e,
    )));
}

#[macro_export]
macro_rules! n_fn {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Node::Fn($crate::nodes::Fn::new(
        vec![$($params),*],
        vec![$($e),*],
    )));
    ($params:expr, $body:expr) => ($crate::Node::Fn($crate::nodes::Fn::new(
        $params,
        $body,
    )));
}

#[macro_export]
macro_rules! n_macro {
    ([$($params:expr),*], [$($e:expr),*]) => ($crate::Node::Macro($crate::nodes::Macro::new(
        vec![$($params),*],
        vec![$($e),*],
    )));
    ($params:expr, $body:expr) => ($crate::Node::Macro($crate::nodes::Macro::new(
        $params,
        $body,
    )));
}

#[macro_export]
macro_rules! n_call {
    ($name:expr, $args:expr) => ($crate::Node::Call($crate::nodes::Call::new(
        $crate::nodes::Symbol::new(None, $name.to_string()),
        $args,
    )));
    ($ns:expr, $name:expr, $args:expr) => ($crate::Node::Call($crate::nodes::Call::new(
        $crate::nodes::Symbol::new($ns, $name.to_string()),
        $args,
    )));
}

#[macro_export]
macro_rules! n_let {
    ([$($s:expr, $e:expr),*], $($body:expr),*) => ($crate::Node::Let($crate::nodes::Let::new(
        vec![$($s, $e),*],
        vec![$($body),*],
    )));
    ($bindings:expr, $body:expr) => ($crate::Node::Let($crate::nodes::Let::new(
        $bindings,
        $body,
    )));
}

#[macro_export]
macro_rules! n_alias {
    ($ns:expr, $name:expr) => ($crate::Node::Alias(
        $crate::nodes::Alias::new(
            $crate::nodes::Symbol::new(Some($ns.to_string()), $name.to_string()
        ))
    ));
}

#[macro_export]
macro_rules! is_file {
    ($md:expr) => ($md.map(|s| s.is_file()).unwrap_or(false))
}

#[macro_export]
macro_rules! is_file_exists {
    ($md:expr) => ($md.is_ok())
}
