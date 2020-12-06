//! @ Individual lines of help are recorded in the array |help_line|, which
//! contains entries in positions |0..(help_ptr-1)|. They should be printed
//! in reverse order, i.e., with |help_line[0]| appearing last.
//
// @d hlp1(#)==help_line[0]:=#;@+end
// @d hlp2(#)==help_line[1]:=#; hlp1
// @d hlp3(#)==help_line[2]:=#; hlp2
// @d hlp4(#)==help_line[3]:=#; hlp3
// @d hlp5(#)==help_line[4]:=#; hlp4
// @d hlp6(#)==help_line[5]:=#; hlp5
// @d help0==help_ptr:=0 {sometimes there might be no help}
/// sometimes there might be no help
macro_rules! help0 {
    ($globals:expr) => {{
        $globals.help_ptr = 0.into();
    }}
}
// @d help1==@+begin help_ptr:=1; hlp1 {use this with one help line}
/// use this with one help line
macro_rules! help1 {
    ($globals:expr, $val1:expr) => {{
        $globals.help_ptr = 1.into();
        $globals.help_line[0] = $val1;
    }}
}
// @d help2==@+begin help_ptr:=2; hlp2 {use this with two help lines}
/// use this with two help lines
macro_rules! help2 {
    ($globals:expr, $val1:expr, $val2:expr) => {{
        $globals.help_ptr = 2.into();
        $globals.help_line[0] = $val1;
        $globals.help_line[0] = $val2;
    }}
}
// @d help3==@+begin help_ptr:=3; hlp3 {use this with three help lines}
// @d help4==@+begin help_ptr:=4; hlp4 {use this with four help lines}
// @d help5==@+begin help_ptr:=5; hlp5 {use this with five help lines}
// @d help6==@+begin help_ptr:=6; hlp6 {use this with six help lines}
//
// @<Glob...@>=
// @!help_line:array[0..5] of str_number; {helps for the next |error|}
/// helps for the next `error`
#[globals_struct_field(TeXGlobals)]
pub(crate) static help_line: help_line_array<str_number> = help_line_array::default();
// @!help_ptr:0..6; {the number of help lines present}
/// the number of help lines present
#[globals_struct_field(TeXGlobals)]
pub(crate) static help_ptr: u8_from_0_to_n<U6> = u8_from_0_to_n::default();
// @!use_err_help:boolean; {should the |err_help| list be shown?}
/// should the `err_help` list be shown?
#[globals_struct_field(TeXGlobals)]
pub(crate) static use_err_help: boolean = boolean::default();

#[globals_struct_use(TeXGlobals)]
use typenum::U6;

#[globals_struct_use(TeXGlobals)]
use crate::section_0079::help_line_array;

type help_line_array_LENGTH_TYPENUM = typenum::op!(U5 - U0 + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) help_line_array[u8_from_0_to_n<U5>] =>
    u8; U8; U0; help_line_array_LENGTH_TYPENUM
);

use crate::pascal::u8_from_0_to_n;
use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::{U0, U1, U5};
