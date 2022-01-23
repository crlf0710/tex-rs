//! @ The calculations related to leaders require a bit of care. First, in the
//! case of |a_leaders| (aligned leaders), we want to move |cur_h| to
//! |left_edge| plus the smallest multiple of |leader_wd| for which the result
//! is not less than the current value of |cur_h|; i.e., |cur_h| should become
//! $|left_edge|+|leader_wd|\times\lceil
//! (|cur_h|-|left_edge|)/|leader_wd|\rceil$.  The program here should work in
//! all cases even though some implementations of \PASCAL\ give nonstandard
//! results for the |div| operation when |cur_h| is less than |left_edge|.
//!
//! In the case of |c_leaders| (centered leaders), we want to increase |cur_h|
//! by half of the excess space not occupied by the leaders; and in the
//! case of |x_leaders| (expanded leaders) we increase |cur_h|
//! by $1/(q+1)$ of this excess space, where $q$ is the number of times the
//! leader box will be replicated. Slight inaccuracies in the division might
//! accumulate; half of this rounding error is placed at each end of the leaders.
//
// @<Let |cur_h| be the position of the first box, ...@>=
pub(crate) macro Let_cur_h_be_the_position_of_the_first_box__and_set_leader_wd_plus_lx_to_the_spacing_between_corresponding_parts_of_boxes($globals:expr, $p:expr, $left_edge:expr, $leader_wd:expr, $lx:expr) {{
    // if subtype(p)=a_leaders then
    if subtype!($globals, $p) == glue_node_subtype::a_leaders as _ {
        /// what `dvi_h` should pop to
        let save_h;
        // begin save_h:=cur_h;
        save_h = $globals.cur_h;
        // cur_h:=left_edge+leader_wd*((cur_h-left_edge)@!div leader_wd);
        $globals.cur_h = $left_edge
            + scaled::new_from_inner(
                $leader_wd.inner()
                    * (($globals.cur_h.inner() - $left_edge.inner()) / $leader_wd.inner()),
            );
        // if cur_h<save_h then cur_h:=cur_h+leader_wd;
        if $globals.cur_h < save_h {
            $globals.cur_h += $leader_wd;
        }
        // end
    }
    // else  begin lq:=rule_wd div leader_wd; {the number of box copies}
    else {
        /// quantities used in calculations for leaders
        let (lq, lr);
        /// the number of box copies
        const _: () = ();
        lq = $globals.rule_wd.inner() / $leader_wd.inner();
        // lr:=rule_wd mod leader_wd; {the remaining space}
        /// the remaining space
        const _: () = ();
        lr = $globals.rule_wd.inner() % $leader_wd.inner();
        // if subtype(p)=c_leaders then cur_h:=cur_h+(lr div 2)
        if subtype!($globals, $p) == glue_node_subtype::c_leaders as _ {
            $globals.cur_h += scaled::new_from_inner(lr / 2);
        }
        // else  begin lx:=lr div (lq+1);
        else {
            $lx = scaled::new_from_inner(lr / (lq + 1));
            // cur_h:=cur_h+((lr-(lq-1)*lx) div 2);
            $globals.cur_h += scaled::new_from_inner((lr - (lq - 1) * $lx.inner()) / 2);
            // end;
        }
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0133::subtype;
    use crate::section_0149::glue_node_subtype;
}}
