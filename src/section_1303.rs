//! @ Corresponding to the procedure that dumps a format file, we have a function
//! that reads one in. The function returns |false| if the dumped format is
//! incompatible with the present \TeX\ table sizes, etc.
//
// @d bad_fmt=6666 {go here if the format file is unacceptable}
// @d too_small(#)==begin wake_up_terminal;
//   wterm_ln('---! Must increase the ',#);
// @.Must increase the x@>
//   goto bad_fmt;
//   end
//
// @p @t\4@>@<Declare the function called |open_fmt_file|@>@;
// function load_fmt_file:boolean;
#[allow(unused_variables)]
pub(crate) fn load_fmt_file(globals: &mut TeXGlobals) -> boolean {
    // label bad_fmt,exit;
    // var j,@!k:integer; {all-purpose indices}
    // @!p,@!q: pointer; {all-purpose pointers}
    // @!x: integer; {something undumped}
    // @!w: four_quarters; {four ASCII codes}
    // begin @<Undump constants for consistency check@>;
    // @<Undump the string pool@>;
    // @<Undump the dynamic memory@>;
    // @<Undump the table of equivalents@>;
    // @<Undump the font information@>;
    // @<Undump the hyphenation tables@>;
    // @<Undump a couple more things and the closing check word@>;
    // load_fmt_file:=true; return; {it worked!}
    // bad_fmt: wake_up_terminal;
    //   wterm_ln('(Fatal format file error; I''m stymied)');
    // @.Fatal format file error@>
    // load_fmt_file:=false;
    // exit:end;
    todo!();
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
