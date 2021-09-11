//! ` `
// @<If the preamble list has been traversed, check that the row has ended@>=
pub(crate) macro If_the_preamble_list_has_been_traversed__check_that_the_row_has_ended($globals:expr, $p:expr) {{
    // if (p=null)and(extra_info(cur_align)<cr_code) then
    if $p == null && extra_info!($globals, $globals.cur_align) < cr_code {
        // if cur_loop<>null then @<Lengthen the preamble periodically@>
        if $globals.cur_loop != null {
            todo!("Lengthen the preamble periodically");
        }
        // else  begin print_err("Extra alignment tab has been changed to ");
        else {
            print_err!(
                $globals,
                crate::strpool_str!("Extra alignment tab has been changed to ")
            );
            // @.Extra alignment tab...@>
            // print_esc("cr");
            print_esc($globals, crate::strpool_str!("cr"));
            // help3("You have given more \span or & marks than there were")@/
            // ("in the preamble to the \halign or \valign now in progress.")@/
            // ("So I'll assume that you meant to type \cr instead.");
            help3!(
                $globals,
                crate::strpool_str!("You have given more \\span or & marks than there were"),
                crate::strpool_str!("in the preamble to the \\halign or \\valign now in progress."),
                crate::strpool_str!("So I'll assume that you meant to type \\cr instead.")
            );
            // extra_info(cur_align):=cr_code; error;
            extra_info!($globals, $globals.cur_align) = cr_code;
            error($globals)?;
            // end
        }
    }
    use crate::section_0063::print_esc;
    use crate::section_0073::print_err;
    use crate::section_0079::help3;
    use crate::section_0082::error;
    use crate::section_0115::null;
    use crate::section_0769::extra_info;
    use crate::section_0780::cr_code;
}}
