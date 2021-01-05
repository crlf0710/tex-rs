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
    else if globals.cur_chr.get() == open_node as chr_code_repr {
        Implement_openout!(globals);
    }
    // write_node:@<Implement \.{\\write}@>;
    else if globals.cur_chr.get() == write_node as chr_code_repr {
        Implement_write!(globals);
    }
    // close_node:@<Implement \.{\\closeout}@>;
    else if globals.cur_chr.get() == close_node as chr_code_repr {
        Implement_closeout!(globals);
    }
    // special_node:@<Implement \.{\\special}@>;
    // immediate_code:@<Implement \.{\\immediate}@>;
    else if globals.cur_chr.get() == immediate_code as chr_code_repr {
        Implement_immediate!(globals);
    }
    // set_language_code:@<Implement \.{\\setlanguage}@>;
    // othercases confusion("ext1")
    else {
        confusion(globals, strpool_str!("ext1"))?;
    }
    // @:this can't happen ext1}{\quad ext1@>
    // endcases;
    ok_nojump!()
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0297::chr_code_repr;
use crate::section_1341::close_node;
use crate::section_1341::open_node;
use crate::section_1341::write_node;
use crate::section_1344::immediate_code;
