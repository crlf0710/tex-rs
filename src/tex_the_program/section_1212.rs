//! ` `

// @<Discard erroneous...@>=
pub(crate) macro Discard_erroneous_prefixes_and_return($globals:expr) {{
    // begin print_err("You can't use a prefix with `");
    print_err!(
        $globals,
        crate::strpool_str!("You can't use a prefix with `")
    );
    // @.You can't use a prefix with x@>
    // print_cmd_chr(cur_cmd,cur_chr); print_char("'");
    print_cmd_chr($globals, $globals.cur_cmd, $globals.cur_chr);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'\''),
    );
    // help1("I'll pretend you didn't say \long or \outer or \global.");
    help1!(
        $globals,
        crate::strpool_str!("I'll pretend you didn't say \\long or \\outer or \\global.")
    );
    // back_error; return;
    back_error($globals)?;
    crate::return_nojump!();
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0073::print_err;
    use crate::section_0079::help1;
    use crate::section_0298::print_cmd_chr;
    use crate::section_0327::back_error;
}}
