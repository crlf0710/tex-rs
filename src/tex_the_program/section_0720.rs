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
pub(crate) fn clean_box(globals: &mut TeXGlobals, p: pointer, s: small_number) -> pointer {
    // label found;
    // var q:pointer; {beginning of a list to be boxed}
    // @!save_style:small_number; {|cur_style| to be restored}
    // @!x:pointer; {box to be returned}
    // @!r:pointer; {temporary pointer}
    // begin case math_type(p) of
    // math_char: begin cur_mlist:=new_noad; mem[nucleus(cur_mlist)]:=mem[p];
    //   end;
    // sub_box: begin q:=info(p); goto found;
    //   end;
    // sub_mlist: cur_mlist:=info(p);
    // othercases begin q:=new_null_box; goto found;
    //   end
    // endcases;@/
    // save_style:=cur_style; cur_style:=s; mlist_penalties:=false;@/
    // mlist_to_hlist; q:=link(temp_head); {recursive call}
    // cur_style:=save_style; {restore the style}
    // @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
    // found: if is_char_node(q)or(q=null) then x:=hpack(q,natural)
    //   else if (link(q)=null)and(type(q)<=vlist_node)and(shift_amount(q)=0) then
    //     x:=q {it's already clean}
    //   else x:=hpack(q,natural);
    // @<Simplify a trivial box@>;
    // clean_box:=x;
    todo!("clean_box");
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
