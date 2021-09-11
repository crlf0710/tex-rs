//! ` `
// @<Find an active node...@>=
pub(crate) macro Find_an_active_node_with_fewest_demerits($globals:expr) {{
    /// miscellaneous nodes of temporary interest
    let mut r: pointer;
    // r:=link(active); fewest_demerits:=awful_bad;
    r = link!($globals, active);
    $globals.fewest_demerits = awful_bad;
    // repeat if type(r)<>delta_node then if total_demerits(r)<fewest_demerits then
    loop {
        if r#type!($globals, r) != delta_node
            && total_demerits!($globals, r) < $globals.fewest_demerits
        {
            // begin fewest_demerits:=total_demerits(r); best_bet:=r;
            $globals.fewest_demerits = total_demerits!($globals, r);
            $globals.best_bet = r;
            // end;
        }
        // r:=link(r);
        r = link!($globals, r);
        // until r=last_active;
        if r == last_active!() {
            break;
        }
    }
    // best_line:=line_number(best_bet)
    $globals.best_line = line_number!($globals, $globals.best_bet);
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0162::active;
    use crate::section_0819::last_active;
    use crate::section_0819::line_number;
    use crate::section_0819::total_demerits;
    use crate::section_0822::delta_node;
    use crate::section_0833::awful_bad;
}}
