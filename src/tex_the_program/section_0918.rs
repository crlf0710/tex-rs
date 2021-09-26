//! ` `
//! Ligature insertion can cause a word to grow exponentially in size. Therefore
//! we must test the size of |r_count| here, even though the hyphenated text
//! was at most 63 characters long.
//
// @<Move pointer |s| to the end of the current list...@>=
pub(crate) macro Move_pointer_s_to_the_end_of_the_current_list__and_set_replace_count_r_appropriately($globals:expr, $s:expr, $r:expr, $r_count:expr, $major_tail:expr) {{
    // if r_count>127 then {we have to forget the discretionary hyphen}
    if $r_count > 127 {
        /// we have to forget the discretionary hyphen
        const _: () = ();
        // begin link(s):=link(r); link(r):=null; flush_node_list(r);
        link!($globals, $s) = link!($globals, $r);
        link!($globals, $r) = null;
        flush_node_list($globals, $r)?;
        // end
    }
    // else begin link(s):=r; replace_count(r):=r_count;
    else {
        link!($globals, $s) = $r;
        replace_count!($globals, $r) = $r_count as _;
        // end;
    }
    // s:=major_tail
    $s = $major_tail as _;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0145::replace_count;
    use crate::section_0202::flush_node_list;
}}
