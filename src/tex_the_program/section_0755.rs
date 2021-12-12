//! ` `

// @<Create a character node |p| for |nucleus(q)|...@>=
pub(crate) macro Create_a_character_node_p_for_nucleus_q__possibly_followed_by_a_kern_node_for_the_italic_correction__and_set_delta_to_the_italic_correction_if_a_subscript_is_present($globals:expr, $q:expr, $p:expr, $delta:expr) {{
    // begin fetch(nucleus(q));
    let fetched = fetch($globals, nucleus!($q));
    // if char_exists(cur_i) then
    if fetched.cur_i.char_exists() {
        // begin delta:=char_italic(cur_f)(cur_i); p:=new_character(cur_f,qo(cur_c));
        $delta = char_italic!($globals, fetched.cur_f, fetched.cur_i);
        $p = new_character($globals, fetched.cur_f, fetched.cur_c);
        // if (math_type(nucleus(q))=math_text_char)and(space(cur_f)<>0) then
        if math_type!($globals, nucleus!($q)) == math_type_kind::math_text_char as _
            && space!($globals, fetched.cur_f) != scaled::zero()
        {
            // delta:=0; {no italic correction in mid-word of text font}
            /// no italic correction in mid-word of text font
            const _: () = ();
            $delta = scaled::zero();
        }
        // if (math_type(subscr(q))=empty)and(delta<>0) then
        if math_type!($globals, subscr!($q)) == math_type_kind::empty as _
            && $delta != scaled::zero()
        {
            // begin link(p):=new_kern(delta); delta:=0;
            link!($globals, $p) = new_kern($globals, $delta)?;
            $delta = scaled::zero();
            // end;
        }
        // end
    }
    // else p:=null;
    else {
        $p = null;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0156::new_kern;
    use crate::section_0554::char_italic;
    use crate::section_0558::space;
    use crate::section_0582::new_character;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::nucleus;
    use crate::section_0681::subscr;
    use crate::section_0722::fetch;
}}
