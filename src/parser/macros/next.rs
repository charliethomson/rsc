#[macro_export]
macro_rules! next {
    ($rules:expr, $slug:literal) => {
        $rules.next().ok_or(crate::parser::error::missing($slug))?
    };
}
