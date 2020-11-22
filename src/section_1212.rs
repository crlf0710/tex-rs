//! ` `

// @<Discard erroneous...@>=
macro_rules! Discard_erroneous_prefixes_and_return {
    ($globals:expr) => {{
        // begin print_err("You can't use a prefix with `");
        print_err!($globals, strpool_str!("You can't use a prefix with `"));
        // @.You can't use a prefix with x@>
        // print_cmd_chr(cur_cmd,cur_chr); print_char("'");
        print_cmd_chr($globals, $globals.cur_cmd, $globals.cur_chr);
        print_char(
            make_globals_io_string_view!($globals),
            ASCII_code_literal!(b'\''),
        );
        // help1("I'll pretend you didn't say \long or \outer or \global.");
        todo!();
        // back_error; return;
        back_error($globals)?;
        return_nojump!();
        // end
        use crate::section_0004::TeXGlobalsIoStringView;
        use crate::section_0058::print_char;
        use crate::section_0298::print_cmd_chr;
        use crate::section_0327::back_error;
    }};
}
