//! ` `
// @<If all characters of the family fit relative to |h|...@>=
pub(crate) macro If_all_characters_of_the_family_fit_relative_to_h__then_goto_found__otherwise_goto_not_found($globals:expr, $h:expr, $p:expr, $lbl_not_found:lifetime, $lbl_found:lifetime) {{
    /// runs through the family starting at `p`
    let mut q: trie_pointer;
    // q:=trie_r[p];
    q = $globals.trie_r[$p];
    // while q>0 do
    while q > 0 {
        let trie_c_q = $globals.trie_c[q];
        let trie_c_q_u8 = if trie_c_q.numeric_value() > 255 {
            todo!()
        } else {
            trie_c_q.numeric_value() as u8
        };
        // begin if trie_link(h+so(trie_c[q]))=0 then goto not_found;
        if trie_link!(
            $globals,
            ($h.get() as integer + trie_c_q_u8 as integer) as trie_pointer_repr
        ) == 0
        {
            crate::goto_forward_label!($lbl_not_found);
        }
        // q:=trie_r[q];
        q = $globals.trie_r[q];
        // end;
    }
    // goto found
    crate::goto_forward_label!($lbl_found);

    use crate::pascal::integer;
    use crate::section_0920::trie_pointer;
    use crate::section_0920::trie_pointer_repr;
    use crate::section_0921::trie_link;
}}
