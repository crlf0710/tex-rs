//! ` ` When |delete_last| is called, |cur_chr| is the |type| of node that
//! will be deleted, if present.
//
// @<Declare action...@>=
// procedure delete_last;
pub(crate) fn delete_last(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit;
    // var @!p,@!q:pointer; {run through the current list}
    // @!m:quarterword; {the length of a replacement list}
    // begin if (mode=vmode)and(tail=head) then
    if mode!(globals) == vmode && tail!(globals) == head!(globals) {
        // @<Apologize for inability to do the operation now,
        //   unless \.{\\unskip} follows non-glue@>
        todo!("apologize");
    }
    // else  begin if not is_char_node(tail) then if type(tail)=cur_chr then
    else {
        if !is_char_node!(globals, tail!(globals))
            && r#type!(globals, tail!(globals)) as integer == globals.cur_chr.get() as integer
        {
            /// run through the current list
            let (mut p, mut q);
            // begin q:=head;
            q = head!(globals);
            // repeat p:=q;
            loop {
                p = q;
                // if not is_char_node(q) then if type(q)=disc_node then
                if !is_char_node!(globals, q) && r#type!(globals, q) == disc_node {
                    // begin for m:=1 to replace_count(q) do p:=link(p);
                    for _ in 1..=replace_count!(globals, q) {
                        p = link!(globals, p);
                    }
                    // if p=tail then return;
                    if p == tail!(globals) {
                        crate::return_nojump!();
                    }
                    // end;
                }
                // q:=link(p);
                q = link!(globals, p);
                // until q=tail;
                if q == tail!(globals) {
                    break;
                }
            }
            // link(p):=null; flush_node_list(tail); tail:=p;
            link!(globals, p) = null;
            flush_node_list(globals, tail!(globals))?;
            tail!(globals) = p;
            // end;
        }
        // end;
    }
    // exit:end;
    crate::ok_nojump!()
}

use crate::info::ok_nojump;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0118::link;
use crate::section_0133::r#type;
use crate::section_0134::is_char_node;
use crate::section_0145::disc_node;
use crate::section_0145::replace_count;
use crate::section_0202::flush_node_list;
use crate::section_0211::vmode;
use crate::section_0213::head;
use crate::section_0213::mode;
use crate::section_0213::tail;
