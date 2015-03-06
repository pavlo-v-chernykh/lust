macro_rules! try_unwrap {
    ($e:expr, $r:expr) => (match $e {
        Some(v) => {
            v
        },
        _ => {
            return $r
        }
    });
}
