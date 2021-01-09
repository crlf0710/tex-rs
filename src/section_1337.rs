//! @ When we begin the following code, \TeX's tables may still contain garbage;
//! the strings might not even be present. Thus we must proceed cautiously to get
//! bootstrapped in.
//!
//! But when we finish this part of the program, \TeX\ is ready to call on the
//! |main_control| routine to do its work.

// @<Get the first line...@>=
macro_rules! Get_the_first_line_of_input_and_prepare_to_start {
    ($globals:expr, $lbl_end_of_TEX:lifetime, $lbl_final_end:lifetime) => {
        // begin @<Initialize the input routines@>;
        Initialize_the_input_routines!($globals, $lbl_final_end);
        // if (format_ident=0)or(buffer[loc]="&") then
        if $globals.format_ident.is_zero() ||
            $globals.buffer[loc!($globals)] == ASCII_code_literal!(b'&') {
            // begin if format_ident<>0 then initialize; {erase preloaded format}
            if !$globals.format_ident.is_zero() {
                /// erase preloaded format
                initialize($globals);
            }
            // if not open_fmt_file then goto final_end;
            // if not load_fmt_file then
            //   begin w_close(fmt_file); goto final_end;
            //   end;
            // w_close(fmt_file);
            // while (loc<limit)and(buffer[loc]=" ") do incr(loc);
            // end;
        }
        // if end_line_char_inactive then decr(limit)
        if end_line_char_inactive!($globals) {
            decr!(limit!($globals));
        } else {
            // else  buffer[limit]:=end_line_char;
            $globals.buffer[limit!($globals)] = ASCII_code(end_line_char!($globals) as _);
        }
        // fix_date_and_time;@/
        fix_date_and_time($globals);
        // @<Compute the magic offset@>;
        // @<Initialize the print |selector|...@>;
        Initialize_the_print_selector_based_on_interaction!($globals);
        // if (loc<limit)and(cat_code(buffer[loc])<>escape) then start_input;
        if loc!($globals) < limit!($globals) &&
            cat_code!($globals, $globals.buffer[loc!($globals)]) != escape as halfword {
            try_or_jump!(start_input($globals), $lbl_end_of_TEX);
            // {\.{\\input} assumed}
            /// `\input` assumed.
            const _: () = ();
        }
        // end

        use crate::section_0037::init_terminal;
        use crate::section_0207::escape;
        use crate::section_0537::start_input;
        use crate::section_0113::halfword;
        use crate::section_0018::ASCII_code;
    };
}
