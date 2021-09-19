//! ` `

// @d add_delims_to(#)==#+math_shift,#+tab_mark,#+mac_param,
//   #+sub_mark,#+letter,#+other_char
pub(crate) macro state_plus_cur_cmd_matches_state_with_delims_added($state_plus_cur_cmd:expr, $state:expr) {{
    let result = $state_plus_cur_cmd == $state + math_shift
        || $state_plus_cur_cmd == $state + tab_mark
        || $state_plus_cur_cmd == $state + mac_param
        || $state_plus_cur_cmd == $state + sub_mark
        || $state_plus_cur_cmd == $state + letter
        || $state_plus_cur_cmd == $state + other_char;
    use crate::section_0207::*;
    result
}}
//
// @<Handle situations involving spaces, braces, changes of state@>=
pub(crate) macro Handle_situations_involving_spaces_braces_changes_of_state($globals:expr, $state_plus_cur_cmd:expr, $lbl_switch:lifetime) {{
    crate::trace_span_verbose!("Handle situations involving...");
    // mid_line+spacer:@<Enter |skip_blanks| state, emit a space@>;
    let processed = if $state_plus_cur_cmd == mid_line + spacer {
        crate::section_0349::Enter_skip_blanks_state__emit_a_space!($globals);
        true
    }
    // mid_line+car_ret:@<Finish line, emit a space@>;
    else if $state_plus_cur_cmd == mid_line + car_ret {
        crate::section_0348::Finish_line__emit_a_space!($globals);
        true
    }
    // skip_blanks+car_ret,any_state_plus(comment):
    //   @<Finish line, |goto switch|@>;
    else if $state_plus_cur_cmd == skip_blanks + car_ret
        || state_plus_cur_cmd_matches_any_case_plus!($state_plus_cur_cmd, comment)
    {
        crate::section_0350::Finish_line__goto_switch!($globals, $lbl_switch);
        true
    }
    // new_line+car_ret:@<Finish line, emit a \.{\\par}@>;
    else if $state_plus_cur_cmd == new_line + car_ret {
        crate::section_0351::Finish_line__emit_a_par!($globals);
        true
    }
    // mid_line+left_brace: incr(align_state);
    else if $state_plus_cur_cmd == mid_line + left_brace {
        incr!($globals.align_state);
        true
    }
    // skip_blanks+left_brace,new_line+left_brace: begin
    //   state:=mid_line; incr(align_state);
    //   end;
    else if $state_plus_cur_cmd == skip_blanks + left_brace
        || $state_plus_cur_cmd == new_line + left_brace
    {
        state!($globals) = mid_line;
        incr!($globals.align_state);
        true
    }
    // mid_line+right_brace: decr(align_state);
    else if $state_plus_cur_cmd == mid_line + right_brace {
        decr!($globals.align_state);
        true
    }
    // skip_blanks+right_brace,new_line+right_brace: begin
    //   state:=mid_line; decr(align_state);
    //   end;
    else if $state_plus_cur_cmd == skip_blanks + right_brace
        || $state_plus_cur_cmd == new_line + right_brace
    {
        state!($globals) = mid_line;
        decr!($globals.align_state);
        true
    }
    // add_delims_to(skip_blanks),add_delims_to(new_line): state:=mid_line;
    else if state_plus_cur_cmd_matches_state_with_delims_added!($state_plus_cur_cmd, skip_blanks)
        || state_plus_cur_cmd_matches_state_with_delims_added!($state_plus_cur_cmd, new_line)
    {
        state!($globals) = mid_line;
        true
    } else {
        false
    };
    use crate::section_0016::decr;
    use crate::section_0016::incr;
    use crate::section_0207::*;
    use crate::section_0302::state;
    use crate::section_0303::*;
    use crate::section_0344::state_plus_cur_cmd_matches_any_case_plus;
    processed
}}
