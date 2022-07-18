/// macro for a more clean way to early return with `Option`
#[macro_export]
macro_rules! some_or_return {
    ($option:expr) => {
        match $option {
            Some(x) => x,
            None => return,
        }
    };
}

/// macro for a more clean way to early return with `Result`
#[macro_export]
macro_rules! ok_or_return {
    ($option:expr) => {
        match $option {
            Ok(x) => x,
            Err(_) => return,
        }
    };
}
