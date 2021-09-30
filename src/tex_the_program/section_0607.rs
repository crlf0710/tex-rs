//! @ Here is a subroutine that produces a \.{DVI} command for some specified
//! downward or rightward motion. It has two parameters: |w| is the amount
//! of motion, and |o| is either |down1| or |right1|. We use the fact that
//! the command codes have convenient arithmetic properties: |y1-down1=w1-right1|
//! and |z1-down1=x1-right1|.
//
// @p procedure movement(@!w:scaled;@!o:eight_bits);
#[allow(unused_variables)]
pub(crate) fn movement(globals: &mut TeXGlobals, mut w: scaled, o: dvi_command) -> TeXResult<()> {
    // label exit,found,not_found,2,1;
    // var mstate:small_number; {have we seen a |y| or |z|?}
    /// have we seen a `y` or `z`?
    let mut mstate;
    // @!p,@!q:pointer; {current and top nodes on the stack}
    /// current and top nodes on the stack
    let (mut p, mut q): (pointer, pointer);
    // @!k:integer; {index into |dvi_buf|, modulo |dvi_buf_size|}
    crate::region_forward_label! {
    |'found|
    {
        // begin q:=get_node(movement_node_size); {new node for the top of the stack}
        /// new node for the top of the stack
        const _: () = ();
        q = get_node(globals, movement_node_size.into())?;
        // width(q):=w; location(q):=dvi_offset+dvi_ptr;
        width!(globals, q) = w;
        location!(globals, q) = globals.dvi_offset + globals.dvi_ptr.get() as integer;
        // if o=down1 then
        if o == down1 {
            // begin link(q):=down_ptr; down_ptr:=q;
            link!(globals, q) = globals.down_ptr;
            globals.down_ptr = q;
            // end
        }
        // else  begin link(q):=right_ptr; right_ptr:=q;
        else {
            link!(globals, q) = globals.right_ptr;
            globals.right_ptr = q;
            // end;
        }
        // @<Look at the other stack entries until deciding what sort of \.{DVI} command
        //   to generate; |goto found| if node |p| is a ``hit''@>;
        crate::section_0611::Look_at_the_other_stack_entries_until_deciding_what_sort_of_DVI_command_to_generate__goto_found_if_node_p_is_a_hit!(
            globals, p, q, w, mstate, 'found
        );
        // @<Generate a |down| or |right| command for |w| and |return|@>;
        crate::section_0610::Generate_a_down_or_right_command_for_w_and_return!(globals, q, w, o);
    }
    // found: @<Generate a |y0| or |z0| command in order to reuse a previous
    //   appearance of~|w|@>;
    'found <-
    };
    crate::section_0609::Generate_a_y0_or_z0_command_in_order_to_reuse_a_previous_appearance_of_w!(
        globals, p, q, o
    );
    // exit:end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0125::get_node;
use crate::section_0135::width;
use crate::section_0586::down1;
use crate::section_0586::dvi_command;
use crate::section_0605::location;
use crate::section_0605::movement_node_size;
