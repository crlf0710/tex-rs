//! @ The |input_ln| function brings the next line of input from the specified
//! file into available positions of the buffer array and returns the value
//! |true|, unless the file has already been entirely read, in which case it
//! returns |false| and sets |last:=first|.  In general, the |ASCII_code|
//! numbers that represent the next line of the file are input into
//! |buffer[first]|, |buffer[first+1]|, \dots, |buffer[last-1]|; and the
//! global variable |last| is set equal to |first| plus the length of the
//! line. Trailing blanks are removed from the line; thus, either |last=first|
//! (in which case the line was entirely blank) or |buffer[last-1]<>" "|.
//!
//! An overflow error is given, however, if the normal actions of |input_ln|
//! would make |last>=buf_size|; this is done so that other parts of \TeX\
//! can safely look at the contents of |buffer[last+1]| without overstepping
//! the bounds of the |buffer| array. Upon entry to |input_ln|, the condition
//! |first<buf_size| will always hold, so that there is always room for an
//! ``empty'' line.
//!
//! The variable |max_buf_stack|, which is used to keep track of how large
//! the |buf_size| parameter must be to accommodate the present job, is
//! also kept up to date by |input_ln|.
//!
//! If the |bypass_eoln| parameter is |true|, |input_ln| will do a |get|
//! before looking at the first character of the line; this skips over
//! an |eoln| that was in |f^|. The procedure does not do a |get| when it
//! reaches the end of the line; therefore it can be used to acquire input
//! from the user's terminal as well as from ordinary text files.
//!
//! Standard \PASCAL\ says that a file should have |eoln| immediately
//! before |eof|, but \TeX\ needs only a weaker restriction: If |eof|
//! occurs in the middle of a line, the system function |eoln| should return
//! a |true| result (even though |f^| will be undefined).
//!
//! Since the inner loop of |input_ln| is part of \TeX's ``inner loop''---each
//! character of input comes in at this place---it is wise to reduce system
//! overhead by making use of special routines that read in an entire array
//! of characters at once, if such routines are available. The following
//! code uses standard \PASCAL\ to illustrate what needs to be done, but
//! finer tuning is often possible at well-developed \PASCAL\ sites.
//! @^inner loop@>

// @p function input_ln(var f:alpha_file;@!bypass_eoln:boolean):boolean;
//   {inputs the next line or returns |false|}
/// inputs the next line or returns `false`
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn input_ln(
    globals_view: TeXGlobalsIoView<'_>,
    f: &mut alpha_file,
    bypass_eoln: boolean,
) -> boolean {
    // var last_nonblank:0..buf_size; {|last| with trailing blanks removed}
    /// `last` with trailing blanks removed
    let mut last_nonblank: u16_from_0_to_n<buf_size_TYPENUM>;
    let input_ln;
    // begin if bypass_eoln then if not eof(f) then get(f);
    //   {input the first character of the line into |f^|}
    /// input the first character of the line into `f^`
    if bypass_eoln {
        if !eof(f) {
            get(f);
        }
    }
    // last:=first; {cf.\ Matthew 19\thinspace:\thinspace30}
    *globals_view.last = *globals_view.first;
    // if eof(f) then input_ln:=false
    if eof(f) {
        input_ln = false;
    }
    // else  begin last_nonblank:=first;
    else {
        last_nonblank = *globals_view.first;
        // while not eoln(f) do
        while !eoln(f) {
            // begin if last>=max_buf_stack then
            if globals_view.last >= globals_view.max_buf_stack {
                // begin max_buf_stack:=last+1;
                *globals_view.max_buf_stack = *globals_view.last + 1;
                // if max_buf_stack=buf_size then
                if *globals_view.max_buf_stack == buf_size {
                    // @<Report overflow of the input buffer, and abort@>;
                    todo!("report overflow");
                }
                // end;
            }
            // buffer[last]:=xord[f^]; get(f); incr(last);
            globals_view.buffer[*globals_view.last] = xord(buffer_variable(f));
            get(f);
            incr!(*globals_view.last);
            // if buffer[last-1]<>" " then last_nonblank:=last;
            if globals_view.buffer[*globals_view.last - 1] != ASCII_code_literal!(b' ') {
                last_nonblank = *globals_view.last;
            }
            // end;
        }
        // last:=last_nonblank; input_ln:=true;
        *globals_view.last = last_nonblank;
        crate::trace_expr!(
            "input_ln: buffer[{:?}..{:?}] = {:?}",
            &globals_view.first,
            &globals_view.last,
            &globals_view.buffer[*globals_view.first..*globals_view.last]
        );
        input_ln = true;
        // end;
    }
    // end;
    input_ln
}

use crate::io_support::{buffer_variable, eof, eoln, get};
use crate::pascal::boolean;
use crate::pascal::u16_from_0_to_n;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0011::buf_size;
use crate::section_0011::buf_size_TYPENUM;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code_literal;
use crate::section_0020::xord;
use crate::section_0025::alpha_file;
