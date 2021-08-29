//! ` `

// @<Discard the prefixes...@>=
macro_rules! Discard_the_prefixes_long_and_outer_if_they_are_irrelevant {
    ($globals:expr, $a:expr) => {{
        // if (cur_cmd<>def)and(a mod 4<>0) then
        if $globals.cur_cmd != def && $a % 4 != 0 {
            todo!("discard the prefixes");
            // begin print_err("You can't use `"); print_esc("long"); print("' or `");
            // print_esc("outer"); print("' with `");
            // @.You can't use \\long...@>
            // print_cmd_chr(cur_cmd,cur_chr); print_char("'");
            // help1("I'll pretend you didn't say \long or \outer here.");
            // error;
            // end
        }
    }}
}