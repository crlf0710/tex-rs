//! ` `
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
    // @<Dump the string pool@>;
    // @<Dump the dynamic memory@>;
    // @<Dump the table of equivalents@>;
    // @<Dump the font information@>;
    // @<Dump the hyphenation tables@>;
    // @<Dump a couple more things and the closing check word@>;
    // @<Close the format file@>;
    Close_the_format_file!(globals);
    // end;
    ok_nojump!()
    // tini
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;

