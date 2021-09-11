//! @ The processing of boxes is somewhat different, because we may need
//! to scan and create an entire box before we actually change the value of the old
//! one.
//
// @<Assignments@>=
pub(crate) macro Assignments_1241($globals:expr, $cur_cmd:expr, $a:expr) {{
    // set_box: begin scan_eight_bit_int;
    let processed = if $cur_cmd == set_box {
        /// for temporary short-term use
        let n: integer;

        scan_eight_bit_int($globals)?;
        // if global then n:=256+cur_val@+else n:=cur_val;
        if global!($a) {
            n = 256 + $globals.cur_val;
        } else {
            n = $globals.cur_val;
        }
        // scan_optional_equals;
        scan_optional_equals($globals)?;
        // if set_box_allowed then scan_box(box_flag+n)
        if $globals.set_box_allowed {
            scan_box($globals, box_flag + n)?;
        }
        // else begin print_err("Improper "); print_esc("setbox");
        else {
            print_err!($globals, crate::strpool_str!("Improper "));
            print_esc($globals, crate::strpool_str!("setbox"));
            // @.Improper \\setbox@>
            //   help2("Sorry, \setbox is not allowed after \halign in a display,")@/
            //   ("or between \accent and an accented character."); error;
            //   end;
            // end;
            todo!();
        }
        true
    } else {
        false
    };
    use crate::pascal::integer;
    use crate::section_0063::print_esc;
    use crate::section_0073::print_err;
    use crate::section_0209::*;
    use crate::section_0405::scan_optional_equals;
    use crate::section_0433::scan_eight_bit_int;
    use crate::section_1071::box_flag;
    use crate::section_1084::scan_box;
    use crate::section_1214::global;
    processed
}}
