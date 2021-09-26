//! ` `
//! In this repeat loop we will insert another discretionary if |hyf[j-1]| is
//! odd, when both branches of the previous discretionary end at position |j-1|.
//! Strictly speaking, we aren't justified in doing this, because we don't know
//! that a hyphen after |j-1| is truly independent of those branches. But in almost
//! all applications we would rather not lose a potentially valuable hyphenation
//! point. (Consider the word `difficult', where the letter `c' is in position |j|.)
//
// @d advance_major_tail==begin major_tail:=link(major_tail); incr(r_count);
macro advance_major_tail($globals:expr, $major_tail:expr, $r_count:expr) {{
    $major_tail = link!($globals, $major_tail);
    incr!($r_count);
    // end
    use crate::section_0016::incr;
    use crate::section_0118::link;
}}

// @<Create and append a discretionary node as an alternative...@>=
pub(crate) macro Create_and_append_a_discretionary_node_as_an_alternative_to_the_unhyphenated_word__and_continue_to_develop_both_branches_until_they_become_equivalent($globals:expr, $j:expr, $s:expr, $l:expr) {{
    // repeat r:=get_node(small_node_size);
    loop {
        /// temporary registers for list manipulation
        let (r,);
        /// replacement count for discretionary
        let mut r_count: integer;
        /// the end of lists in the main and discretionary branches being reconstructed
        let (mut major_tail, mut minor_tail): (pointer, pointer);
        /// indices into `hc` or `hu`
        let (mut i,);
        r = get_node($globals, small_node_size as _)?;
        // link(r):=link(hold_head); type(r):=disc_node;
        link!($globals, r) = link!($globals, hold_head);
        r#type!($globals, r) = disc_node;
        // major_tail:=r; r_count:=0;
        major_tail = r;
        r_count = 0;
        // while link(major_tail)>null do advance_major_tail;
        while link!($globals, major_tail) > null {
            advance_major_tail!($globals, major_tail, r_count);
        }
        // i:=hyphen_passed; hyf[i]:=0;
        i = $globals.hyphen_passed;
        $globals.hyf[i.get() as usize] = 0.into();
        // @<Put the \(c)characters |hu[l..i]| and a hyphen into |pre_break(r)|@>;
        crate::section_0915::Put_the_characters_hu_l_to_i_and_a_hyphen_into_pre_break_r!($globals, minor_tail, $l, i, r);
        // @<Put the \(c)characters |hu[i+1..@,]| into |post_break(r)|, appending to this
        //   list and to |major_tail| until synchronization has been achieved@>;
        crate::section_0916::Put_the_characters_hu_i_plus_1_to_end_into_post_break_r__appending_to_this_list_and_to_major_tail_until_synchronization_has_been_achieved!($globals, minor_tail, $l, i, r, $j);
        // @<Move pointer |s| to the end of the current list, and set |replace_count(r)|
        //   appropriately@>;
        crate::section_0918::Move_pointer_s_to_the_end_of_the_current_list__and_set_replace_count_r_appropriately!($globals, $s, r, r_count, major_tail);
        // hyphen_passed:=j-1; link(hold_head):=null;
        $globals.hyphen_passed = small_number::new(($j - 1) as _);
        link!($globals, hold_head) = null;
        // until not odd(hyf[j-1])
        if !$globals.hyf[$j - 1].is_odd() {
            break;
        }
    }
    use crate::pascal::integer;
    use crate::pascal::IsOddOrEven;
    use crate::section_0101::small_number;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0125::get_node;
    use crate::section_0133::r#type;
    use crate::section_0141::small_node_size;
    use crate::section_0145::disc_node;
    use crate::section_0162::hold_head;
}}
