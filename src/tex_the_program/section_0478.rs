//! @ Here we insert an entire token list created by |the_toks| without
//! expanding it further.
//
// @<Expand the next part of the input@>=
macro_rules! Expand_the_next_part_of_the_input {
    ($globals:expr, $p:expr, $q:expr) => {{
        region_forward_label! {
        |'done2|
        {
            // begin loop begin get_next;
            loop {
                get_next($globals)?;
                // if cur_cmd<=max_command then goto done2;
                if $globals.cur_cmd <= max_command {
                    goto_forward_label!('done2);
                }
                // if cur_cmd<>the then expand
                if $globals.cur_cmd != the {
                    expand($globals)?;
                }
                // else  begin q:=the_toks;
                else {
                    $q = the_toks($globals)?;
                    // if link(temp_head)<>null then
                    if link!($globals, temp_head) != null {
                        // begin link(p):=link(temp_head); p:=q;
                        link!($globals, $p) = link!($globals, temp_head);
                        $p = $q;
                        // end;
                    }
                    // end;
                }
                // end;
            }
        }
        // done2: x_token
        'done2 <-
        }
        x_token($globals)?;
        // end
        use crate::section_0162::temp_head;
        use crate::section_0209::max_command;
        use crate::section_0210::the;
        use crate::section_0341::get_next;
        use crate::section_0366::expand;
        use crate::section_0381::x_token;
        use crate::section_0465::the_toks;
    }}
}
