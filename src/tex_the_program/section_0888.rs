//! ` `
// @<Append the new box to the current vertical list...@>=
pub(crate) macro Append_the_new_box_to_the_current_vertical_list__followed_by_the_list_of_special_nodes_taken_out_of_the_box_by_the_packager($globals:expr) {{
    // append_to_vlist(just_box);
    append_to_vlist($globals, $globals.just_box)?;
    // if adjust_head<>adjust_tail then
    if adjust_head != $globals.adjust_tail {
        // begin link(tail):=link(adjust_head); tail:=adjust_tail;
        link!($globals, tail!($globals)) = link!($globals, adjust_head);
        tail!($globals) = $globals.adjust_tail;
        //  end;
    }
    // adjust_tail:=null
    $globals.adjust_tail = null;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0162::adjust_head;
    use crate::section_0213::tail;
    use crate::section_0679::append_to_vlist;
}}
