//! ` `

// @ @d bad_pool(#)==begin wake_up_terminal; write_ln(term_out,#);
macro_rules! bad_pool {
    ($globals:expr, $get_strings_started:expr, $val:expr) => {
        wake_up_terminal($globals);
        write_ln(&mut $globals.term_out, $val);
        // a_close(pool_file); get_strings_started:=false; return;
        a_close(&mut $globals.pool_file);
        $get_strings_started = false;
        return $get_strings_started;
        // end
        use crate::section_0034::wake_up_terminal;
        use crate::pascal::write_ln;
    }
}

// @<Read the other strings...@>=
macro_rules! Read_the_other_strings_from_the_TEX_POOL_file_and_return_true_or_give_an_error_message_and_return_false {
    ($globals:expr, $get_strings_started:expr, $g:expr) => {
        trace_span!("Read the other strings...");
        // name_of_file:=pool_name; {we needn't set |name_length|}
        /// we needn't set `name_length`
        $globals.name_of_file.assign_str(pool_name);
        // if a_open_in(pool_file) then
        if a_open_in(make_globals_filename_view!($globals), &mut $globals.pool_file) {
            // begin c:=false;
            let mut c = false;
            // repeat @<Read one string, but return |false| if the
            //   string memory space is getting too tight for comfort@>;
            loop {
                Read_one_string_but_return_false_if_the_string_memory_space_is_getting_too_tight_for_comfort!
                    ($globals, c, $g);
                // until c;
                if c {
                    break;
                }
            }
            // a_close(pool_file); get_strings_started:=true;
            a_close(&mut $globals.pool_file);
            $get_strings_started = true;
            // end
            // else  bad_pool('! I can''t read TEX.POOL.')
        } else {
            bad_pool!($globals, $get_strings_started, "! I can''t read TEX.POOL.");
        }
        // @.I can't read TEX.POOL@>
        use crate::section_0004::TeXGlobalsFilenameView;
        use crate::section_0011::pool_name;
        use crate::section_0027::a_open_in;
        use crate::section_0028::a_close;
    };
}
