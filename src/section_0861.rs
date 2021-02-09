//! @ The following code uses the fact that |type(last_active)<>delta_node|. If the
//! active list has just become empty, we do not need to update the
//! |active_width| array, since it will be initialized when an active
//! node is next inserted.
//
// @d update_active(#)==active_width[#]:=active_width[#]+mem[r+#].sc
macro_rules! update_active {
    ($globals:expr, $idx:expr, $r:expr) => {{
        $globals.active_width[$idx] = $globals.active_width[$idx]
            + $globals.mem[$r + $idx][crate::section_0101::MEMORY_WORD_SC]
    }};
}

// @<Update the active widths,...@>=
macro_rules! Update_the_active_widths__since_the_first_active_node_has_been_deleted {
    ($globals:expr, $r:expr) => {{
        // begin r:=link(active);
        $r = link!($globals, active);
        // if type(r)=delta_node then
        if r#type!($globals, $r) == delta_node {
            // begin do_all_six(update_active);
            do_all_six!(update_active !; @globals = $globals; $r);
            // do_all_six(copy_to_cur_active);
            do_all_six!(copy_to_cur_active !; @globals = $globals);
            // link(active):=link(r); free_node(r,delta_node_size);
            link!($globals, active) = link!($globals, $r);
            free_node($globals, $r, delta_node_size as _);
            // end;
        }
        // end
        use crate::section_0822::delta_node_size;
    }}
}
