//! @ The integer parameters should really be initialized by a macro package;
//! the following initialization does the minimum to keep \TeX\ from
//! complete failure.
//! @^null delimiter@>
//
// @<Initialize table entries...@>=
pub(crate) macro Initialize_table_entries_done_by_initex_only_0240($globals:expr) {{
    let globals = &mut *$globals;
    // for k:=int_base to del_code_base-1 do eqtb[k].int:=0;
    // mag:=1000; tolerance:=10000; hang_after:=1; max_dead_cycles:=25;
    mag!(globals) = 1000;
    tolerance!(globals) = 10000;
    hang_after!(globals) = 1;
    max_dead_cycles!(globals) = 25;
    // escape_char:="\"; end_line_char:=carriage_return;
    escape_char!(globals) = b'\\'.into();
    end_line_char!(globals) = carriage_return.into();
    // for k:=0 to 255 do del_code(k):=-1;
    // del_code("."):=0; {this null delimiter is used in error recovery}
}}

use crate::section_0004::TeXGlobals;
use crate::section_0022::carriage_return;
use crate::section_0236::end_line_char;
use crate::section_0236::escape_char;
use crate::section_0236::hang_after;
use crate::section_0236::mag;
use crate::section_0236::max_dead_cycles;
use crate::section_0236::tolerance;
