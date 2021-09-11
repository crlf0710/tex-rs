//! @ When an active node disappears, we must delete an adjacent delta node if the
//! active node was at the beginning or the end of the active list, or if it
//! was surrounded by delta nodes. We also must preserve the property that
//! |cur_active_width| represents the length of material from |link(prev_r)|
//! to~|cur_p|.
//
// @d combine_two_deltas(#)==@|mem[prev_r+#].sc:=mem[prev_r+#].sc+mem[r+#].sc
pub(crate) macro combine_two_deltas($globals:expr, $idx:expr, $r:expr, $prev_r:expr) {{
    let v = $globals.mem[$r + $idx][MEMORY_WORD_SC];
    $globals.mem[$prev_r + $idx][MEMORY_WORD_SC] += v;
    use crate::section_0101::MEMORY_WORD_SC;
}}
// @d downdate_width(#)==@|cur_active_width[#]:=cur_active_width[#]-
//   mem[prev_r+#].sc
pub(crate) macro downdate_width($globals:expr, $idx:expr, $prev_r:expr) {{
    $globals.cur_active_width[$idx] = $globals.cur_active_width[$idx]
        - $globals.mem[$prev_r + $idx][crate::section_0101::MEMORY_WORD_SC]
}}

// @<Deactivate node |r|@>=
pub(crate) macro Deactivate_node_r
    ($globals:expr, $r:expr, $prev_r:expr, $prev_prev_r:expr) {{
        crate::trace_span!("Deactivate node `r`");
        // link(prev_r):=link(r); free_node(r,active_node_size);
        link!($globals, $prev_r) = link!($globals, $r);
        free_node($globals, $r, active_node_size as _);
        // if prev_r=active then @<Update the active widths, since the first active
        //   node has been deleted@>
        if $prev_r == active {
            crate::section_0861::Update_the_active_widths__since_the_first_active_node_has_been_deleted!($globals, $r);
        }
        // else if type(prev_r)=delta_node then
        else if r#type!($globals, $prev_r) == delta_node {
            // begin r:=link(prev_r);
            $r = link!($globals, $prev_r);
            // if r=last_active then
            if $r == last_active!() {
                // begin do_all_six(downdate_width);
                do_all_six!(downdate_width !; @globals = $globals; $prev_r);
                // link(prev_prev_r):=last_active;
                link!($globals, $prev_prev_r) = last_active!();
                // free_node(prev_r,delta_node_size); prev_r:=prev_prev_r;
                free_node($globals, $prev_r, delta_node_size as _);
                $prev_r = $prev_prev_r;
                // end
            }
            // else if type(r)=delta_node then
            else if r#type!($globals, $r) == delta_node {
                // begin do_all_six(update_width);
                do_all_six!(update_width !; @globals = $globals; $r);
                // do_all_six(combine_two_deltas);
                do_all_six!(combine_two_deltas !; @globals = $globals; $r, $prev_r);
                // link(prev_r):=link(r); free_node(r,delta_node_size);
                link!($globals, $prev_r) = link!($globals, $r);
                free_node($globals, $r, delta_node_size.into());
                // end;
            }
            // end
        }
        use crate::section_0130::free_node;
        use crate::section_0162::active;
        use crate::section_0819::active_node_size;
        use crate::section_0822::delta_node;
        use crate::section_0822::delta_node_size;
        use crate::section_0118::link;
        use crate::section_0133::r#type;
        use crate::section_0832::update_width;
        use crate::section_0819::last_active;
        use crate::section_0823::do_all_six;
    }}
