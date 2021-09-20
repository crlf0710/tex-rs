//! ` `
// Assuming that these auxiliary tables have been set up properly, the
// hyphenation algorithm is quite short. In the following code we set |hc[hn+2]|
// to the impossible value 256, in order to guarantee that |hc[hn+3]| will
// never be fetched.
//
// @<Find hyphen locations for the word in |hc|...@>=
pub(crate) macro Find_hyphen_locations_for_the_word_in_hc__or_return($globals:expr) {{
    // for j:=0 to hn do hyf[j]:=0;
    for j in 0..=$globals.hn.get() {
        $globals.hyf[j as usize] = 0.into();
    }
    crate::region_forward_label! {
        |'found|
        {
            // @<Look for the word |hc[1..hn]| in the exception table, and |goto found| (with
            //   |hyf| containing the hyphens) if an entry is found@>;
            crate::section_0930::Look_for_the_word_hc_1_to_hn_in_the_exception_table__and_goto_found_with_hyf_containing_the_hyphens_if_an_entry_is_found!($globals, 'found);
            // if trie_char(cur_lang+1)<>qi(cur_lang) then return; {no patterns for |cur_lang|}
            if trie_char!($globals, $globals.cur_lang.numeric_value() + 1) != $globals.cur_lang {
                /// no patterns for `cur_lang`
                crate::return_nojump!();
            }
            // hc[0]:=0; hc[hn+1]:=0; hc[hn+2]:=256; {insert delimiters}
            /// insert delimiters
            const _ : () = ();
            $globals.hc[0] = 0;
            $globals.hc[$globals.hn.get() as usize + 1] = 0;
            $globals.hc[$globals.hn.get() as usize + 2] = non_char;
            // for j:=0 to hn-r_hyf+1 do
            for j in 0..=$globals.hn.get() as integer - $globals.r_hyf + 1 {
                /// an index into `trie`
                let mut z: u16;
                /// indices into `hc` or `hu`
                let mut l;
                // begin z:=trie_link(cur_lang+1)+hc[j]; l:=j;
                z = trie_link!($globals, $globals.cur_lang.numeric_value() as u16 + 1) + $globals.hc[j as usize] as u16;
                l = j;
                // while hc[l]=qo(trie_char(z)) do
                while $globals.hc[l as usize] as integer == trie_char!($globals, z).numeric_value() as integer {
                    // begin if trie_op(z)<>min_quarterword then
                    if trie_op!($globals, z) != min_quarterword {
                        // @<Store \(m)maximum values in the |hyf| table@>;
                        crate::section_0924::Store_maximum_values_in_the_hyf_table!($globals, l, z);
                    }
                    // incr(l); z:=trie_link(z)+hc[l];
                    incr!(l);
                    z = trie_link!($globals, z) + $globals.hc[l as usize] as u16;
                    // end;
                }
                // end;
            }
        }
        // found: for j:=0 to l_hyf-1 do hyf[j]:=0;
        'found <-
    };
    for j in 0..=$globals.l_hyf - 1 {
        $globals.hyf[j as usize] = 0.into();
    }
    // for j:=0 to r_hyf-1 do hyf[hn-j]:=0
    for j in 0..=$globals.r_hyf - 1 {
        $globals.hyf[$globals.hn.get() as usize - j as usize] = 0.into();
    }
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0110::min_quarterword;
    use crate::section_0549::non_char;
    use crate::section_0921::trie_link;
    use crate::section_0921::trie_char;
    use crate::section_0921::trie_op;
}}
