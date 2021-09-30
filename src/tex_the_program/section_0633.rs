//! ` `

// @<Output a rule in a vlist...@>=
pub(crate) macro Output_a_rule_in_a_vlist__goto_next_p($globals:expr, $this_box:expr, $lbl_next_p:lifetime) {{
    // if is_running(rule_wd) then rule_wd:=width(this_box);
    if is_running!($globals.rule_wd) {
        $globals.rule_wd = width!($globals, $this_box);
    }
    // rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
    /// this is the rule thickness
    const _: () = ();
    $globals.rule_ht += $globals.rule_dp;
    // cur_v:=cur_v+rule_ht;
    $globals.cur_v += $globals.rule_ht;
    // if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
    /// we don't output empty rules
    const _: () = ();
    if $globals.rule_ht > scaled::zero() && $globals.rule_wd > scaled::zero() {
        // begin synch_h; synch_v;
        synch_h!($globals);
        synch_v!($globals);
        // dvi_out(put_rule); dvi_four(rule_ht); dvi_four(rule_wd);
        dvi_out!($globals, put_rule.byte());
        dvi_four($globals, $globals.rule_ht.inner());
        dvi_four($globals, $globals.rule_wd.inner());
        // end;
    }
    // goto next_p
    crate::goto_forward_label!($lbl_next_p);
    use crate::section_0101::scaled;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0138::is_running;
    use crate::section_0586::put_rule;
    use crate::section_0598::dvi_out;
    use crate::section_0600::dvi_four;
    use crate::section_0616::synch_h;
    use crate::section_0616::synch_v;
}}
