//! ` `

// @<Input and store tokens from the next line of the file@>=
pub(crate) macro Input_and_store_tokens_from_the_next_line_of_the_file {
    ($globals:expr, $m:expr, $p:expr, $q:expr, $n:expr, $r:expr) => {{
        // begin_file_reading; name:=m+1;
        begin_file_reading($globals);
        name!($globals) = $m.get() as halfword + 1;
        // if read_open[m]=closed then @<Input for \.{\\read} from the terminal@>
        if $globals.read_open[$m.get()] == read_open_kind::closed {
            crate::section_0484::Input_for_read_from_the_terminal!($globals, $n, $r);
        }
        // else if read_open[m]=just_open then @<Input the first line of |read_file[m]|@>
        else if $globals.read_open[$m.get()] == read_open_kind::just_open {
            crate::section_0485::Input_the_first_line_of_read_file_m!($globals, $m);
        }
        // else @<Input the next line of |read_file[m]|@>;
        else {
            crate::section_0486::Input_the_next_line_of_read_file_m!($globals, $m);
        }
        // limit:=last;
        limit!($globals) = $globals.last.get();
        // if end_line_char_inactive then decr(limit)
        if end_line_char_inactive!($globals) {
            decr!(limit!($globals));
        }
        // else  buffer[limit]:=end_line_char;
        else {
            $globals.buffer[limit!($globals)] = ASCII_code::from(end_line_char!($globals));
        }
        // first:=limit+1; loc:=start; state:=new_line;@/
        $globals.first = (limit!($globals) + 1).into();
        loc!($globals) = start!($globals);
        state!($globals) = new_line;
        crate::region_forward_label!(
            |'done|
            {
                // loop@+  begin get_token;
                loop {
                    get_token($globals)?;
                    // if cur_tok=0 then goto done;
                    //   {|cur_cmd=cur_chr=0| will occur at the end of the line}
                    /// `cur_cmd=cur_chr=0` will occur at the end of the line
                    if $globals.cur_tok.get() == 0 {
                        crate::goto_forward_label!('done);
                    }
                    // if align_state<1000000 then {unmatched `\.\}' aborts the line}
                    /// unmatched `}` aborts the line
                    if $globals.align_state < 1000000 {
                        // begin repeat get_token; until cur_tok=0;
                        loop {
                            get_token($globals)?;
                            if $globals.cur_tok.get() == 0 {
                                break;
                            }
                        }
                        // align_state:=1000000; goto done;
                        $globals.align_state = 1000000;
                        crate::goto_forward_label!('done);
                        // end;
                    }
                    // store_new_token(cur_tok);
                    store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
                    // end;
                }
            }
            // done: end_file_reading
            'done <-
        );
        end_file_reading($globals);

        use crate::section_0016::decr;
        use crate::section_0018::ASCII_code;
        use crate::section_0036::loc;
        use crate::section_0113::halfword;
        use crate::section_0236::end_line_char;
        use crate::section_0302::limit;
        use crate::section_0302::name;
        use crate::section_0302::state;
        use crate::section_0302::start;
        use crate::section_0303::new_line;
        use crate::section_0328::begin_file_reading;
        use crate::section_0329::end_file_reading;
        use crate::section_0360::end_line_char_inactive;
        use crate::section_0365::get_token;
        use crate::section_0371::store_new_token;
        use crate::section_0480::read_open_kind;
    }}
}
