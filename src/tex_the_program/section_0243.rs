//! ` `
// @<Set variable |c| to the current escape character@>=c:=escape_char
pub(crate) macro Set_variable_c_to_the_current_escape_character($globals:expr, $c:expr) {{
    $c = escape_char!($globals);
    use crate::section_0236::escape_char;
}}
