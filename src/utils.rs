use ast::Node;

pub fn format_vec(v: &[Node]) -> String {
    let mut a = String::new();
    if !v.is_empty() {
        let last_idx = v.len() - 1;
        for (i, e) in v.iter().enumerate() {
            if i < last_idx {
                a.push_str(&format!("{} ", e))
            } else {
                a.push_str(&format!("{}", e))
            }
        }
    }
    a
}
