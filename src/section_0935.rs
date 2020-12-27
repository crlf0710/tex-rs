//! ` `

// @<Enter as many...@>=
macro_rules! Enter_as_many_hyphenation_exceptions_as_are_listed__until_coming_to_a_right_brace__then_return {
    ($globals:expr) => {{
        /// length of current word; not always a `small_number`
        let mut n: u8_from_0_to_n<U64>;
        /// head of a list of hyphen positions
        let mut p: pointer;
        // n:=0; p:=null;
        n = 0.into();
        p = null;
        // loop@+  begin get_x_token;
        loop {
            get_x_token($globals)?;
            region_backward_label!(
            'reswitch <-
            {
                // reswitch: case cur_cmd of
                // letter,other_char,char_given:@<Append a new letter or hyphen@>;
                if $globals.cur_cmd == letter || $globals.cur_cmd == other_char || $globals.cur_cmd == char_given {
                    Append_a_new_letter_or_hyphen!($globals, n, p);
                }
                // char_num: begin scan_char_num; cur_chr:=cur_val; cur_cmd:=char_given;
                else if $globals.cur_cmd == char_num {
                    todo!("scan_char_num");
                    // goto reswitch;
                    // end;
                }
                // spacer,right_brace: begin if n>1 then @<Enter a hyphenation exception@>;
                else if $globals.cur_cmd == spacer || $globals.cur_cmd == right_brace {
                    if n > 1 {
                        Enter_a_hyphenation_exception!($globals, n, p);
                    }
                    // if cur_cmd=right_brace then return;
                    if $globals.cur_cmd == right_brace {
                        return_nojump!();
                    }
                    // n:=0; p:=null;
                    n = 0.into();
                    p = null;
                    // end;
                }
                // othercases @<Give improper \.{\\hyphenation} error@>
                else {
                    todo!("error");
                }
                // endcases;
            }
            |'reswitch|
            );
            // end
        }
        use crate::pascal::u8_from_0_to_n;
        use crate::section_0115::pointer;
        use crate::section_0115::null;
        use crate::section_0207::spacer;
        use crate::section_0207::letter;
        use crate::section_0207::other_char;
        use crate::section_0207::right_brace;
        use crate::section_0208::char_num;
        use crate::section_0208::char_given;
        use crate::section_0380::get_x_token;
        use typenum::U64;
    }}
}
