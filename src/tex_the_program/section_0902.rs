//! @ \TeX\ will never insert a hyphen that has fewer than
//! \.{\\lefthyphenmin} letters before it or fewer than
//! \.{\\righthyphenmin} after it; hence, a short word has
//! comparatively little chance of being hyphenated. If no hyphens have
//! been found, we can save time by not having to make any changes to the
//! paragraph.
//
// @<If no hyphens were found, |return|@>=
pub(crate) macro If_no_hyphens_were_found__return($globals:expr) {{
    crate::region_forward_label! {
    |'found1|
    {
        // for j:=l_hyf to hn-r_hyf do if odd(hyf[j]) then goto found1;
        for j in $globals.l_hyf..=$globals.hn.get() as integer - $globals.r_hyf {
            if $globals.hyf[j as usize].is_odd() {
                crate::goto_forward_label!('found1);
            }
        }
        // return;
        crate::return_nojump!();
    }
    // found1:
    'found1 <-
    };
    use crate::pascal::integer;
    use crate::pascal::IsOddOrEven;
}}
