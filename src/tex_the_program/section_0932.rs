//! ` `

// @<Insert hyphens as specified...@>=
pub(crate) macro Insert_hyphens_as_specified_in_hyph_list_h($globals:expr, $h:expr) {{
    /// temporary registers for list manipulation
    let mut s;
    // s:=hyph_list[h];
    s = $globals.hyph_list[$h];
    // while s<>null do
    while s != null {
        // begin hyf[info(s)]:=1; s:=link(s);
        $globals.hyf[info_inner!($globals, s) as usize] = 1.into();
        // end
        s = link!($globals, s);
    }
    use crate::section_0115::null;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
}}
