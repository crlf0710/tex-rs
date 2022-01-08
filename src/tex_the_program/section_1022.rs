//! ` `
// @<Either append the insertion node |p|...@>=
pub(crate) macro Either_append_the_insertion_node_p_after_node_q__and_remove_it_from_the_current_page__or_delete_node_p($globals:expr, $p:expr, $prev_p:expr, $q:expr, $wait:expr) {{
    // link(prev_p):=link(p); link(p):=null;
    link!($globals, $prev_p) = link!($globals, $p);
    link!($globals, $p) = null;
    // if wait then
    if $wait {
        // begin link(q):=p; q:=p; incr(insert_penalties);
        link!($globals, $q) = $p;
        $q = $p;
        incr!($globals.insert_penalties);
        // end
    }
    // else  begin delete_glue_ref(split_top_ptr(p));
    else {
        delete_glue_ref($globals, split_top_ptr!($globals, $p));
        // free_node(p,ins_node_size);
        free_node($globals, $p, ins_node_size as _);
        // end;
    }
    // p:=prev_p
    $p = $prev_p;

    use crate::section_0016::incr;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0140::ins_node_size;
    use crate::section_0140::split_top_ptr;
    use crate::section_0201::delete_glue_ref;
}}
