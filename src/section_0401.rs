//! ` `

// @<Show the text of the macro being expanded@>=
macro_rules! Show_the_text_of_the_macro_being_expanded {
    ($globals:expr, $ref_count:expr) => {{
        // begin begin_diagnostic; print_ln; print_cs(warning_index);
        begin_diagnostic($globals);
        print_ln(make_globals_io_view!($globals));
        print_cs($globals, $globals.warning_index as _);
        // token_show(ref_count); end_diagnostic(false);
        token_show($globals, $ref_count);
        end_diagnostic($globals, false);
        // end
        use crate::section_0057::print_ln;
        use crate::section_0245::begin_diagnostic;
        use crate::section_0245::end_diagnostic;
        use crate::section_0262::print_cs;
        use crate::section_0295::token_show;
    }}
}