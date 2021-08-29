//! @ Novices are not supposed to be using \.{\\patterns}, so the error
//! messages are terse. (Note that all error messages appear in \TeX's string
//! pool, even if they are used only by \.{INITEX}.)
//
// @<Enter all of the patterns into a linked trie...@>=
macro_rules! Enter_all_of_the_patterns_into_a_linked_trie__until_coming_to_a_right_brace {
    ($globals:expr) => {{
        trace_span!("Enter all of the patterns into a linked trie...");
        /// indices into `hc` and `hyf`; not always in `small_number` range
        let (mut k, mut l): (u8_from_0_to_n<U64>, u8_from_0_to_n<U64>);

        /// should the next digit be treated as a letter?
        let mut digit_sensed: boolean;

        // k:=0; hyf[0]:=0; digit_sensed:=false;
        k = 0.into();
        $globals.hyf[0] = 0.into();
        digit_sensed = false;
        region_forward_label!(
        |'done|
        {
        // loop@+  begin get_x_token;
        loop {
            get_x_token($globals)?;
            // case cur_cmd of
            // letter,other_char:@<Append a new letter or a hyphen level@>;
            if $globals.cur_cmd == letter || $globals.cur_cmd == other_char {
                Append_a_new_letter_or_a_hyphen_level!($globals, digit_sensed, k);
            }
            // spacer,right_brace: begin if k>0 then
            else if $globals.cur_cmd == spacer || $globals.cur_cmd == right_brace {
                if k > 0 {
                    // @<Insert a new pattern into the linked trie@>;
                    Insert_a_new_pattern_into_the_linked_trie!($globals, k, l);
                }
                // if cur_cmd=right_brace then goto done;
                if $globals.cur_cmd == right_brace {
                    goto_forward_label!('done);
                }
                // k:=0; hyf[0]:=0; digit_sensed:=false;
                k = 0.into();
                $globals.hyf[0] = 0.into();
                digit_sensed = false;
                // end;
            }
            // othercases begin print_err("Bad "); print_esc("patterns");
            else {
                print_err!($globals, strpool_str!("Bad "));
                print_esc($globals, strpool_str!("patterns"));
                // @.Bad \\patterns@>
                // help1("(See Appendix H.)"); error;
                help1!($globals, strpool_str!("(See Appendix H.)"));
                error($globals)?;
                // end
            }
            // endcases;
            // end;
        }
        // done:
        }
        'done <-
        );

        use crate::pascal::boolean;
        use crate::pascal::u8_from_0_to_n;
        use crate::section_0207::right_brace;
        use crate::section_0207::letter;
        use crate::section_0207::other_char;
        use crate::section_0207::spacer;
        use crate::section_0380::get_x_token;
        use typenum::U64;
    }}
}
