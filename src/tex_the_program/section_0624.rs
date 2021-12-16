//! ` `

// @<Output a rule in an hlist@>=
pub(crate) macro Output_a_rule_in_an_hlist($globals:expr, $this_box:expr, $base_line:expr) {{
    // if is_running(rule_ht) then rule_ht:=height(this_box);
    if is_running!($globals.rule_ht) {
        $globals.rule_ht = height!($globals, $this_box);
    }
    // if is_running(rule_dp) then rule_dp:=depth(this_box);
    if is_running!($globals.rule_dp) {
        $globals.rule_dp = depth!($globals, $this_box);
    }
    // rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
    /// this is the rule thickness
    const _: () = ();
    $globals.rule_ht += $globals.rule_dp;
    // if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
    /// we don't output empty rules
    const _: () = ();
    if $globals.rule_ht > scaled::zero() && $globals.rule_wd > scaled::zero() {
        // begin synch_h; cur_v:=base_line+rule_dp; synch_v;
        synch_h!($globals);
        $globals.cur_v = $base_line + $globals.rule_dp;
        synch_v!($globals);
        // dvi_out(set_rule); dvi_four(rule_ht); dvi_four(rule_wd);
        dvi_out!($globals, put_rule.byte());
        dvi_four($globals, $globals.rule_ht.inner());
        dvi_four($globals, $globals.rule_wd.inner());
        // cur_v:=base_line; dvi_h:=dvi_h+rule_wd;
        $globals.cur_v = $base_line;
        $globals.dvi_h += $globals.rule_wd;
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0138::is_running;
    use crate::section_0586::put_rule;
    use crate::section_0598::dvi_out;
    use crate::section_0600::dvi_four;
    use crate::section_0616::synch_h;
    use crate::section_0616::synch_v;
}}
