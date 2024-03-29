//! ` `

// @<Clear off...@>=
pub(crate) macro Clear_off_top_level_from_save_stack {
    ($globals:expr) => {{
        /// saved level, if in fullword regions of `eqtb`
        let mut l: quarterword = 0;
        // NOTE: this variables is not always updated. Is this a bug?

        crate::region_forward_label!(
            |'done|
            {
                // loop@+begin decr(save_ptr);
                loop {
                    decr!($globals.save_ptr);
                    // if save_type(save_ptr)=level_boundary then goto done;
                    if save_type!($globals, $globals.save_ptr) == level_boundary {
                        crate::goto_forward_label!('done);
                    }
                    /// position to be restored
                    let p: pointer;
                    // p:=save_index(save_ptr);
                    p = save_index!($globals, $globals.save_ptr);
                    // if save_type(save_ptr)=insert_token then
                    if save_type!($globals, $globals.save_ptr) == insert_token {
                        // @<Insert token |p| into \TeX's input@>
                        crate::section_0326::Insert_token_p_into_TeX_s_input!($globals, p);
                    }
                    // else  begin if save_type(save_ptr)=restore_old_value then
                    else {
                        if save_type!($globals, $globals.save_ptr) == restore_old_value {
                            // begin l:=save_level(save_ptr); decr(save_ptr);
                            l = save_level!($globals, $globals.save_ptr);
                            decr!($globals.save_ptr);
                            // end
                        }
                        // else save_stack[save_ptr]:=eqtb[undefined_control_sequence];
                        else {
                            $globals.save_stack[$globals.save_ptr] = $globals.eqtb[undefined_control_sequence];
                        }
                        // @<Store \(s)|save_stack[save_ptr]| in |eqtb[p]|, unless
                        //   |eqtb[p]| holds a global value@>;
                        crate::section_0283::Store_s_save_stack_save_ptr_in_eqtb_p__unless_eqtb_p_holds_a_global_value!($globals, p, l);
                        // end;
                    }
                    // end;
                }
            }
            // done: cur_group:=save_level(save_ptr); cur_boundary:=save_index(save_ptr)
            'done <-
        );
        $globals.cur_group = save_level!($globals, $globals.save_ptr).into();
        $globals.cur_boundary = save_index!($globals, $globals.save_ptr).into();

        use crate::section_0016::decr;
        use crate::section_0113::quarterword;
        use crate::section_0115::pointer;
        use crate::section_0222::undefined_control_sequence;
        use crate::section_0268::level_boundary;
        use crate::section_0268::restore_old_value;
        use crate::section_0268::insert_token;
        use crate::section_0268::save_level;
        use crate::section_0268::save_index;
        use crate::section_0268::save_type;
    }}
}
