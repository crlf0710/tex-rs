//! ` `

use super::section_0110::min_quarterword;
// @<Store \(m)maximum values in the |hyf| table@>=
pub(crate) macro Store_maximum_values_in_the_hyf_table($globals:expr, $l:expr, $z:expr) {{
    /// an index into `hyf_distance`, etc.
    let mut v: integer;
    /// indices into `hc` or `hu`
    let mut i;
    // begin v:=trie_op(z);
    v = trie_op!($globals, $z) as integer;
    // repeat v:=v+op_start[cur_lang]; i:=l-hyf_distance[v];
    loop {
        v += $globals.op_start[$globals.cur_lang.numeric_value() as usize].get() as integer;
        i = $l - $globals.hyf_distance[u16_from_m_to_n::new(v as _)].get() as integer;
        // if hyf_num[v]>hyf[i] then hyf[i]:=hyf_num[v];
        if $globals.hyf_num[u16_from_m_to_n::new(v as _)].get() > $globals.hyf[i as usize].get() {
            $globals.hyf[i as usize] = $globals.hyf_num[u16_from_m_to_n::new(v as _)].get().into();
        }
        // v:=hyf_next[v];
        v = $globals.hyf_next[u16_from_m_to_n::new(v as _)] as integer;
        // until v=min_quarterword;
        if v == min_quarterword as integer {
            break;
        }
    }
    // end
    use crate::pascal::integer;
    use crate::pascal::u16_from_m_to_n;
    use crate::section_0921::trie_op;
}}
