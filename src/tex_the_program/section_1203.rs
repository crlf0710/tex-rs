//! @ If the equation number is set on a line by itself, either before or
//! after the formula, we append an infinite penalty so that no page break will
//! separate the display from its number; and we use the same size and
//! displacement for all three potential lines of the display, even though
//! `\.{\\parshape}' may specify them differently.
//
// @<Append the glue or equation number preceding the display@>=
pub(crate) macro Append_the_glue_or_equation_number_preceding_the_display($globals:expr, $d:expr, $s:expr, $l:expr, $e:expr, $g1:expr, $g2:expr) {{
    // tail_append(new_penalty(pre_display_penalty));@/
    tail_append!(
        $globals,
        new_penalty($globals, pre_display_penalty!($globals))?
    );
    // if (d+s<=pre_display_size)or l then {not enough clearance}
    if $d + $s <= pre_display_size!($globals) || $l {
        /// not enough clearance
        const _: () = ();
        // begin g1:=above_display_skip_code; g2:=below_display_skip_code;
        $g1 = above_display_skip_code;
        $g2 = below_display_skip_code;
        // end
    }
    // else  begin g1:=above_display_short_skip_code;
    else {
        $g1 = above_display_short_skip_code;
        // g2:=below_display_short_skip_code;
        $g2 = below_display_short_skip_code;
        // end;
    }
    // if l and(e=0) then {it follows that |type(a)=hlist_node|}
    if $l && $e == scaled::zero() {
        /// it follows that `type(a)=hlist_node`
        const _ : () = ();
        // begin shift_amount(a):=s; append_to_vlist(a);
        // tail_append(new_penalty(inf_penalty));
        // end
        todo!("l and e == 0");
    }
    // else tail_append(new_param_glue(g1))
    else {
        tail_append!($globals, new_param_glue($globals, $g1.into())?);
    }
    use crate::section_0101::scaled;
    use crate::section_0152::new_param_glue;
    use crate::section_0158::new_penalty;
    use crate::section_0214::tail_append;
    use crate::section_0224::above_display_short_skip_code;
    use crate::section_0224::below_display_short_skip_code;
    use crate::section_0224::above_display_skip_code;
    use crate::section_0224::below_display_skip_code;
    use crate::section_0236::pre_display_penalty;
    use crate::section_0247::pre_display_size;
}}
