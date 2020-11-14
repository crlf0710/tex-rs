//! ` `
// @<Set variable |c| to the current escape character@>=c:=escape_char
macro_rules! Set_variable_c_to_the_current_escape_character {
    ($globals:expr, $c:expr) => {
        $c = escape_char!($globals)
    }
}
