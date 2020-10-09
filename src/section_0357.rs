//! @ Let's consider now what happens when |get_next| is looking at a token list.

// @<Input from token list, |goto restart| if end of list or
//   if a parameter needs to be expanded@>=
macro_rules! Input_from_token_list__goto_restart_if_end_of_list_or_if_a_parameter_needs_to_be_expanded {
    ($globals:expr, $lbl_restart:lifetime) => {
        // if loc<>null then {list not exhausted}
        if loc!($globals) != null {
            /// list not exhausted
            const _ : () = ();
            // @^inner loop@>
            //   begin t:=info(loc); loc:=link(loc); {move to next}
            //   if t>=cs_token_flag then {a control sequence token}
            //     begin cur_cs:=t-cs_token_flag;
            //     cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
            //     if cur_cmd>=outer_call then
            //       if cur_cmd=dont_expand then
            //         @<Get the next token, suppressing expansion@>
            //       else check_outer_validity;
            //     end
            //   else  begin cur_cmd:=t div @'400; cur_chr:=t mod @'400;
            //     case cur_cmd of
            //     left_brace: incr(align_state);
            //     right_brace: decr(align_state);
            //     out_param: @<Insert macro parameter and |goto restart|@>;
            //     othercases do_nothing
            //     endcases;
            //     end;
            //   end
        } else {
            // else  begin {we are done with this token list}
            /// we are done with this token list
            const _: () = ();
            // end_token_list; goto restart; {resume previous level}
            end_token_list($globals);
            /// resume previous level
            goto_backward_label!($lbl_restart);
            //end
        }

        use crate::section_0324::end_token_list;
        use crate::section_0115::null;
    }
}
