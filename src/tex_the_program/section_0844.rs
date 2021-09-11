//! @ When the following code is performed, we will have just inserted at
//! least one active node before |r|, so |type(prev_r)<>delta_node|.
//
// @d new_delta_from_break_width(#)==@|mem[q+#].sc:=
//     cur_active_width[#]-break_width[#]
pub(crate) macro new_delta_from_break_width($globals:expr, $idx:expr, $q:expr) {{
    $globals.mem[$q + $idx][crate::section_0101::MEMORY_WORD_SC] =
        $globals.cur_active_width[$idx] - $globals.break_width[$idx];
}}
//
// @<Insert a delta node to prepare for the next active node@>=
pub(crate) macro Insert_a_delta_node_to_prepare_for_the_next_active_node
    ($globals:expr, $r:expr, $prev_r:expr, $prev_prev_r:expr) {{
        // if r<>last_active then
        if $r != last_active!() {
            /// points to a new node being created
            let q: pointer;
            // begin q:=get_node(delta_node_size); link(q):=r; type(q):=delta_node;@/
            q = get_node($globals, delta_node_size as _)?;
            link!($globals, q) = $r;
            r#type!($globals, q) = delta_node;
            // subtype(q):=0; {the |subtype| is not used}
            /// the `subtype` is not used
            const _ : () = ();
            subtype!($globals, q) = 0;
            // do_all_six(new_delta_from_break_width);
            do_all_six!(new_delta_from_break_width !; @globals = $globals; q);
            // link(prev_r):=q; prev_prev_r:=prev_r; prev_r:=q;
            link!($globals, $prev_r) = q;
            $prev_prev_r = $prev_r;
            $prev_r = q;
            // end
        }
        use crate::section_0115::pointer;
        use crate::section_0118::link;
        use crate::section_0125::get_node;
        use crate::section_0133::subtype;
        use crate::section_0819::last_active;
        use crate::section_0822::delta_node;
        use crate::section_0133::r#type;
        use crate::section_0822::delta_node_size;
        use crate::section_0823::do_all_six;
    }}
