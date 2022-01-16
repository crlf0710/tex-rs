//! ` `

// @<Make node |p| look like a |char_node| and |goto reswitch|@>=
pub(crate) macro Make_node_p_look_like_a_char_node_and_goto_reswitch($globals:expr, $p:expr, $lbl_reswitch:lifetime) {{
    // begin mem[lig_trick]:=mem[lig_char(p)]; link(lig_trick):=link(p);
    $globals.mem[lig_trick] = $globals.mem[lig_char!($p)];
    link!($globals, lig_trick) = link!($globals, $p);
    // p:=lig_trick; goto reswitch;
    $p = lig_trick;
    crate::goto_backward_label!($lbl_reswitch);
    // end
    use crate::section_0118::link;
    use crate::section_0143::lig_char;
    use crate::section_0162::lig_trick;
}}
