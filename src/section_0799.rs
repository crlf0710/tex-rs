//! @ At the end of a row, we append an unset box to the current vlist (for
//! \.{\\halign}) or the current hlist (for \.{\\valign}). This unset box
//! contains the unset boxes for the columns, separated by the tabskip glue.
//! Everything will be set later.
//
// @p procedure fin_row;
#[allow(unused_variables)]
pub(crate) fn fin_row(globals: &mut TeXGlobals) {
    todo!();
    // var p:pointer; {the new unset box}
    // begin if mode=-hmode then
    //   begin p:=hpack(link(head),natural);
    //   pop_nest; append_to_vlist(p);
    //   if cur_head<>cur_tail then
    //     begin link(tail):=link(cur_head); tail:=cur_tail;
    //     end;
    //   end
    // else  begin p:=vpack(link(head),natural); pop_nest;
    //   link(tail):=p; tail:=p; space_factor:=1000;
    //   end;
    // type(p):=unset_node; glue_stretch(p):=0;
    // if every_cr<>null then begin_token_list(every_cr,every_cr_text);
    // align_peek;
    // end; {note that |glue_shrink(p)=0| since |glue_shrink==shift_amount|}
}

use crate::section_0004::TeXGlobals;
