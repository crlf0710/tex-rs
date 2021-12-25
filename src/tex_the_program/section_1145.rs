//! @ When we enter display math mode, we need to call |line_break| to
//! process the partial paragraph that has just been interrupted by the
//! display. Then we can set the proper values of |display_width| and
//! |display_indent| and |pre_display_size|.
//
// @<Go into display math mode@>=
pub(crate) macro Go_into_display_math_mode($globals:expr) {
    #[allow(unused_variables)]
    {
        /// new or partial `pre_display_size`
        let mut w: scaled;
        /// new `display_width`
        let l;
        /// new `display_indent`
        let s;

        // begin if head=tail then {`\.{\\noindent\$\$}' or `\.{\$\${ }\$\$}'}
        if head!($globals) == tail!($globals) {
            /// `\noindent$$` or `$$ $$`
            const _: () = ();
            // begin pop_nest; w:=-max_dimen;
            pop_nest($globals);
            w = scaled::new_from_inner(-max_dimen);
            // end
        }
        // else  begin line_break(display_widow_penalty);@/
        else {
            line_break($globals, display_widow_penalty!($globals))?;
            // @<Calculate the natural width, |w|, by which the characters of the
            //   final line extend to the right of the reference point,
            //   plus two ems; or set |w:=max_dimen| if the non-blank information
            //   on that line is affected by stretching or shrinking@>;
            crate::section_1146::Calculate_the_natural_width__w__by_which_the_characters_of_the_final_line_extend_to_the_right_of_the_reference_point__plus_two_ems__or_set_w_to_max_dimen_if_the_non_blank_information_on_that_line_is_affected_by_stretching_or_shrinking!($globals, w);
            // end;
        }
        // {now we are in vertical mode, working on the list that will contain the display}
        /// now we are in vertical mode, working on the list that will contain the display
        const _: () = ();
        // @<Calculate the length, |l|, and the shift amount, |s|, of the display lines@>;
        crate::section_1149::Calculate_the_length__l__and_the_shift_amount__s__of_the_display_lines!($globals, l, s);
        // push_math(math_shift_group); mode:=mmode;
        push_math($globals, math_shift_group.into());
        mode!($globals) = mmode.into();
        // eq_word_define(int_base+cur_fam_code,-1);@/
        eq_word_define($globals, (int_base + cur_fam_code as word) as _, -1);
        // eq_word_define(dimen_base+pre_display_size_code,w);
        eq_word_define(
            $globals,
            (dimen_base + pre_display_size_code as word) as _,
            w.inner(),
        );
        // eq_word_define(dimen_base+display_width_code,l);
        eq_word_define($globals, (dimen_base + display_width_code as word) as _, l.inner());
        // eq_word_define(dimen_base+display_indent_code,s);
        eq_word_define($globals, (dimen_base + display_indent_code as word) as _, s.inner());
        // if every_display<>null then begin_token_list(every_display,every_display_text);
        if every_display!($globals) != null {
            todo!("token_list");
        }
        // if nest_ptr=1 then build_page;
        if $globals.nest_ptr == 1 {
            build_page($globals)?;
        }
        // end
        use crate::pascal::word;
        use crate::section_0101::scaled;
        use crate::section_0115::null;
        use crate::section_0211::mmode;
        use crate::section_0213::head;
        use crate::section_0213::mode;
        use crate::section_0213::tail;
        use crate::section_0217::pop_nest;
        use crate::section_0230::every_display;
        use crate::section_0230::int_base;
        use crate::section_0236::cur_fam_code;
        use crate::section_0236::dimen_base;
        use crate::section_0236::display_widow_penalty;
        use crate::section_0247::display_indent_code;
        use crate::section_0247::display_width_code;
        use crate::section_0247::pre_display_size_code;
        use crate::section_0269::math_shift_group;
        use crate::section_0278::eq_word_define;
        use crate::section_0421::max_dimen;
        use crate::section_0815::line_break;
        use crate::section_0994::build_page;
        use crate::section_1136::push_math;
    }
}
