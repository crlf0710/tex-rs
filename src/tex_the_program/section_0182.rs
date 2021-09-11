//! @ Now we are ready for |show_node_list| itself. This procedure has been
//! written to be ``extra robust'' in the sense that it should not crash or get
//! into a loop even if the data structures have been messed up by bugs in
//! the rest of the program. You can safely call its parent routine
//! |show_box(p)| for arbitrary values of |p| when you are debugging \TeX.
//! However, in the presence of bad data, the procedure may
//! @^dirty \PASCAL@>@^debugging@>
//! fetch a |memory_word| whose variant is different from the way it was stored;
//! for example, it might try to read |mem[p].hh| when |mem[p]|
//! contains a scaled integer, if |p| is a pointer that has been
//! clobbered or chosen at random.
//
// @p procedure show_node_list(@!p:integer); {prints a node list symbolically}
/// prints a node list symbolically
#[allow(unused_variables)]
pub(crate) fn show_node_list(globals: &mut TeXGlobals, mut p: integer) {
    // label exit;
    // var n:integer; {the number of items already printed at this level}
    /// the number of items already printed at this level
    let mut n: integer;
    // @!g:real; {a glue ratio, as a floating point number}
    // begin if cur_length>depth_threshold then
    if cur_length!(globals) as integer > globals.depth_threshold {
        todo!(" trunc");
        // begin if p>null then print(" []");
        //   {indicate that there's been some truncation}
        // return;
        // end;
    }
    // n:=0;
    n = 0;
    // while p>mem_min do
    while p > mem_min as integer {
        // begin print_ln; print_current_string; {display the nesting history}
        /// display the nesting history
        const _: () = ();
        print_ln(make_globals_io_string_log_view!(globals));
        print_current_string(globals);
        // if p>mem_end then {pointer out of range}
        if p > globals.mem_end as integer {
            /// pointer out of range
            const _: () = ();
            // begin print("Bad link, display aborted."); return;
            // @.Bad link...@>
            // end;
        }
        // incr(n); if n>breadth_max then {time to stop}
        incr!(n);
        if n > globals.breadth_max {
            /// time to stop
            const _: () = ();
            // begin print("etc."); return;
            print(globals, crate::strpool_str!("etc.").get() as _);
            return;
            // @.etc@>
            // end;
        }
        // @<Display node |p|@>;
        crate::section_0183::Display_node_p!(globals, p);
        // p:=link(p);
        p = link!(globals, p as pointer) as _;
        // end;
    }
    // exit:
    // end;
}

use crate::pascal::integer;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0011::mem_min;
use crate::section_0016::incr;
use crate::section_0041::cur_length;
use crate::section_0057::print_ln;
use crate::section_0059::print;
use crate::section_0070::print_current_string;
use crate::section_0115::pointer;
use crate::section_0118::link;
