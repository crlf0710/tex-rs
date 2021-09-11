//! @ Users occasionally want to interrupt \TeX\ while it's running.
//! If the \PASCAL\ runtime system allows this, one can implement
//! a routine that sets the global variable |interrupt| to some nonzero value
//! when such an interrupt is signalled. Otherwise there is probably at least
//! a way to make |interrupt| nonzero using the \PASCAL\ debugger.
//! @^system dependencies@>
//! @^debugging@>
//
// @d check_interrupt==begin if interrupt<>0 then pause_for_instructions;
//   end
pub(crate) macro check_interrupt($globals:expr) {
    if $globals.interrupt != 0 {
        pause_for_instructions($globals);
    }

    use crate::section_0098::pause_for_instructions;
}
//
// @<Global...@>=
// @!interrupt:integer; {should \TeX\ pause for instructions?}
/// should `TeX` pause for instructions?
#[globals_struct_field(TeXGlobals)]
pub(crate) static interrupt: integer = 0;
// @!OK_to_interrupt:boolean; {should interrupts be observed?}
/// should interrupts be observed?
#[globals_struct_field(TeXGlobals)]
pub(crate) static OK_to_interrupt: boolean = true;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
