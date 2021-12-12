//! ` `

// @<Declare act...@>=
// procedure sub_sup;
#[allow(unused_variables)]
pub(crate) fn sub_sup(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var t:small_number; {type of previous sub/superscript}
    /// type of previous sub/superscript
    let mut t;
    // @!p:pointer; {field to be filled by |scan_math|}
    /// field to be filled by `scan_math`
    let mut p;
    // begin t:=empty; p:=null;
    t = math_type_kind::empty as _;
    p = null;
    // if tail<>head then if scripts_allowed(tail) then
    if tail!(globals) != head!(globals) && scripts_allowed!(globals, tail!(globals)) {
        // begin p:=supscr(tail)+cur_cmd-sup_mark; {|supscr| or |subscr|}
        /// `supscr` or `subscr`
        const _: () = ();
        p = supscr!(tail!(globals)) + (globals.cur_cmd - sup_mark) as pointer;
        // t:=math_type(p);
        t = math_type!(globals, p);
        // end;
    }
    // if (p=null)or(t<>empty) then @<Insert a dummy noad to be sub/superscripted@>;
    if p == null || t != math_type_kind::empty as _ {
        todo!("Insert a dummy noad");
    }
    // scan_math(p);
    scan_math(globals, p)?;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0207::sup_mark;
use crate::section_0213::head;
use crate::section_0213::tail;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::supscr;
use crate::section_0687::scripts_allowed;
use crate::section_1151::scan_math;
