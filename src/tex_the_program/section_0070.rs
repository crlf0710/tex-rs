//! @ The |print| subroutine will not print a string that is still being
//! created. The following procedure will.
//
// @p procedure print_current_string; {prints a yet-unmade string}
/// prints a yet-unmade string
pub(crate) fn print_current_string(globals: &mut TeXGlobals) {
    #[cfg(not(feature = "unicode_support"))]
    {
        // var j:pool_pointer; {points to current character code}
        // begin j:=str_start[str_ptr];
        // while j<pool_ptr do
        //   begin print_char(so(str_pool[j])); incr(j);
        //   end;
        // end;
        todo!();
    }
    #[cfg(feature = "unicode_support")]
    {
        let range = globals.str_start[globals.str_ptr]..globals.pool_ptr;
        let chars = globals
            .str_pool
            .slice_ascii_codes(range)
            .collect::<Vec<_>>();
        for ch in chars {
            print_char(make_globals_io_string_log_view!(globals), xord(ch));
        }
    }
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0020::xord;
use crate::section_0058::print_char;
