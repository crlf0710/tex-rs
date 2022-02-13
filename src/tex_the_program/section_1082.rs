//! @ Here we deal with things like `\.{\\vsplit 13 to 100pt}'.
//
// @<Split off part of a vertical box, make |cur_box| point to it@>=
pub(crate) macro Split_off_part_of_a_vertical_box__make_cur_box_point_to_it($globals:expr) {{
    /// a box number
    let n;
    // begin scan_eight_bit_int; n:=cur_val;
    scan_eight_bit_int($globals)?;
    n = $globals.cur_val;
    // if not scan_keyword("to") then
    if !scan_keyword($globals, crate::strpool_str!("to"))? {
        // @.to@>
        // begin print_err("Missing `to' inserted");
        // @.Missing `to' inserted@>
        // help2("I'm working on `\vsplit<box number> to <dimen>';")@/
        // ("will look for the <dimen> next."); error;
        // end;
        todo!("to");
    }
    // scan_normal_dimen;
    scan_normal_dimen!($globals)?;
    // cur_box:=vsplit(n,cur_val);
    $globals.cur_box = vsplit($globals, n as _, scaled::new_from_inner($globals.cur_val))?;
    // end
    use crate::section_0101::scaled;
    use crate::section_0407::scan_keyword;
    use crate::section_0433::scan_eight_bit_int;
    use crate::section_0448::scan_normal_dimen;
    use crate::section_0977::vsplit;
}}
