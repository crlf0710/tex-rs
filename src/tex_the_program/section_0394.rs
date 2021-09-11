//! @ A slightly subtle point arises here: When the parameter delimiter ends
//! with `\.{\#\{}', the token list will have a left brace both before and
//! after the |end_match|\kern-.4pt. Only one of these should affect the
//! |align_state|, but both will be scanned, so we must make a correction.
//
// @<Advance \(r)|r|; |goto found| if the parameter delimiter has been fully...@>=
pub(crate) macro Advance_r__goto_found_if_the_parameter_delimiter_has_been_fully_matched__otherwise_goto_continue($globals:expr, $r:expr, $info_r:expr, $lbl_found:lifetime, $lbl_continue:lifetime) {{
    // begin r:=link(r);
    $r = link!($globals, $r);
    $info_r = info_tok!($globals, $r);
    // if (info(r)>=match_token)and(info(r)<=end_match_token) then
    if $info_r >= match_token && $info_r <= end_match_token {
        // begin if cur_tok<left_brace_limit then decr(align_state);
        if $globals.cur_tok.get() < left_brace_limit {
            decr!($globals.align_state);
        }
        // goto found;
        crate::goto_forward_label!($lbl_found);
    // end
    }
    // else goto continue;
    else {
        crate::goto_backward_label!($lbl_continue);
    }
    // end
    use crate::section_0016::decr;
    use crate::section_0118::info_tok;
    use crate::section_0118::link;
    use crate::section_0289::end_match_token;
    use crate::section_0289::left_brace_limit;
    use crate::section_0289::match_token;
}}
