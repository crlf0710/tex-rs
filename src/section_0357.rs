//! @ Let's consider now what happens when |get_next| is looking at a token list.

// @<Input from token list, |goto restart| if end of list or
//   if a parameter needs to be expanded@>=
macro_rules! Input_from_token_list__goto_restart_if_end_of_list_or_if_a_parameter_needs_to_be_expanded {
    ($globals:expr, $lbl_restart:lifetime) => {{
        trace_span!("Input from token list...");

        // if loc<>null then {list not exhausted}
        if loc!($globals) != null {
            /// list not exhausted
            const _ : () = ();
            // @^inner loop@>
            // begin t:=info(loc); loc:=link(loc); {move to next}
            #[cfg(not(feature = "unicode_support"))]
            let t: cur_tok_type = cur_tok_type::new(info!($globals, loc!($globals)));
            #[cfg(feature = "unicode_support")]
            let t: cur_tok_type = cur_tok_type::new(crate::unicode_support::info_value($globals, info!($globals, loc!($globals))));
            /// move to next
            {
                loc!($globals) = link!($globals, loc!($globals));
                trace_expr!("loc = link(loc) = {}", loc!($globals));
            }
            // if t>=cs_token_flag then {a control sequence token}
            if let Some(cs) = t.get_cs() {
                /// a control sequence token
                const _ : () = ();
                // begin cur_cs:=t-cs_token_flag;
                $globals.cur_cs = cs;
                // cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
                $globals.cur_cmd = eq_type!($globals, $globals.cur_cs);
                $globals.cur_chr = cur_chr_type::new(equiv!($globals, $globals.cur_cs) as _);
                // if cur_cmd>=outer_call then
                if $globals.cur_cmd >= outer_call {
                    // if cur_cmd=dont_expand then
                    //   @<Get the next token, suppressing expansion@>
                    // else check_outer_validity;
                }
                // end
            }
            // else  begin cur_cmd:=t div @'400; cur_chr:=t mod @'400;
            else {
                let (cmd, chr) = t.get_cmd_and_chr().unwrap();
                $globals.cur_cmd = cmd;
                $globals.cur_chr = chr;
                // case cur_cmd of
                // left_brace: incr(align_state);
                if $globals.cur_cmd == left_brace {
                    incr!($globals.align_state);
                } else if $globals.cur_cmd == right_brace {
                    // right_brace: decr(align_state);
                    decr!($globals.align_state);
                } else if $globals.cur_cmd == out_param {
                    // out_param: @<Insert macro parameter and |goto restart|@>;
                    todo!();
                } else {
                    // othercases do_nothing
                    do_nothing!();
                }
                // endcases;
                // end;
            }
            // end
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
        
        use crate::section_0207::left_brace;
        use crate::section_0207::right_brace;
        use crate::section_0207::out_param;
        use crate::section_0210::outer_call;
        use crate::section_0289::cs_token_flag;
        use crate::section_0297::cur_tok_type;
        use crate::section_0297::cur_chr_type;
        use crate::section_0324::end_token_list;
        use crate::section_0115::null;
    }}
}
