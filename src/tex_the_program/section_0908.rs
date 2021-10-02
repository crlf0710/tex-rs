//! ` `

// @d append_charnode_to_t(#)== begin link(t):=get_avail; t:=link(t);
macro append_charnode_to_t($globals:expr, $t:expr, $v:expr) {{
    crate::trace_span!("append_charnode_to_t");
    link!($globals, $t) = get_avail($globals);
    $t = link!($globals, $t);
    // font(t):=hf; character(t):=#;
    assign_font_and_character!($globals, $t, $globals.hf, $v);
    // end
    use crate::section_0118::link;
    use crate::section_0120::get_avail;
    use crate::section_0134::assign_font_and_character;
}}

// @d set_cur_r==begin if j<n then cur_r:=qi(hu[j+1])@+else cur_r:=bchar;
pub(crate) macro set_cur_r($globals:expr, $j:expr, $n:expr, $bchar:expr, $hchar:expr, $cur_rh:expr) {{
    if $j < $n {
        $globals.cur_r = $globals.hu[$j.get() as usize + 1];
    } else {
        $globals.cur_r = $bchar;
    }
    // if odd(hyf[j]) then cur_rh:=hchar@+else cur_rh:=non_char;
    if $globals.hyf[$j.get() as usize].is_odd() {
        $cur_rh = $hchar;
    } else {
        $cur_rh = non_char;
    }
    // end
    use crate::pascal::IsOddOrEven;
    use crate::section_0549::non_char;
}}

// @<Set up data structures with the cursor following position |j|@>=
pub(crate) macro Set_up_data_structures_with_the_cursor_following_position_j($globals:expr, $j:expr, $n:expr, $t:expr, $bchar:expr, $hchar:expr, $cur_rh:expr) {{
    // cur_l:=qi(hu[j]); cur_q:=t;
    $globals.cur_l = $globals.hu[$j.get() as usize];
    $globals.cur_q = $t;
    // if j=0 then
    if $j == 0 {
        /// temporary register for list manipulation
        let mut p: pointer;
        // begin ligature_present:=init_lig; p:=init_list;
        $globals.ligature_present = $globals.init_lig;
        p = $globals.init_list;
        // if ligature_present then lft_hit:=init_lft;
        if $globals.ligature_present {
            $globals.lft_hit = $globals.init_lft;
        }
        // while p>null do
        while p > null {
            // begin append_charnode_to_t(character(p)); p:=link(p);
            let character_p = character!($globals, p);
            append_charnode_to_t!($globals, $t, character_p);
            p = link!($globals, p);
            // end;
        }
        // end
    }
    // else if cur_l<non_char then append_charnode_to_t(cur_l);
    else {
        if $globals.cur_l < non_char {
            append_charnode_to_t!($globals, $t, ASCII_code::from($globals.cur_l as integer));
        }
    }
    // lig_stack:=null; set_cur_r
    $globals.lig_stack = null;
    set_cur_r!($globals, $j, $n, $bchar, $hchar, $cur_rh);

    use crate::pascal::integer;
    use crate::section_0018::ASCII_code;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0134::character;
    use crate::section_0549::non_char;
}}
