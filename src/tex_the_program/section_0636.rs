//! ` `

// @<Let |cur_v| be the position of the first box, ...@>=
pub(crate) macro Let_cur_v_be_the_position_of_the_first_box__and_set_leader_ht_plus_lx_to_the_spacing_between_corresponding_parts_of_boxes($globals:expr, $p:expr, $top_edge:expr, $lx:expr, $leader_ht:expr) {{
    // if subtype(p)=a_leaders then
    if subtype!($globals, $p) == glue_node_subtype::a_leaders as _ {
        /// what `dvi_h` and `dvi_v` should pop to
        let save_v;
        // begin save_v:=cur_v;
        save_v = $globals.cur_v;
        // cur_v:=top_edge+leader_ht*((cur_v-top_edge)@!div leader_ht);
        $globals.cur_v = $top_edge
            + scaled::new_from_inner(
                $leader_ht.inner() * (($globals.cur_v - $top_edge).inner() / $leader_ht.inner()),
            );
        // if cur_v<save_v then cur_v:=cur_v+leader_ht;
        if $globals.cur_v < save_v {
            $globals.cur_v = $globals.cur_v + $leader_ht;
        }
        // end
    }
    // else  begin lq:=rule_ht div leader_ht; {the number of box copies}
    else {
        /// quantities used in calculations for leaders
        let (lq, lr);
        /// the number of box copies
        const _: () = ();
        lq = $globals.rule_ht.inner() / $leader_ht.inner();
        // lr:=rule_ht mod leader_ht; {the remaining space}
        /// the remaining space
        const _: () = ();
        lr = $globals.rule_ht.inner() % $leader_ht.inner();
        // if subtype(p)=c_leaders then cur_v:=cur_v+(lr div 2)
        if subtype!($globals, $p) == glue_node_subtype::c_leaders as _ {
            $globals.cur_v = $globals.cur_v + scaled::new_from_inner(lr / 2);
        }
        // else  begin lx:=lr div (lq+1);
        else {
            $lx = scaled::new_from_inner(lr / (lq + 1));
            // cur_v:=cur_v+((lr-(lq-1)*lx) div 2);
            $globals.cur_v =
                $globals.cur_v + scaled::new_from_inner((lr - (lq - 1) * $lx.inner()) / 2);
            // end;
        }
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0133::subtype;
    use crate::section_0149::glue_node_subtype;
}}
