//! ` `
//! We move the pre-break list from inside node |q| to the main list by
//! re\-attaching it just after the present node |q|, then resetting |q|.
//
// @<Transplant the pre-break list@>=
pub(crate) macro Transplant_the_pre_break_list($globals:expr, $q:expr) {{
    /// temporary registers for list manipulation
    let mut s;
    // begin s:=pre_break(q); link(q):=s;
    s = pre_break!($globals, $q);
    link!($globals, $q) = s;
    // while link(s)<>null do s:=link(s);
    while link!($globals, s) != null {
        s = link!($globals, s);
    }
    // pre_break(q):=null; q:=s;
    pre_break!($globals, $q) = null;
    $q = s;
    // end
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0145::pre_break;
}}
