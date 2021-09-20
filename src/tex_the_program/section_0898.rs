//! ` `
// We let |j| be the index of the character being stored when a ligature node
// is being expanded, since we do not want to advance |hn| until we are sure
// that the entire ligature consists of letters. Note that it is possible
// to get to |done3| with |hn=0| and |hb| not set to any value.
//
// @<Move the characters of a ligature node to |hu| and |hc|...@>=
pub(crate) macro Move_the_characters_of_a_ligature_node_to_hu_and_hc__but_goto_done3_if_they_are_not_all_letters($globals:expr, $s:expr, $lbl_done3:lifetime) {{
    /// miscellaneous nodes of temporary interest
    let q;
    /// an index into `hc` or `hu`
    let j;
    // begin if font(lig_char(s))<>hf then goto done3;
    if font!($globals, lig_char!($s)) != $globals.hf {
        crate::goto_forward_label!($lbl_done3);
    }
    // j:=hn; q:=lig_ptr(s);@+if q>null then hyf_bchar:=character(q);
    j = $globals.hn;
    q = lig_ptr!($globals, $s);
    if q > null {
        $globals.hyf_bchar = character!($globals, q).numeric_value();
    }
    todo!("move character");
    // while q>null do
    //   begin c:=qo(character(q));
    //   if lc_code(c)=0 then goto done3;
    //   if j=63 then goto done3;
    //   incr(j); hu[j]:=c; hc[j]:=lc_code(c);@/
    //   q:=link(q);
    //   end;
    // hb:=s; hn:=j;
    // if odd(subtype(s)) then hyf_bchar:=font_bchar[hf]@+else hyf_bchar:=non_char;
    // end
    use crate::section_0115::null;
    use crate::section_0134::character;
    use crate::section_0134::font;
    use crate::section_0143::lig_char;
    use crate::section_0143::lig_ptr;
}}
