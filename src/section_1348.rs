//! ` `
// @<Declare act...@>=
// @t\4@>@<Declare procedures needed in |do_extension|@>@;
// procedure do_extension;
pub(crate) fn do_extension(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var i,@!j,@!k:integer; {all-purpose integers}
    // @!p,@!q,@!r:pointer; {all-purpose pointers}
    // begin case cur_chr of
    if false {
        unreachable!()
    }
    // open_node:@<Implement \.{\\openout}@>;
    // write_node:@<Implement \.{\\write}@>;
    // close_node:@<Implement \.{\\closeout}@>;
    // special_node:@<Implement \.{\\special}@>;
    // immediate_code:@<Implement \.{\\immediate}@>;
    else if globals.cur_chr.get() == immediate_code as _ {
        Implement_immediate!(globals);
    }
    // set_language_code:@<Implement \.{\\setlanguage}@>;
    // othercases confusion("ext1")
    else {
        confusion(globals, strpool_str!("ext1"));
    }
    // @:this can't happen ext1}{\quad ext1@>
    // endcases;
    ok_nojump!()
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_1344::immediate_code;
