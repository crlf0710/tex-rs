//! ` `
// @<If the string |hyph_word[h]| is less than \(hc)...@>=
pub(crate) macro If_the_string_hyph_word_h_is_less_than_hc_1_to_hn__goto_not_found__but_if_the_two_strings_are_equal__set_hyf_to_the_hyphen_positions_and_goto_found($globals:expr, $h:expr, $lbl_not_found:lifetime, $lbl_found:lifetime) {{
    /// an index into `str_start`
    let k;
    // k:=hyph_word[h]; if k=0 then goto not_found;
    k = $globals.hyph_word[$h as u16];
    if k == 0 {
        crate::goto_forward_label!($lbl_not_found);
    }
    crate::region_forward_label! {
        |'done|
        {
            // if length(k)<hn then goto not_found;
            #[cfg(not(feature = "unicode_support"))]
            {
                // if length(k)<hn then goto not_found;
                if length($globals, k) < $globals.hn {
                    crate::goto_forward_label!($lbl_not_found);
                }
                // if length(k)=hn then
                if length($globals, k) == $globals.hn {
                    // begin j:=1; u:=str_start[k];
                    // repeat if so(str_pool[u])<hc[j] then goto not_found;
                    // if so(str_pool[u])>hc[j] then goto done;
                    // incr(j); incr(u);
                    // until j>hn;
                    // @<Insert hyphens as specified in |hyph_list[h]|@>;
                    // decr(hn); goto found;
                    // end;
                    todo!("without unicode support")
                }
            }
            #[cfg(feature = "unicode_support")]
            {
                let count = $globals
                    .str_pool
                    .str_ascii_codes(&$globals.str_start, k)
                    .count();
                // if length(k)<hn then goto not_found;
                if count < $globals.hn.get() as usize {
                    crate::goto_forward_label!($lbl_not_found);
                }
                // if length(k)=hn then
                if count == $globals.hn.get() as usize {
                    // begin j:=1; u:=str_start[k];
                    // repeat if so(str_pool[u])<hc[j] then goto not_found;
                    // if so(str_pool[u])>hc[j] then goto done;
                    // incr(j); incr(u);
                    // until j>hn;
                    for (ch, j) in $globals
                        .str_pool
                        .str_ascii_codes(&$globals.str_start, k)
                        .zip(1..=$globals.hn.get())
                    {
                        let ch = xord(ch).numeric_value();
                        if ch < $globals.hc[j as usize] {
                            crate::goto_forward_label!($lbl_not_found);
                        }
                        if ch > $globals.hc[j as usize] {
                            crate::goto_forward_label!('done);
                        }
                    }
                    // @<Insert hyphens as specified in |hyph_list[h]|@>;
                    crate::section_0932::Insert_hyphens_as_specified_in_hyph_list_h!($globals, $h as u16);
                    // decr(hn); goto found;
                    decr!($globals.hn);
                    crate::goto_forward_label!($lbl_found);
                    // end;
                    todo!("without unicode support")
                }
            }
        }
        // done:
        'done <-
    }
    use crate::section_0016::decr;
    use crate::section_0020::xord;
}}
