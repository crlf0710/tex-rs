//! ` `

// @<Complain that \.{\\the} can't do this; give zero result@>=
pub(crate) macro Complain_that_the_cant_do_this__give_zero_result($globals:expr, $level:expr) {{
    // begin print_err("You can't use `"); print_cmd_chr(cur_cmd,cur_chr);
    print_err!($globals, crate::strpool_str!("You can't use `"));
    print_cmd_chr($globals, $globals.cur_cmd, $globals.cur_chr);
    // @.You can't use x after ...@>
    // print("' after "); print_esc("the");
    print($globals, crate::strpool_str!("' after ").get() as _);
    print_esc($globals, crate::strpool_str!("the"));
    // help1("I'm forgetting what you said and using zero instead.");
    help1!(
        $globals,
        crate::strpool_str!("I'm forgetting what you said and using zero instead.")
    );
    // error;
    error($globals)?;
    // if level<>tok_val then scanned_result(0)(dimen_val)
    if $level.get() != cur_val_level_kind::tok_val as u8 {
        scanned_result!($globals, 0, cur_val_level_kind::dimen_val);
    }
    // else scanned_result(0)(int_val);
    else {
        scanned_result!($globals, 0, cur_val_level_kind::int_val);
    }
    // end
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0073::print_err;
    use crate::section_0079::help1;
    use crate::section_0082::error;
    use crate::section_0298::print_cmd_chr;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0413::scanned_result;
}}
