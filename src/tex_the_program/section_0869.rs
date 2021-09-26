//! ` `
// The following code knows that discretionary texts contain
// only character nodes, kern nodes, box nodes, rule nodes, and ligature nodes.
//
// @<Try to break after a discretionary fragment...@>=
pub(crate) macro Try_to_break_after_a_discretionary_fragment__then_goto_done5($globals:expr, $prev_p:expr, $lbl_done5:lifetime) {{
    /// miscellaneous nodes of temporary interest
    let (r, mut s);
    // begin s:=pre_break(cur_p); disc_width:=0;
    s = pre_break!($globals, $globals.cur_p);
    $globals.disc_width = scaled::zero();
    // if s=null then try_break(ex_hyphen_penalty,hyphenated)
    if s == null {
        try_break($globals, ex_hyphen_penalty!($globals), hyphenated.into())?;
    }
    // else  begin repeat @<Add the width of node |s| to |disc_width|@>;
    else {
        loop {
            crate::section_0870::Add_the_width_of_node_s_to_disc_width!($globals, s);
            // s:=link(s);
            s = link!($globals, s);
            // until s=null;
            if s == null {
                break;
            }
        }
        // act_width:=act_width+disc_width;
        act_width!($globals) += $globals.disc_width;
        // try_break(hyphen_penalty,hyphenated);
        try_break($globals, hyphen_penalty!($globals), hyphenated.into())?;
        // act_width:=act_width-disc_width;
        act_width!($globals) -= $globals.disc_width;
        // end;
    }
    // r:=replace_count(cur_p); s:=link(cur_p);
    r = replace_count!($globals, $globals.cur_p);
    s = link!($globals, $globals.cur_p);
    // while r>0 do
    while r > 0 {
        // begin @<Add the width of node |s| to |act_width|@>;
        // decr(r); s:=link(s);
        // end;
        todo!("r > 0");
    }
    // prev_p:=cur_p; cur_p:=s; goto done5;
    $prev_p = $globals.cur_p;
    $globals.cur_p = s;
    crate::goto_forward_label!($lbl_done5);
    // end
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0135::width;
    use crate::section_0145::pre_break;
    use crate::section_0145::replace_count;
    use crate::section_0236::ex_hyphen_penalty;
    use crate::section_0236::hyphen_penalty;
    use crate::section_0819::hyphenated;
    use crate::section_0829::try_break;
    use crate::section_0866::act_width;
}}
