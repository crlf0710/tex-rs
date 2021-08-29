//! @ Now that we can see what eventually happens to boxes, we can consider
//! the first steps in their creation. The |begin_box| routine is called when
//! |box_context| is a context specification, |cur_chr| specifies the type of
//! box desired, and |cur_cmd=make_box|.
//
// @<Declare act...@>=
// procedure begin_box(@!box_context:integer);
#[allow(unused_variables)]
pub(crate) fn begin_box(globals: &mut TeXGlobals, box_context: integer) -> TeXResult<()> {
    // label exit, done;
    // var @!p,@!q:pointer; {run through the current list}
    // @!m:quarterword; {the length of a replacement list}
    // @!k:halfword; {0 or |vmode| or |hmode|}
    // @!n:eight_bits; {a box number}
    let cur_chr = globals.cur_chr.get();
    // begin case cur_chr of
    if false {
        unreachable!()
    }
    // box_code: begin scan_eight_bit_int; cur_box:=box(cur_val);
    else if cur_chr == box_code as chr_code_repr {
        todo!("box_code");
        // box(cur_val):=null; {the box becomes void, at the same level}
        // end;
    }
    // copy_code: begin scan_eight_bit_int; cur_box:=copy_node_list(box(cur_val));
    else if cur_chr == copy_code as chr_code_repr {
        todo!("copy_code");
        // end;
    }
    // last_box_code: @<If the current list ends with a box node, delete it from
    //   the list and make |cur_box| point to it; otherwise set |cur_box:=null|@>;
    else if cur_chr == last_box_code as chr_code_repr {
        todo!("last_box_code");
    }
    // vsplit_code: @<Split off part of a vertical box, make |cur_box| point to it@>;
    else if cur_chr == vsplit_code as chr_code_repr {
        todo!("vsplit_code");
    }
    // othercases @<Initiate the construction of an hbox or vbox, then |return|@>
    else {
        Initiate_the_construction_of_an_hbox_or_vbox_then_return!(globals, box_context);
    }
    // endcases;@/
    // box_end(box_context); {in simple cases, we use the box immediately}
    todo!("box_end");
    // exit:end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0297::chr_code_repr;
use crate::section_1071::*;
