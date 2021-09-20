//! ` `
// We let |j| be the index of the character being stored when a ligature node
// is being expanded, since we do not want to advance |hn| until we are sure
// that the entire ligature consists of letters. Note that it is possible
// to get to |done3| with |hn=0| and |hb| not set to any value.
//
// @<Move the characters of a ligature node to |hu| and |hc|...@>=
pub(crate) macro Move_the_characters_of_a_ligature_node_to_hu_and_hc__but_goto_done3_if_they_are_not_all_letters($globals:expr, $s:expr, $lbl_done3:lifetime) {{
    crate::trace_span!("Move the characters of a ligature node to |hu| and |hc|...");
    /// miscellaneous nodes of temporary interest
    let mut q;
    /// an index into `hc` or `hu`
    let mut j;
    crate::trace_debug_expr_verbose!(
        "s = {}, lig_char(s) = {}, lig_char(s).lh = {}",
        $s,
        lig_char!($s),
        $globals.mem[lig_char!($s)][crate::section_0113::MEMORY_WORD_HH_LH]
    );
    // begin if font(lig_char(s))<>hf then goto done3;
    if font!($globals, lig_char!($s)) != $globals.hf {
        crate::goto_forward_label!($lbl_done3);
    }
    // j:=hn; q:=lig_ptr(s);@+if q>null then hyf_bchar:=character(q);
    j = $globals.hn;
    q = lig_ptr!($globals, $s);
    crate::trace_debug_expr_verbose!(
        "q = {}, q.lh = {}",
        q,
        $globals.mem[q][crate::section_0113::MEMORY_WORD_HH_LH]
    );
    if q > null {
        $globals.hyf_bchar = character!($globals, q).numeric_value();
    }
    // while q>null do
    while q > null {
        // begin c:=qo(character(q));
        let c = character!($globals, q);
        // if lc_code(c)=0 then goto done3;
        if lc_code!($globals, c) == 0 {
            crate::goto_forward_label!($lbl_done3);
        }
        // if j=63 then goto done3;
        if j == 63 {
            crate::goto_forward_label!($lbl_done3);
        }
        // incr(j); hu[j]:=c; hc[j]:=lc_code(c);@/
        incr!(j);
        $globals.hu[j.get() as usize] = c.numeric_value() as _;
        $globals.hc[j.get() as usize] = lc_code!($globals, c) as _;
        // q:=link(q);
        q = link!($globals, q);
        // end;
    }
    // hb:=s; hn:=j;
    $globals.hb = $s;
    $globals.hn = j;
    // if odd(subtype(s)) then hyf_bchar:=font_bchar[hf]@+else hyf_bchar:=non_char;
    if subtype!($globals, $s).is_odd() {
        $globals.hyf_bchar = $globals.font_bchar[$globals.hf];
    } else {
        $globals.hyf_bchar = non_char;
    }
    // end
    use crate::pascal::integer;
    use crate::pascal::IsOddOrEven;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0134::character;
    use crate::section_0134::font;
    use crate::section_0143::lig_char;
    use crate::section_0143::lig_ptr;
    use crate::section_0230::lc_code;
    use crate::section_0549::non_char;
}}
