//! @ When a character of type |spacer| gets through, its character code is
//! changed to $\.{"\ "}=@'40$. This means that the ASCII codes for tab and space,
//! and for the space inserted at the end of a line, will
//! be treated alike when macro parameters are being matched. We do this
//! since such characters are indistinguishable on most computer terminal displays.
//
// @<Finish line, emit a space@>=
macro_rules! Finish_line__emit_a_space {
    ($globals:expr) => {
        // begin loc:=limit+1; cur_cmd:=spacer; cur_chr:=" ";
        loc!($globals) = limit!($globals) + 1;
        $globals.cur_cmd = spacer;
        $globals.cur_chr = ASCII_code_literal!(b' ').into();
        // end
    }
}