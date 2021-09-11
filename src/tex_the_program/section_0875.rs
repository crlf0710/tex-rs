//! @ The adjustment for a desired looseness is a slightly more complicated
//! version of the loop just considered. Note that if a paragraph is broken
//! into segments by displayed equations, each segment will be subject to the
//! looseness calculation, independently of the other segments.
//
// @<Find the best active node...@>=
pub(crate) macro Find_the_best_active_node_for_the_desired_looseness($globals:expr) {{
    /// miscellaneous nodes of temporary interest
    let mut r: pointer;
    // begin r:=link(active); actual_looseness:=0;
    r = link!($globals, active);
    $globals.actual_looseness = 0;
    // repeat if type(r)<>delta_node then
    loop {
        if r#type!($globals, r) != delta_node {
            // begin line_diff:=line_number(r)-best_line;
            $globals.line_diff =
                line_number!($globals, r) as integer - $globals.best_line as integer;
            // if ((line_diff<actual_looseness)and(looseness<=line_diff))or@|
            // ((line_diff>actual_looseness)and(looseness>=line_diff)) then
            if ($globals.line_diff < $globals.actual_looseness
                && looseness!($globals) <= $globals.line_diff)
                || ($globals.line_diff > $globals.actual_looseness
                    && looseness!($globals) >= $globals.line_diff)
            {
                // begin best_bet:=r; actual_looseness:=line_diff;
                $globals.best_bet = r;
                $globals.actual_looseness = $globals.line_diff;
                // fewest_demerits:=total_demerits(r);
                $globals.fewest_demerits = total_demerits!($globals, r);
            // end
            }
            // else if (line_diff=actual_looseness)and@|
            //   (total_demerits(r)<fewest_demerits) then
            else if $globals.line_diff == $globals.actual_looseness
                && total_demerits!($globals, r) < $globals.fewest_demerits
            {
                // begin best_bet:=r; fewest_demerits:=total_demerits(r);
                $globals.best_bet = r;
                $globals.fewest_demerits = total_demerits!($globals, r);
                // end;
            }
            // end;
        }
        // r:=link(r);
        r = link!($globals, r);
        // until r=last_active;
        if r == last_active!() {
            break;
        }
    }
    // best_line:=line_number(best_bet);
    $globals.best_line = line_number!($globals, $globals.best_bet);
    // end
    use crate::pascal::integer;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0162::active;
    use crate::section_0236::looseness;
    use crate::section_0819::last_active;
    use crate::section_0819::line_number;
    use crate::section_0819::total_demerits;
    use crate::section_0822::delta_node;
}}
