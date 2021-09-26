//! ` `
//! When the following code is performed, |hyf[0]| and |hyf[hn]| will be zero.
//
// @<Reconstitute nodes for the hyphenated word...@>=
pub(crate) macro Reconstitute_nodes_for_the_hyphenated_word__inserting_discretionary_hyphens($globals:expr, $j:expr, $bchar:expr, $q:expr, $r:expr, $s:expr) {{
    let mut l;
    // repeat l:=j; j:=reconstitute(j,hn,bchar,qi(hyf_char))+1;
    loop {
        l = $j;
        $j = reconstitute(
            $globals,
            small_number::new($j as _),
            $globals.hn.get().into(),
            $bchar,
            $globals.hyf_char as u32,
        )?
        .get() as usize
            + 1;
        // if hyphen_passed=0 then
        if $globals.hyphen_passed == 0 {
            // begin link(s):=link(hold_head);
            link!($globals, $s) = link!($globals, hold_head);
            // while link(s)>null do s:=link(s);
            while link!($globals, $s) > null {
                $s = link!($globals, $s);
            }
            // if odd(hyf[j-1]) then
            if $globals.hyf[$j - 1].is_odd() {
                // begin l:=j; hyphen_passed:=j-1; link(hold_head):=null;
                l = $j;
                $globals.hyphen_passed = small_number::new(($j - 1) as _);
                link!($globals, hold_head) = null;
                // end;
            }
            // end;
        }
        // if hyphen_passed>0 then
        if $globals.hyphen_passed > 0 {
            // @<Create and append a discretionary node as an alternative to the
            //   unhyphenated word, and continue to develop both branches until they
            //   become equivalent@>;
            crate::section_0914::Create_and_append_a_discretionary_node_as_an_alternative_to_the_unhyphenated_word__and_continue_to_develop_both_branches_until_they_become_equivalent!(
                $globals, $j, $s, l
            );
        }
        // until j>hn;
        if $j > $globals.hn.get() as usize {
            break;
        }
    }
    // link(s):=q
    link!($globals, $s) = $q;

    use crate::pascal::IsOddOrEven;
    use crate::section_0101::small_number;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0162::hold_head;
    use crate::section_0906::reconstitute;
}}
