//! ` `

// @<Assignments@>=
pub(crate) macro Assignments_1252($globals:expr, $cur_cmd:expr, $a:expr, $lbl_done:lifetime) {{
    // hyph_data: if cur_chr=1 then
    let processed = if $cur_cmd == hyph_data {
        if $globals.cur_chr.get() == 1 {
            // begin @!init new_patterns; goto done;@;@+tini@/
            crate::region_initex! {
                new_patterns($globals)?;
                crate::goto_forward_label!($lbl_done);
            }
            todo!("hyph data error");
            // print_err("Patterns can be loaded only by INITEX");
            // @.Patterns can be...@>
            // help0; error;
            // repeat get_token; until cur_cmd=right_brace; {flush the patterns}
            // return;
            // end
        }
        // else  begin new_hyph_exceptions; goto done;
        else {
            new_hyph_exceptions($globals)?;
            crate::goto_forward_label!($lbl_done);
            // end;
        }
        use crate::section_0934::new_hyph_exceptions;
        use crate::section_0960::new_patterns;
        true
    } else {
        false
    };
    use crate::section_0209::*;
    processed
}}
