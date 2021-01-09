//! ` `
// @<Complete a potentially long \.{\\show} command@>=
macro_rules! Complete_a_potentially_long_show_command {
    ($globals:expr) => {{
        // end_diagnostic(true); print_err("OK");
        end_diagnostic($globals, true);
        print_err!($globals, strpool_str!("OK"));
        // @.OK@>
        // if selector=term_and_log then if tracing_online<=0 then
        if $globals.selector == term_and_log && tracing_online!($globals) <= 0 {
            // begin selector:=term_only; print(" (see the transcript file)");
            $globals.selector = term_only.into();
            print($globals, strpool_str!(" (see the transcript file)").get() as _);
            // selector:=term_and_log;
            $globals.selector = term_and_log.into();
            // end
        }

        use crate::section_0054::term_only;
        use crate::section_0054::term_and_log;
        use crate::section_0059::print;
        use crate::section_0245::end_diagnostic;
    }}
}
