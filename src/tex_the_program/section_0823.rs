//! @ As the algorithm runs, it maintains a set of six delta-like registers
//! for the length of the line following the first active breakpoint to the
//! current position in the given hlist. When it makes a pass through the
//! active list, it also maintains a similar set of six registers for the
//! length following the active breakpoint of current interest. A third set
//! holds the length of an empty line (namely, the sum of \.{\\leftskip} and
//! \.{\\rightskip}); and a fourth set is used to create new delta nodes.
//!
//! When we pass a delta node we want to do operations like
//! $$\hbox{\ignorespaces|for
//! k:=1 to 6 do cur_active_width[k]:=cur_active_width[k]+mem[q+k].sc|};$$ and we
//! want to do this without the overhead of |for| loops. The |do_all_six|
//! macro makes such six-tuples convenient.
//
// @d do_all_six(#)==#(1);#(2);#(3);#(4);#(5);#(6)
pub(crate) macro do_all_six {
    ($macro_ident:ident !; @globals = $globals:expr) => {{
        $macro_ident!($globals, 1);
        $macro_ident!($globals, 2);
        $macro_ident!($globals, 3);
        $macro_ident!($globals, 4);
        $macro_ident!($globals, 5);
        $macro_ident!($globals, 6);
    }},
    ($macro_ident:ident !; @globals = $globals:expr; $($arg:expr),*) => {{
        $macro_ident!($globals, 1, $($arg),*);
        $macro_ident!($globals, 2, $($arg),*);
        $macro_ident!($globals, 3, $($arg),*);
        $macro_ident!($globals, 4, $($arg),*);
        $macro_ident!($globals, 5, $($arg),*);
        $macro_ident!($globals, 6, $($arg),*);
    }}
}
//
// @<Glob...@>=
// @!active_width:array[1..6] of scaled;
//   {distance from first active node to~|cur_p|}
/// distance from first active node to `cur_p`
#[globals_struct_field(TeXGlobals)]
pub(crate) static active_width: all_six_array<scaled> = all_six_array::default();
// @!cur_active_width:array[1..6] of scaled; {distance from current active node}
/// distance from current active node
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_active_width: all_six_array<scaled> = all_six_array::default();
// @!background:array[1..6] of scaled; {length of an ``empty'' line}
/// length of an "empty" line
#[globals_struct_field(TeXGlobals)]
pub(crate) static background: all_six_array<scaled> = all_six_array::default();
// @!break_width:array[1..6] of scaled; {length being computed after current break}
/// length being computed after current break
#[globals_struct_field(TeXGlobals)]
pub(crate) static break_width: all_six_array<scaled> = all_six_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0823::all_six_array;

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) all_six_array[u8_from_m_to_n<U1, U6>] =>
    u8; U8; U1; U6
);

use crate::pascal::u8_from_m_to_n;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::{Unsigned, U1, U6};
