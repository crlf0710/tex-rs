//! ` `

// @<Allocate from the top...@>=
pub(crate) macro Allocate_from_the_top_of_node_p_and_goto_found($globals:expr, $p:expr, $r:expr, $lbl_found:lifetime) {{
    // begin node_size(p):=r-p; {store the remaining size}
    /// store the remaining size
    {
        node_size!($globals, $p) = $r as halfword - $p;
    }
    // @^inner loop@>
    // rover:=p; {start searching here next time}
    /// start searching here next time
    {
        $globals.rover = $p;
    }
    // goto found;
    crate::goto_forward_label!($lbl_found);
    // end
    use crate::section_0113::halfword;
    use crate::section_0124::node_size;
}}
