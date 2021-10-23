//! @ At the end of a row, we append an unset box to the current vlist (for
//! \.{\\halign}) or the current hlist (for \.{\\valign}). This unset box
//! contains the unset boxes for the columns, separated by the tabskip glue.
//! Everything will be set later.
//
// @p procedure fin_row;
#[allow(unused_variables)]
pub(crate) fn fin_row(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var p:pointer; {the new unset box}
    /// the new unset box
    let p: pointer;
    // begin if mode=-hmode then
    if mode!(globals) == -hmode {
        // begin p:=hpack(link(head),natural);
        // pop_nest; append_to_vlist(p);
        // if cur_head<>cur_tail then
        //   begin link(tail):=link(cur_head); tail:=cur_tail;
        //   end;
        // end
        todo!("-hmode");
    }
    // else  begin p:=vpack(link(head),natural); pop_nest;
    else {
        p = vpack(
            globals,
            link!(globals, head!(globals)),
            natural0!(),
            natural1!(),
        )?;
        pop_nest(globals);
        // link(tail):=p; tail:=p; space_factor:=1000;
        link!(globals, tail!(globals)) = p;
        tail!(globals) = p;
        space_factor!(globals) = 1000;
        // end;
    }
    // type(p):=unset_node; glue_stretch(p):=0;
    r#type!(globals, p) = unset_node;
    glue_stretch!(globals, p) = scaled::zero();
    // if every_cr<>null then begin_token_list(every_cr,every_cr_text);
    if every_cr!(globals) != null {
        begin_token_list(globals, every_cr!(globals), every_cr_text);
    }
    // align_peek;
    align_peek(globals)?;
    // end; {note that |glue_shrink(p)=0| since |glue_shrink==shift_amount|}
    /// note that `glue_shrink(p)=0` since `glue_shrink==shift_amount`
    const _: () = ();
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0133::r#type;
use crate::section_0159::glue_stretch;
use crate::section_0159::unset_node;
use crate::section_0211::hmode;
use crate::section_0213::head;
use crate::section_0213::mode;
use crate::section_0213::space_factor;
use crate::section_0213::tail;
use crate::section_0217::pop_nest;
use crate::section_0230::every_cr;
use crate::section_0307::every_cr_text;
use crate::section_0323::begin_token_list;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0668::vpack;
use crate::section_0785::align_peek;
