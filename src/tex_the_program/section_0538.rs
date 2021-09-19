//! @ Here we have to remember to tell the |input_ln| routine not to
//! start with a |get|. If the file is empty, it is considered to
//! contain a single blank line.
//! @^system dependencies@>
//! @^empty line at end of file@>

// @<Read the first line...@>=
pub(crate) macro Read_the_first_line_of_the_new_file($globals:expr) {
    // begin line:=1;
    $globals.line = 1;
    // if input_ln(cur_file,false) then do_nothing;
    if input_ln(
        make_globals_io_view!($globals),
        &mut cur_file!($globals),
        false,
    ) {
        do_nothing!();
    }
    // firm_up_the_line;
    firm_up_the_line($globals);
    // if end_line_char_inactive then decr(limit)
    if end_line_char_inactive!($globals) {
        decr!(limit!($globals));
    }
    // else  buffer[limit]:=end_line_char;
    else {
        $globals.buffer[limit!($globals)] = end_line_char!($globals).into();
    }
    // first:=limit+1; loc:=start;
    $globals.first = (limit!($globals) + 1).into();
    loc!($globals) = start!($globals);
    // end

    use crate::section_0004::make_globals_io_view;
    use crate::section_0004::TeXGlobalsIoView;
    use crate::section_0016::decr;
    use crate::section_0016::do_nothing;
    use crate::section_0031::input_ln;
    use crate::section_0036::loc;
    use crate::section_0236::end_line_char;
    use crate::section_0302::limit;
    use crate::section_0302::start;
    use crate::section_0304::cur_file;
    use crate::section_0360::end_line_char_inactive;
    use crate::section_0363::firm_up_the_line;
}
