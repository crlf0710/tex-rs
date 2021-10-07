//! ` `
//! The new hyphen might combine with the previous character via ligature
//! or kern. At this point we have |l-1<=i<j| and |i<hn|.
//
// @<Put the \(c)characters |hu[l..i]| and a hyphen into |pre_break(r)|@>=
pub(crate) macro Put_the_characters_hu_l_to_i_and_a_hyphen_into_pre_break_r($globals:expr, $minor_tail:expr, $l:expr, $i:expr, $r:expr, $c:expr) {{
    /// the hyphen, if it exists
    let hyf_node: pointer;
    // minor_tail:=null; pre_break(r):=null; hyf_node:=new_character(hf,hyf_char);
    $minor_tail = null;
    pre_break!($globals, $r) = null;
    hyf_node = new_character($globals, $globals.hf, ASCII_code::from($globals.hyf_char));
    // if hyf_node<>null then
    if hyf_node != null {
        // begin incr(i); c:=hu[i]; hu[i]:=hyf_char; free_avail(hyf_node);
        incr!($i);
        $c = ASCII_code::from($globals.hu[$i.get() as usize] as integer);
        $globals.hu[$i.get() as usize] = $globals.hyf_char as _;
        free_avail!($globals, hyf_node);
        // end;
    }
    // while l<=i do
    while $l.get() <= $i.get() {
        // begin l:=reconstitute(l,i,font_bchar[hf],non_char)+1;
        $l = (reconstitute(
            $globals,
            small_number::new($l.get() as _),
            $i,
            $globals.font_bchar[$globals.hf],
            non_char,
        )?
        .get()
            + 1)
        .into();
        // if link(hold_head)>null then
        if link!($globals, hold_head) > null {
            // begin if minor_tail=null then pre_break(r):=link(hold_head)
            if $minor_tail == null {
                pre_break!($globals, $r) = link!($globals, hold_head);
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
        // end;
    }
    // if hyf_node<>null then
    if hyf_node != null {
        // begin hu[i]:=c; {restore the character in the hyphen position}
        /// restore the character in the hyphen position
        const _: () = ();
        $globals.hu[$i.get() as usize] = $c.numeric_value() as _;
        // l:=i; decr(i);
        $l = $i.get().into();
        decr!($i);
        // end
    }
    use crate::pascal::integer;
    use crate::section_0016::decr;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0101::small_number;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0121::free_avail;
    use crate::section_0145::pre_break;
    use crate::section_0162::hold_head;
    use crate::section_0549::non_char;
    use crate::section_0582::new_character;
    use crate::section_0906::reconstitute;
}}
