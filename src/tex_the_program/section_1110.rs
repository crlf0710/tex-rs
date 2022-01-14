//! ` `

// @<Declare act...@>=
// procedure unpackage;
pub(crate) fn unpackage(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit;
    // var p:pointer; {the box}
    /// the box
    let p: pointer;
    // @!c:box_code..copy_code; {should we copy?}
    /// should we copy?
    let c;
    // begin c:=cur_chr; scan_eight_bit_int; p:=box(cur_val);
    c = globals.cur_chr.get();
    scan_eight_bit_int(globals)?;
    p = r#box!(globals, globals.cur_val);
    // if p=null then return;
    if p == null {
        crate::return_nojump!();
    }
    // if (abs(mode)=mmode)or((abs(mode)=vmode)and(type(p)<>vlist_node))or@|
    //    ((abs(mode)=hmode)and(type(p)<>hlist_node)) then
    if mode!(globals).get().abs() == mmode
        || (mode!(globals).get().abs() == vmode && r#type!(globals, p) != vlist_node)
        || (mode!(globals).get().abs() == hmode && r#type!(globals, p) != hlist_node)
    {
        todo!("incompatible list");
        //   begin print_err("Incompatible list can't be unboxed");
        // @.Incompatible list...@>
        //   help3("Sorry, Pandora. (You sneaky devil.)")@/
        //   ("I refuse to unbox an \hbox in vertical mode or vice versa.")@/
        //   ("And I can't open any boxes in math mode.");@/
        //   error; return;
        //   end;
    }
    // if c=copy_code then link(tail):=copy_node_list(list_ptr(p))
    if c == copy_code as chr_code_repr {
        link!(globals, tail!(globals)) = copy_node_list(globals, list_ptr!(globals, p))?;
    }
    // else  begin link(tail):=list_ptr(p); box(cur_val):=null;
    else {
        link!(globals, tail!(globals)) = list_ptr!(globals, p);
        r#box!(globals, globals.cur_val) = null;
        // free_node(p,box_node_size);
        free_node(globals, p, box_node_size.into());
        // end;
    }
    // while link(tail)<>null do tail:=link(tail);
    while link!(globals, tail!(globals)) != null {
        tail!(globals) = link!(globals, tail!(globals));
    }
    // exit:end;
    use crate::section_0118::link;
    use crate::section_0135::list_ptr;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0130::free_node;
use crate::section_0133::r#type;
use crate::section_0135::box_node_size;
use crate::section_0135::hlist_node;
use crate::section_0137::vlist_node;
use crate::section_0204::copy_node_list;
use crate::section_0211::hmode;
use crate::section_0211::mmode;
use crate::section_0211::vmode;
use crate::section_0213::mode;
use crate::section_0213::tail;
use crate::section_0230::r#box;
use crate::section_0297::chr_code_repr;
use crate::section_0433::scan_eight_bit_int;
use crate::section_1071::copy_code;
