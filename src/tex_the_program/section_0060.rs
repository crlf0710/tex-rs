//! @ Control sequence names, file names, and strings constructed with
//! \.{\\string} might contain |ASCII_code| values that can't
//! be printed using |print_char|. Therefore we use |slow_print| for them:

// @<Basic print...@>=

// procedure slow_print(@!s:integer); {prints string |s|}

/// prints string `s`.
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn slow_print(globals: &mut TeXGlobals, s: integer) {
    // var j:pool_pointer; {current character code position}
    // begin if (s>=str_ptr) or (s<256) then print(s)
    if s >= globals.str_ptr.get() as _ || s < 256 {
        print(globals, s);
    }
    // else begin j:=str_start[s];
    //   while j<str_start[s+1] do
    //     begin print(so(str_pool[j])); incr(j);
    //     end;
    //   end;
    else {
        #[cfg(not(feature = "unicode_support"))]
        {
            let mut j = globals.str_start[s as u32];
            while j < globals.str_start[s as u32 + 1] {
                print_char(so(globals.str_pool[j]));
                incr!(j);
            }
        }
        #[cfg(feature = "unicode_support")]
        {
            let chars = globals
                .str_pool
                .str_ascii_codes(&globals.str_start, str_number::new(s as u32))
                .collect::<Vec<_>>();
            for ch in chars {
                print_char(make_globals_io_string_log_view!(globals), xord(ch));
            }
        }
        // end;
    }
}

use crate::pascal::integer;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0020::xord;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0059::print;
