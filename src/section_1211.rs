//! @ If the user says, e.g., `\.{\\global\\global}', the redundancy is
//! silently accepted.

macro_rules! Assignments {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {
        if false {
            unreachable!();
        /*} else if Assignments_xxx {
            todo!();
        */} else if Assignments_1232!($globals, $cur_cmd, $a) {
            /// already processed
            do_nothing!();
            true
        /*} else if Assignments_xxx {
            todo!();
        */} else {
            false
        }
    };
}

// @<Declare act...@>=
// @t\4@>@<Declare subprocedures for |prefixed_command|@>@t@>@;@/
// procedure prefixed_command;
#[allow(unused_variables)]
pub(crate) fn prefixed_command(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // label done,exit;
    // var a:small_number; {accumulated prefix codes so far}
    // @!f:internal_font_number; {identifies a font}
    // @!j:halfword; {index into a \.{\\parshape} specification}
    // @!k:font_index; {index into |font_info|}
    // @!p,@!q:pointer; {for temporary short-term use}
    // @!n:integer; {ditto}
    // @!e:boolean; {should a definition be expanded? or was \.{\\let} not done?}
    // begin a:=0;
    let mut a =0;
    // while cur_cmd=prefix do
    while globals.cur_cmd == prefix {
        //   begin if not odd(a div cur_chr) then a:=a+cur_chr;
        if !(a / globals.cur_chr.get()).is_odd() {
            a += globals.cur_chr.get();
        }
        // @<Get the next non-blank non-relax...@>;
        Get_the_next_non_blank_non_relax_non_call_token!(globals);
        // if cur_cmd<=max_non_prefixed_command then
        if globals.cur_cmd <= max_non_prefixed_command {
            // @<Discard erroneous prefixes and |return|@>;
            todo!();
            // end;
        }
    }
    // @<Discard the prefixes \.{\\long} and \.{\\outer} if they are irrelevant@>;
    // @<Adjust \(f)for the setting of \.{\\globaldefs}@>;
    // case cur_cmd of
    // @t\4@>@<Assignments@>@;
    if Assignments!(globals, globals.cur_cmd, a) {
        /// already processed
        do_nothing!();
    }
    // othercases confusion("prefix")
    else {
        confusion(strpool_str!("prefix"));
        // @:this can't happen prefix}{\quad prefix@>
        // endcases;
    }
    // done: @<Insert a token saved by \.{\\afterassignment}, if any@>;
    // exit:end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::IsOddOrEven;
use crate::section_0004::TeXGlobals;
use crate::section_0095::confusion;
use crate::section_0209::*;
use crate::section_0210::*;
use crate::section_0208::max_non_prefixed_command;
use crate::section_0081::JumpOutToEndOfTEX;