//! ` `

// @d add_delims_to(#)==#+math_shift,#+tab_mark,#+mac_param,
//   #+sub_mark,#+letter,#+other_char
//
// @<Handle situations involving spaces, braces, changes of state@>=
macro_rules! Handle_situations_involving_spaces_braces_changes_of_state {
    ($globals:expr, $state_plus_cur_cmd:expr) => {{
        trace_span!("Handle situations involving...");
        // mid_line+spacer:@<Enter |skip_blanks| state, emit a space@>;
        if $state_plus_cur_cmd == mid_line + spacer {
            todo!("mid_line + spacer");
        }
        // mid_line+car_ret:@<Finish line, emit a space@>;
        // skip_blanks+car_ret,any_state_plus(comment):
        //   @<Finish line, |goto switch|@>;
        // new_line+car_ret:@<Finish line, emit a \.{\\par}@>;
        else if $state_plus_cur_cmd == new_line + car_ret {
            Finish_line_emit_a_par!($globals);
            true
        }
        // mid_line+left_brace: incr(align_state);
        // skip_blanks+left_brace,new_line+left_brace: begin
        //   state:=mid_line; incr(align_state);
        //   end;
        // mid_line+right_brace: decr(align_state);
        // skip_blanks+right_brace,new_line+right_brace: begin
        //   state:=mid_line; decr(align_state);
        //   end;
        // add_delims_to(skip_blanks),add_delims_to(new_line): state:=mid_line;
        else {
            false
        }
    }}
}