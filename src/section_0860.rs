//! @ When an active node disappears, we must delete an adjacent delta node if the
//! active node was at the beginning or the end of the active list, or if it
//! was surrounded by delta nodes. We also must preserve the property that
//! |cur_active_width| represents the length of material from |link(prev_r)|
//! to~|cur_p|.
//
// @d combine_two_deltas(#)==@|mem[prev_r+#].sc:=mem[prev_r+#].sc+mem[r+#].sc
// @d downdate_width(#)==@|cur_active_width[#]:=cur_active_width[#]-
//   mem[prev_r+#].sc
//
// @<Deactivate node |r|@>=
macro_rules! Deactivate_node_r {
    ($globals:expr, $r:expr, $prev_r:expr) => {{
        // link(prev_r):=link(r); free_node(r,active_node_size);
        link!($globals, $prev_r) = link!($globals, $r);
        free_node($globals, $r, active_node_size as _);
        // if prev_r=active then @<Update the active widths, since the first active
        //   node has been deleted@>
        if $prev_r == active {
            Update_the_active_widths__since_the_first_active_node_has_been_deleted!($globals, $r);
        }
        // else if type(prev_r)=delta_node then
        else if r#type!($globals, $prev_r) == delta_node {
            todo!("process when prev_r is delta node");
            // begin r:=link(prev_r);
            // if r=last_active then
            //   begin do_all_six(downdate_width);
            //   link(prev_prev_r):=last_active;
            //   free_node(prev_r,delta_node_size); prev_r:=prev_prev_r;
            //   end
            // else if type(r)=delta_node then
            //   begin do_all_six(update_width);
            //   do_all_six(combine_two_deltas);
            //   link(prev_r):=link(r); free_node(r,delta_node_size);
            //   end;
            // end
        }
        use crate::section_0130::free_node;
        use crate::section_0819::active_node_size;
        use crate::section_0822::delta_node;
    }}
}
