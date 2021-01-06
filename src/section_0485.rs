//! @ The first line of a file must be treated specially, since |input_ln|
//! must be told not to start with |get|.
//! @^system dependencies@>
//
// @<Input the first line of |read_file[m]|@>=
macro_rules! Input_the_first_line_of_read_file_m {
    ($globals:expr, $m:expr) => {{
        // if input_ln(read_file[m],false) then read_open[m]:=normal
        if input_ln(make_globals_io_view!($globals), &mut $globals.read_file[$m.get()], false) {
            $globals.read_open[$m.get()] = read_open_kind::normal;
        }
        // else  begin a_close(read_file[m]); read_open[m]:=closed;
        else {
            a_close(&mut $globals.read_file[$m.get()]);
            $globals.read_open[$m.get()] = read_open_kind::closed;
        }
        // end
        use crate::section_0004::TeXGlobalsIoView;
        use crate::section_0028::a_close;
        use crate::section_0031::input_ln;
    }}
}
