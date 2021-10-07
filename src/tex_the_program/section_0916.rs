//! ` `
//！ The synchronization algorithm begins with |l=i+1<=j|.
//
// @<Put the \(c)characters |hu[i+1..@,]| into |post_break(r)|...@>=
pub(crate) macro Put_the_characters_hu_i_plus_1_to_end_into_post_break_r__appending_to_this_list_and_to_major_tail_until_synchronization_has_been_achieved($globals:expr, $major_tail:expr, $minor_tail:expr, $bchar:expr, $r_count:expr, $l:expr, $i:expr, $r:expr, $j:expr, $c:expr) {{
    /// where that character came from
    let mut c_loc: u8_from_0_to_n<U63>;
    // minor_tail:=null; post_break(r):=null; c_loc:=0;
    $minor_tail = null;
    post_break!($globals, $r) = null;
    c_loc = 0.into();
    // if bchar_label[hf]<>non_address then {put left boundary at beginning of new line}
    if $globals.bchar_label[$globals.hf] != non_address {
        /// put left boundary at beginning of new line
        const _: () = ();
        // begin decr(l); c:=hu[l]; c_loc:=l; hu[l]:=256;
        decr!($l);
        $c = ASCII_code::from($globals.hu[$l.get() as usize] as integer);
        c_loc = $l.get().into();
        $globals.hu[$l.get() as usize] = non_char;
        // end;
    }
    // while l<j do
    while $l.get() < $j.get() {
        // begin repeat l:=reconstitute(l,hn,bchar,non_char)+1;
        loop {
            $l = (reconstitute(
                $globals,
                $l.get().into(),
                $globals.hn.get().into(),
                $bchar,
                non_char,
            )?
            .get()
                + 1)
            .into();
            // if c_loc>0 then
            if c_loc > 0 {
                // begin hu[c_loc]:=c; c_loc:=0;
                $globals.hu[c_loc.get() as usize] = $c.numeric_value() as _;
                c_loc = 0.into();
                // end;
            }
            // if link(hold_head)>null then
            if link!($globals, hold_head) > null {
                // begin if minor_tail=null then post_break(r):=link(hold_head)
                if $minor_tail == null {
                    post_break!($globals, $r) = link!($globals, hold_head);
                }
                // else link(minor_tail):=link(hold_head);
                else {
                    link!($globals, $minor_tail) = link!($globals, hold_head);
                }
                // minor_tail:=link(hold_head);
                $minor_tail = link!($globals, hold_head);
                // while link(minor_tail)>null do minor_tail:=link(minor_tail);
                while link!($globals, $minor_tail) > null {
                    $minor_tail = link!($globals, $minor_tail);
                }
                // end;
            }
            // until l>=j;
            if $l.get() >= $j.get() {
                break;
            }
        }
        // while l>j do
        while $l.get() > $j.get() {
            // @<Append characters of |hu[j..@,]| to |major_tail|, advancing~|j|@>;
            crate::section_0917::Append_characters_of_hu_j_to_end_to_major_tail__advancing_j!(
                $globals,
                $j,
                $major_tail,
                $r_count,
                $bchar
            );
            // end
        }
    }
    use crate::pascal::integer;
    use crate::pascal::u8_from_0_to_n;
    use crate::section_0016::decr;
    use crate::section_0018::ASCII_code;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0145::post_break;
    use crate::section_0162::hold_head;
    use crate::section_0549::non_address;
    use crate::section_0549::non_char;
    use crate::section_0906::reconstitute;
    use typenum::U63;
}}
