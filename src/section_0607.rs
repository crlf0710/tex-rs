//! @ Here is a subroutine that produces a \.{DVI} command for some specified
//! downward or rightward motion. It has two parameters: |w| is the amount
//! of motion, and |o| is either |down1| or |right1|. We use the fact that
//! the command codes have convenient arithmetic properties: |y1-down1=w1-right1|
//! and |z1-down1=x1-right1|.
//
// @p procedure movement(@!w:scaled;@!o:eight_bits);
#[allow(unused_variables)]
pub(crate) fn movement(globals: &mut TeXGlobals, w: scaled, o: dvi_command) {
    // label exit,found,not_found,2,1;
    // var mstate:small_number; {have we seen a |y| or |z|?}
    // @!p,@!q:pointer; {current and top nodes on the stack}
    // @!k:integer; {index into |dvi_buf|, modulo |dvi_buf_size|}
    // begin q:=get_node(movement_node_size); {new node for the top of the stack}
    // width(q):=w; location(q):=dvi_offset+dvi_ptr;
    // if o=down1 then
    //   begin link(q):=down_ptr; down_ptr:=q;
    //   end
    // else  begin link(q):=right_ptr; right_ptr:=q;
    //   end;
    // @<Look at the other stack entries until deciding what sort of \.{DVI} command
    //   to generate; |goto found| if node |p| is a ``hit''@>;
    // @<Generate a |down| or |right| command for |w| and |return|@>;
    // found: @<Generate a |y0| or |z0| command in order to reuse a previous
    //   appearance of~|w|@>;
    // exit:end;
    todo!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
use crate::section_0586::dvi_command;