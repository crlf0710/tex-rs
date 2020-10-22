//! @ Here is the very first thing that \TeX\ prints: a headline that identifies
//! the version number and format package. The |term_offset| variable is temporarily
//! incorrect, but the discrepancy is not serious since we assume that the banner
//! and format identifier together will occupy at most |max_print_line|
//! character positions.

// @<Initialize the output...@>=
macro_rules! Initialize_the_output_routines {
    ($globals:expr) => {
        // wterm(banner);
        wterm($globals, banner);

        // if format_ident=0 then wterm_ln(' (no format preloaded)')
        if $globals.format_ident.is_zero() {
            wterm_ln($globals, " (no format preloaded)");
        } else {
            // else  begin slow_print(format_ident); print_ln;
            slow_print($globals, $globals.format_ident.into());
            print_ln($globals);
            // end;
        }

        // update_terminal;
        update_terminal($globals);

        use crate::section_0002::banner;
        use crate::section_0034::update_terminal;
        use crate::section_0056::wterm;
        use crate::section_0056::wterm_ln;
        use crate::section_0057::print_ln;
        use crate::section_0060::slow_print;
    };
}

migration_complete!();
