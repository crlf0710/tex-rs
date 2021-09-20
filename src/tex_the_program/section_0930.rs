//! ` `
// First we compute the hash code |h|, then we search until we either
// find the word or we don't. Words from different languages are kept
// separate by appending the language code to the string.
//
// @<Look for the word |hc[1...@>=
pub(crate) macro Look_for_the_word_hc_1_to_hn_in_the_exception_table__and_goto_found_with_hyf_containing_the_hyphens_if_an_entry_is_found($globals:expr, $lbl_found:lifetime) {{
    /// an index into `hyph_word` and `hyph_list`
    let mut h: integer;
    // h:=hc[1]; incr(hn); hc[hn]:=cur_lang;
    h = $globals.hc[1] as integer;
    incr!($globals.hn);
    $globals.hc[$globals.hn.get() as usize] = $globals.cur_lang.numeric_value() as _;
    // for j:=2 to hn do h:=(h+h+hc[j]) mod hyph_size;
    for j in 2..=$globals.hn.get() {
        h = (h + h + $globals.hc[j as usize] as integer) % hyph_size;
    }
    crate::region_forward_label! {
        |'not_found|
        {
            // loop@+  begin @<If the string |hyph_word[h]| is less than \(hc)|hc[1..hn]|,
            //     |goto not_found|; but if the two strings are equal,
            //     set |hyf| to the hyphen positions and |goto found|@>;
            crate::section_0931::If_the_string_hyph_word_h_is_less_than_hc_1_to_hn__goto_not_found__but_if_the_two_strings_are_equal__set_hyf_to_the_hyphen_positions_and_goto_found!($globals, h, 'not_found, $lbl_found);
            loop {
                // if h>0 then decr(h)@+else h:=hyph_size;
                if h > 0 {
                    decr!(h);
                } else {
                    h = hyph_size;
                }
                // end;
            }
        }
        // not_found: decr(hn)
        'not_found <-
    }
    decr!($globals.hn);
    use crate::pascal::integer;
    use crate::section_0012::hyph_size;
    use crate::section_0016::decr;
    use crate::section_0016::incr;
    use crate::section_0925::hyph_pointer;
}}
