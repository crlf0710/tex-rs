//! @ When a character of type |spacer| gets through, its character code is
//! changed to $\.{"\ "}=@'40$. This means that the ASCII codes for tab and space,
//! and for the space inserted at the end of a line, will
//! be treated alike when macro parameters are being matched. We do this
//! since such characters are indistinguishable on most computer terminal displays.
//!
//! @<Finish line, emit a space@>=
//! begin loc:=limit+1; cur_cmd:=spacer; cur_chr:=" ";
//! end
