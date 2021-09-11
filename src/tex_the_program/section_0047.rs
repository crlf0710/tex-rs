//! @ The initial values of |str_pool|, |str_start|, |pool_ptr|,
//! and |str_ptr| are computed by the \.{INITEX} program, based in part
//! on the information that \.{WEB} has output while processing \TeX.
//! @.INITEX@>
//! @^string pool@>
//
// @p @!init function get_strings_started:boolean; {initializes the string pool,
//   but returns |false| if something goes wrong}
/// initializes the string pool, but returns `false` if something goes wrong
#[cfg(feature = "initex")]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn get_strings_started(globals: &mut TeXGlobals) -> boolean {
    let get_strings_started;
    // label done,exit;
    // var k,@!l:0..255; {small indices or counters}
    // @!m,@!n:text_char; {characters input from |pool_file|}
    // @!g:str_number; {garbage}
    /// garbage
    let mut g: str_number;
    // @!a:integer; {accumulator for check sum}
    // @!c:boolean; {check sum has been checked}
    // begin pool_ptr:=0; str_ptr:=0; str_start[0]:=0;
    globals.pool_ptr = pool_pointer::zero();
    globals.str_ptr = str_number::zero();
    globals.str_start[0] = pool_pointer::zero();
    // @<Make the first 256 strings@>;
    Make_the_first_256_strings!(globals, g);
    // @<Read the other strings from the \.{TEX.POOL} file and return |true|,
    //   or give an error message and return |false|@>;
    crate::section_0051::Read_the_other_strings_from_the_TEX_POOL_file_and_return_true_or_give_an_error_message_and_return_false!(
        globals,
        get_strings_started,
        g
    );
    // exit:end;
    // tini
    get_strings_started
}

#[cfg(feature = "initex")]
use crate::pascal::boolean;
#[cfg(feature = "initex")]
use crate::section_0004::TeXGlobals;
#[cfg(feature = "initex")]
use crate::section_0038::pool_pointer;
#[cfg(feature = "initex")]
use crate::section_0038::str_number;
#[cfg(feature = "initex")]
use crate::section_0048::Make_the_first_256_strings;
