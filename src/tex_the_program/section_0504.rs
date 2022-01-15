//! ` `

// @<Test if an integer is odd@>=
pub(crate) macro Test_if_an_integer_is_odd($globals:expr, $b:expr) {{
    // begin scan_int; b:=odd(cur_val);
    scan_int($globals)?;
    $b = $globals.cur_val.is_odd();
    // end
    use crate::pascal::IsOddOrEven;
    use crate::section_0440::scan_int;
}}
