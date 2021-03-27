//! ` `

macro_rules! Dump_the_unicode_support_data {
    ($globals:expr) => {{
        crate::unicode_support::dump_the_unicode_support_data($globals);
    }}
}

// @<Declare act...@>=
// @!init procedure store_fmt_file;
#[cfg(feature = "initex")]
#[allow(unused_variables)]
pub(crate) fn store_fmt_file(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label found1,found2,done1,done2;
    // var j,@!k,@!l:integer; {all-purpose indices}
    // @!p,@!q: pointer; {all-purpose pointers}
    // @!x: integer; {something to dump}
    // @!w: four_quarters; {four ASCII codes}
    // begin @<If dumping is not allowed, abort@>;
    If_dumping_is_not_allowed__abort!(globals);
    // @<Create the |format_ident|, open the format file,
    //   and inform the user that dumping has begun@>;
    Create_the_format_ident__open_the_format_file__and_inform_the_user_that_dumping_has_begun!(globals);
    // @<Dump constants for consistency check@>;
    Dump_constants_for_consistency_check!(globals);
    // @<Dump the string pool@>;
    Dump_the_string_pool!(globals);
    // @<Dump the dynamic memory@>;
    Dump_the_dynamic_memory!(globals);
    // @<Dump the table of equivalents@>;
    Dump_the_table_of_equivalents!(globals);
    // @<Dump the font information@>;
    Dump_the_font_information!(globals);
    // @<Dump the hyphenation tables@>;
    Dump_the_hyphenation_tables!(globals);
    #[cfg(feature = "unicode_support")]
    Dump_the_unicode_support_data!(globals);
    // @<Dump a couple more things and the closing check word@>;
    Dump_a_couple_more_things_and_the_closing_check_word!(globals);
    // @<Close the format file@>;
    Close_the_format_file!(globals);
    // end;
    ok_nojump!()
    // tini
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;

