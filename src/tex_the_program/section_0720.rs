//! @ The recursion in |mlist_to_hlist| is due primarily to a subroutine
//! called |clean_box| that puts a given noad field into a box using a given
//! math style; |mlist_to_hlist| can call |clean_box|, which can call
//! |mlist_to_hlist|.
//! @^recursion@>
//!
//! The box returned by |clean_box| is ``clean'' in the
//! sense that its |shift_amount| is zero.
//
// @p procedure@?mlist_to_hlist; forward;@t\2@>@/
// function clean_box(@!p:pointer;@!s:small_number):pointer;
#[allow(unused_variables)]
pub(crate) fn clean_box(
    globals: &mut TeXGlobals,
    p: pointer,
    s: small_number,
) -> TeXResult<pointer> {
    // label found;
    // var q:pointer; {beginning of a list to be boxed}
    /// beginning of a list to be boxed
    let q;
    // @!save_style:small_number; {|cur_style| to be restored}
    /// `cur_style` to be restored
    let save_style;
    // @!x:pointer; {box to be returned}
    /// box to be returned
    let x;
    // @!r:pointer; {temporary pointer}
    // begin case math_type(p) of
    let math_type_p = math_type!(globals, p);
    // math_char: begin cur_mlist:=new_noad; mem[nucleus(cur_mlist)]:=mem[p];
    if math_type_p == math_type_kind::math_char as _ {
        globals.cur_mlist = new_noad(globals)?;
        globals.mem[nucleus!(globals.cur_mlist)] = globals.mem[p];
        // end;
    }
    // sub_box: begin q:=info(p); goto found;
    else if math_type_p == math_type_kind::sub_box as _ {
        todo!("sub_box");
        // end;
    }
    // sub_mlist: cur_mlist:=info(p);
    else if math_type_p == math_type_kind::sub_mlist as _ {
        globals.cur_mlist = info_inner!(globals, p);
    }
    // othercases begin q:=new_null_box; goto found;
    else {
        todo!("othercases");
        // end
    }
    // endcases;@/
    // save_style:=cur_style; cur_style:=s; mlist_penalties:=false;@/
    save_style = globals.cur_style;
    globals.cur_style = s;
    globals.mlist_penalties = false;
    // mlist_to_hlist; q:=link(temp_head); {recursive call}
    /// recursive call
    mlist_to_hlist(globals)?;
    q = link!(globals, temp_head);
    // cur_style:=save_style; {restore the style}
    /// restore the style
    const _: () = ();
    globals.cur_style = save_style;
    // @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
    crate::section_0703::Set_up_the_values_of_cur_size_and_cur_mu__based_on_cur_style!(globals);
    // found: if is_char_node(q)or(q=null) then x:=hpack(q,natural)
    if is_char_node!(globals, q) || q == null {
        x = hpack(globals, q, natural0!(), natural1!())?;
    }
    // else if (link(q)=null)and(type(q)<=vlist_node)and(shift_amount(q)=0) then
    else if link!(globals, q) == null
        && r#type!(globals, q) <= vlist_node
        && shift_amount!(globals, q) == scaled::zero()
    {
        // x:=q {it's already clean}
        /// it's already clean
        const _: () = ();
        x = q;
    }
    // else x:=hpack(q,natural);
    else {
        x = hpack(globals, q, natural0!(), natural1!())?;
    }
    // @<Simplify a trivial box@>;
    crate::section_0721::Simplify_a_trivial_box!(globals, x);
    // clean_box:=x;
    crate::ok_nojump!(x)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0118::link;
use crate::section_0133::r#type;
use crate::section_0134::is_char_node;
use crate::section_0135::shift_amount;
use crate::section_0137::vlist_node;
use crate::section_0162::temp_head;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0649::hpack;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0686::new_noad;
use crate::section_0726::mlist_to_hlist;
