macro_rules! try_ok {
    ($e:expr) => (match $e {
        Ok(res) => {
            res
        },
        Err(err) => {
            return println!("Whoops, error detected.\n{}.\n\
                             Please, try again...", err)
        }
    })
}
