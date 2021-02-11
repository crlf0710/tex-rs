//! @ We use the fact that |type(active)<>delta_node|.
//
// @d convert_to_break_width(#)==@|
//   mem[prev_r+#].sc:=@|@t\hskip10pt@>mem[prev_r+#].sc
//   -cur_active_width[#]+break_width[#]
macro_rules! convert_to_break_width {
    ($globals:expr, $idx:expr, $prev_r:expr) => {{
        $globals.mem[$prev_r + $idx][crate::section_0101::MEMORY_WORD_SC] +=
            -$globals.cur_active_width[$idx] + $globals.break_width[$idx];
    }};
}
// @d store_break_width(#)==active_width[#]:=break_width[#]
macro_rules! store_break_width {
    ($globals:expr, $idx:expr) => {{
        $globals.active_width[$idx] = $globals.break_width[$idx];
    }};
}
// @d new_delta_to_break_width(#)==@|
//   mem[q+#].sc:=break_width[#]-cur_active_width[#]
macro_rules! new_delta_to_break_width {
    ($globals:expr, $idx:expr, $q:expr) => {{
        $globals.mem[$q + $idx][crate::section_0101::MEMORY_WORD_SC] = $globals.break_width[$idx]
            - $globals.cur_active_width[$idx];
    }}
}

// @<Insert a delta node to prepare for breaks at |cur_p|@>=
macro_rules! Insert_a_delta_node_to_prepare_for_breaks_at_cur_p {
    ($globals:expr, $r:expr, $prev_r:expr, $prev_prev_r:expr) => {{
        // if type(prev_r)=delta_node then {modify an existing delta node}
        if r#type!($globals, $prev_r) == delta_node {
            /// modify an existing delta node
            const _ : () = ();
            // begin do_all_six(convert_to_break_width);
            do_all_six!(convert_to_break_width !; @globals = $globals; $prev_r);
            // end
        }
        // else if prev_r=active then {no delta node needed at the beginning}
        else if $prev_r == active {
            /// no delta node needed at the beginning
            const _ : () = ();
            // begin do_all_six(store_break_width);
            do_all_six!(store_break_width !; @globals = $globals);
            // end
        }
        // else  begin q:=get_node(delta_node_size); link(q):=r; type(q):=delta_node;@/
        else {
            /// points to a new node being created
            let q: pointer;

            q = get_node($globals, delta_node_size.into())?;
            link!($globals, q) = $r;
            r#type!($globals, q) = delta_node;
            // subtype(q):=0; {the |subtype| is not used}
            /// the `subtype` is not used
            const _: () = ();
            subtype!($globals, q) = 0;
            // do_all_six(new_delta_to_break_width);
            do_all_six!(new_delta_to_break_width !; @globals = $globals; q);
            // link(prev_r):=q; prev_prev_r:=prev_r; prev_r:=q;
            link!($globals, $prev_r) = q;
            $prev_prev_r = $prev_r;
            $prev_r = q;
            // end
        }
        use crate::section_0125::get_node;
        use crate::section_0822::delta_node;
        use crate::section_0822::delta_node_size;
    }}
}
