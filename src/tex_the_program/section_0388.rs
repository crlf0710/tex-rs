//! @ The parameters, if any, must be scanned before the macro is expanded.
//! Parameters are token lists without reference counts. They are placed on
//! an auxiliary stack called |pstack| while they are being scanned, since
//! the |param_stack| may be losing entries during the matching process.
//! (Note that |param_stack| can't be gaining entries, since |macro_call| is
//! the only routine that puts anything onto |param_stack|, and it
//! is not recursive.)

// @<Glob...@>=
// @!pstack:array[0..8] of pointer; {arguments supplied to a macro}
/// arguments supplied to a macro
#[globals_struct_field(TeXGlobals)]
pub(crate) static pstack: pstack_array<pointer> = pstack_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0388::pstack_array;

type pstack_array_LENGTH_TYPENUM = typenum::op!(U8 + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) pstack_array[u8_from_0_to_n<U8>] => u8; U8; pstack_array_LENGTH_TYPENUM
);

use crate::pascal::u8_from_0_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::Unsigned;
use typenum::{U1, U8};
