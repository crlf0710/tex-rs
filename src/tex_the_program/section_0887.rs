//! @ The following code begins with |q| at the end of the list to be
//! justified. It ends with |q| at the beginning of that list, and with
//! |link(temp_head)| pointing to the remainder of the paragraph, if any.
//
// @<Put the \(l)\.{\\leftskip} glue at the left...@>=
pub(crate) macro Put_the_leftskip_glue_at_the_left_and_detach_this_line($globals:expr, $q:expr) {{
    /// temporary registers for list manipulation
    let mut r: pointer;
    // r:=link(q); link(q):=null; q:=link(temp_head); link(temp_head):=r;
    r = link!($globals, $q);
    link!($globals, $q) = null;
    $q = link!($globals, temp_head);
    link!($globals, temp_head) = r;
    // if left_skip<>zero_glue then
    if left_skip!($globals) != zero_glue {
        // begin r:=new_param_glue(left_skip_code);
        r = new_param_glue($globals, left_skip_code.into())?;
        // link(r):=q; q:=r;
        link!($globals, r) = $q;
        $q = r;
        // end
    }
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0152::new_param_glue;
    use crate::section_0162::temp_head;
    use crate::section_0162::zero_glue;
    use crate::section_0224::left_skip;
    use crate::section_0224::left_skip_code;
}}
