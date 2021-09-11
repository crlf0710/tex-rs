//! @ An empty line is appended at the end of a |read_file|.
//! @^empty line at end of file@>
//
// @<Input the next line of |read_file[m]|@>=
pub(crate) macro Input_the_next_line_of_read_file_m($globals:expr, $m:expr) {{
    // begin if not input_ln(read_file[m],true) then
    if !input_ln(
        make_globals_io_view!($globals),
        &mut $globals.read_file[$m.get()],
        true,
    ) {
        todo!("close");
        // begin a_close(read_file[m]); read_open[m]:=closed;
        // if align_state<>1000000 then
        //   begin runaway;
        //   print_err("File ended within "); print_esc("read");
        // @.File ended within \\read@>
        //   help1("This \read has unbalanced braces.");
        //   align_state:=1000000; limit:=0; error;
        //   end;
        // end;
    }
    // end
    use crate::section_0004::make_globals_io_view;
    use crate::section_0004::TeXGlobalsIoView;
    use crate::section_0031::input_ln;
}}
