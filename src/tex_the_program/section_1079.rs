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
        scan_eight_bit_int(globals)?;
        globals.cur_box = r#box!(globals, globals.cur_val);
        // box(cur_val):=null; {the box becomes void, at the same level}
        /// the box becomes void, at the same level
        const _: () = ();
        r#box!(globals, globals.cur_val) = null;
        // end;
    }
    // copy_code: begin scan_eight_bit_int; cur_box:=copy_node_list(box(cur_val));
    else if cur_chr == copy_code as chr_code_repr {
        scan_eight_bit_int(globals)?;
        globals.cur_box = copy_node_list(globals, r#box!(globals, globals.cur_val))?;
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
        crate::section_1083::Initiate_the_construction_of_an_hbox_or_vbox_then_return!(
            globals,
            box_context
        );
    }
    // endcases;@/
    // box_end(box_context); {in simple cases, we use the box immediately}
    /// in simple cases, we use the box immediately
    box_end(globals, box_context)?;
    // exit:end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0204::copy_node_list;
use crate::section_0230::r#box;
use crate::section_0297::chr_code_repr;
use crate::section_0433::scan_eight_bit_int;
use crate::section_1071::*;
use crate::section_1075::box_end;
