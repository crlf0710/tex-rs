//! ` `

// @<Destroy the |t| nodes following |q|...@>=
pub(crate) macro Destroy_the_t_nodes_following_q__and_make_r_point_to_the_following_node($globals:expr, $q:expr, $r:expr, $t:expr) {{
    /// temporary registers for list manipulation
    let s;
    // if t=0 then r:=link(q)
    if $t == 0 {
        $r = link!($globals, $q);
    }
    // else  begin r:=q;
    else {
        $r = $q;
        // while t>1 do
        while $t > 1 {
            // begin r:=link(r); decr(t);
            $r = link!($globals, $r);
            decr!($t);
            // end;
        }
        // s:=link(r);
        s = link!($globals, $r);
        // r:=link(s); link(s):=null;
        $r = link!($globals, s);
        link!($globals, s) = null;
        // flush_node_list(link(q)); replace_count(q):=0;
        flush_node_list($globals, link!($globals, $q))?;
        replace_count!($globals, $q) = 0;
        // end
    }
    use crate::section_0016::decr;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0145::replace_count;
    use crate::section_0202::flush_node_list;
}}
