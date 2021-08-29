//! @ Corresponding to the procedure that dumps a format file, we have a function
//! that reads one in. The function returns |false| if the dumped format is
//! incompatible with the present \TeX\ table sizes, etc.
//
// @d bad_fmt=6666 {go here if the format file is unacceptable}
// @d too_small(#)==begin wake_up_terminal;
macro_rules! too_small {
    ($globals:expr, $prompt:expr, $lbl_bad_fmt:lifetime) => {{
        wake_up_terminal($globals);
        // wterm_ln('---! Must increase the ',#);
        wterm_ln(make_globals_io_view!($globals), format!("{}{}", "---! Must increase the ", $prompt));
        // @.Must increase the x@>
        // goto bad_fmt;
        goto_forward_label!($lbl_bad_fmt);
        // end
        use crate::section_0004::TeXGlobalsIoView;
        use crate::section_0034::wake_up_terminal;
        use crate::section_0056::wterm_ln;
    }}
}

macro_rules! Undump_the_unicode_support_data {
    ($globals:expr, $lbl_bad_fmt:lifetime) => {{
        if !crate::unicode_support::undump_the_unicode_support_data($globals) {
            goto_forward_label!($lbl_bad_fmt);
        }
    }}
}

// @p @t\4@>@<Declare the function called |open_fmt_file|@>@;
// function load_fmt_file:boolean;
#[allow(unused_variables)]
pub(crate) fn load_fmt_file(globals: &mut TeXGlobals) -> boolean {
    // label bad_fmt,exit;
    // var j,@!k:integer; {all-purpose indices}
    // @!p,@!q: pointer; {all-purpose pointers}
    // @!x: integer; {something undumped}
    // @!w: four_quarters; {four ASCII codes}
    region_forward_label!(
    |'bad_fmt|
    {
        // begin @<Undump constants for consistency check@>;
        Undump_constants_for_consistency_check!(globals, 'bad_fmt);
        // @<Undump the string pool@>;
        Undump_the_string_pool!(globals, 'bad_fmt);
        // @<Undump the dynamic memory@>;
        Undump_the_dynamic_memory!(globals, 'bad_fmt);
        // @<Undump the table of equivalents@>;
        Undump_the_table_of_equivalents!(globals, 'bad_fmt);
        // @<Undump the font information@>;
        Undump_the_font_information!(globals, 'bad_fmt);
        // @<Undump the hyphenation tables@>;
        Undump_the_hyphenation_tables!(globals, 'bad_fmt);
        #[cfg(feature = "unicode_support")]
        Undump_the_unicode_support_data!(globals, 'bad_fmt);
        // @<Undump a couple more things and the closing check word@>;
        Undump_a_couple_more_things_and_the_closing_check_word!(globals, 'bad_fmt);
        // load_fmt_file:=true; return; {it worked!}
        /// it worked!
        return true;
    }
    // bad_fmt: wake_up_terminal;
    'bad_fmt <-
    );
    wake_up_terminal(globals);
    //   wterm_ln('(Fatal format file error; I''m stymied)');
    wterm_ln(
        make_globals_io_view!(globals),
        "(Fatal format file error; I'm stymied)",
    );
    // @.Fatal format file error@>
    // load_fmt_file:=false;
    false
    // exit:end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0034::wake_up_terminal;
use crate::section_0056::wterm_ln;
