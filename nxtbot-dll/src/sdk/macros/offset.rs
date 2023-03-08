#[doc(hidden)]
#[macro_export]
macro_rules! __define_offset3 {
    (OFFSET $var:tt) => {
        faithe::RuntimeOffset::explicit($var)
    };
    (SIG $var:tt) => {
        faithe::RuntimeOffset::pattern($var)
    };
    (PATTERN $var:tt) => {
        faithe::RuntimeOffset::smart($var)
    };
}