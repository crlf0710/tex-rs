//! ` `

// @<Append characters of |hu[j..@,]|...@>=
pub(crate) macro Append_characters_of_hu_j_to_end_to_major_tail__advancing_j($globals:expr, $j:expr, $major_tail:expr, $r_count:expr, $bchar:expr) {{
    // begin j:=reconstitute(j,hn,bchar,non_char)+1;
    $j = (reconstitute(
        $globals,
        $j.get().into(),
        $globals.hn.get().into(),
        $bchar,
        non_char,
    )?
    .get()
        + 1)
    .into();
    // link(major_tail):=link(hold_head);
    link!($globals, $major_tail) = link!($globals, hold_head);
    // while link(major_tail)>null do advance_major_tail;
    while link!($globals, $major_tail) > null {
        advance_major_tail!($globals, $major_tail, $r_count);
    }
    // end
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0162::hold_head;
    use crate::section_0549::non_char;
    use crate::section_0906::reconstitute;
    use crate::section_0914::advance_major_tail;
}}
