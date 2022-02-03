//! ` `
//!
//! Paragraph shapes are set up in the obvious way.
//
// @<Assignments@>=
pub(crate) macro Assignments_1248($globals:expr, $cur_cmd:expr, $a:expr) {{
    // set_shape: begin scan_optional_equals; scan_int; n:=cur_val;
    let processed = if $cur_cmd == set_shape {
        /// for temporary short-term use
        let n;
        /// for temporary short-term use
        let p;
        scan_optional_equals($globals)?;
        scan_int($globals)?;
        n = $globals.cur_val;
        // if n<=0 then p:=null
        if n <= 0 {
            p = null;
        }
        // else  begin p:=get_node(2*n+1); info(p):=n;
        else {
            p = get_node($globals, 2 * n + 1)?;
            info_inner!($globals, p) = n as _;
            // for j:=1 to n do
            for j in 1..=n {
                // begin scan_normal_dimen;
                scan_normal_dimen!($globals)?;
                // mem[p+2*j-1].sc:=cur_val; {indentation}
                /// indentation
                const _: () = ();
                $globals.mem[p + 2 * j as pointer - 1][MEMORY_WORD_SC] =
                    scaled::new_from_inner($globals.cur_val);
                // scan_normal_dimen;
                scan_normal_dimen!($globals)?;
                // mem[p+2*j].sc:=cur_val; {width}
                /// width
                const _: () = ();
                $globals.mem[p + 2 * j as pointer][MEMORY_WORD_SC] =
                    scaled::new_from_inner($globals.cur_val);
                // end;
            }
            // end;
        }
        // define(par_shape_loc,shape_ref,p);
        define!($globals, $a, par_shape_loc, shape_ref, p);
        // end;
        true
    } else {
        false
    };
    use crate::section_0101::scaled;
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0125::get_node;
    use crate::section_0209::*;
    use crate::section_0210::shape_ref;
    use crate::section_0230::par_shape_loc;
    use crate::section_0405::scan_optional_equals;
    use crate::section_0440::scan_int;
    use crate::section_0448::scan_normal_dimen;
    use crate::section_1214::define;
    processed
}}
