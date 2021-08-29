//! @ The recursive machinery is started by calling |show_box|.
//! @^recursion@>
//
// @p procedure show_box(@!p:pointer);
#[allow(unused_variables)]
pub(crate) fn show_box(globals: &mut TeXGlobals, p: pointer) {
    // begin @<Assign the values |depth_threshold:=show_box_depth| and
    //   |breadth_max:=show_box_breadth|@>;
    Assign_the_values_depth_threshold_from_show_box_depth_and_breadth_max_show_box_breadth!(
        globals
    );
    // if breadth_max<=0 then breadth_max:=5;
    if globals.breadth_max <= 0 {
        globals.breadth_max = 5;
    }
    // if pool_ptr+depth_threshold>=pool_size then
    if globals.pool_ptr.get() as integer + globals.depth_threshold as integer
        >= pool_size as integer
    {
        // depth_threshold:=pool_size-pool_ptr-1;
        globals.depth_threshold =
            (pool_size as integer - globals.pool_ptr.get() as integer - 1) as _;
    }
    // {now there's enough room for prefix string}
    /// now there's enough room for prefix string
    const _: () = ();
    // show_node_list(p); {the show starts at |p|}
    /// the show starts at `p`
    show_node_list(globals, p as _);
    // print_ln;
    print_ln(make_globals_io_string_log_view!(globals));
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0011::pool_size;
use crate::section_0057::print_ln;
use crate::section_0115::pointer;
use crate::section_0182::show_node_list;
