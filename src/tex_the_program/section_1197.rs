//! @ \TeX\ gets to the following part of the program when the first `\.\$' ending
//! a display has been scanned.
//
// @<Check that another \.\$ follows@>=
pub(crate) macro Check_that_another_dollar_follows($globals:expr) {{
    // begin get_x_token;
    get_x_token($globals)?;
    // if cur_cmd<>math_shift then
    if $globals.cur_cmd != math_shift {
        // begin print_err("Display math should end with $$");
        // @.Display math...with \$\$@>
        // help2("The `$' that I just saw supposedly matches a previous `$$'.")@/
        //   ("So I shall assume that you typed `$$' both times.");
        // back_error;
        // end;
        todo!("cur_cmd != math_shift");
    }
    // end
}}

use crate::section_0207::math_shift;
use crate::section_0380::get_x_token;
