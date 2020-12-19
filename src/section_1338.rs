//! @* \[52] Debugging.
//! Once \TeX\ is working, you should be able to diagnose most errors with
//! the \.{\\show} commands and other diagnostic features. But for the initial
//! stages of debugging, and for the revelation of really deep mysteries, you
//! can compile \TeX\ with a few more aids, including the \PASCAL\ runtime
//! checks and its debugger. An additional routine called |debug_help|
//! will also come into play when you type `\.D' after an error message;
//! |debug_help| also occurs just before a fatal error causes \TeX\ to succumb.
//! @^debugging@>
//! @^system dependencies@>
//!
//! The interface to |debug_help| is primitive, but it is good enough when used
//! with a \PASCAL\ debugger that allows you to set breakpoints and to read
//! variables and change their values. After getting the prompt `\.{debug \#}', you
//! type either a negative number (this exits |debug_help|), or zero (this
//! goes to a location where you can set a breakpoint, thereby entering into
//! dialog with the \PASCAL\ debugger), or a positive number |m| followed by
//! an argument |n|. The meaning of |m| and |n| will be clear from the
//! program below. (If |m=13|, there is an additional argument, |l|.)
//! @.debug \#@>
//!
//! @d breakpoint=888 {place where a breakpoint is desirable}
//!
//! @<Last-minute...@>=
//! @!debug procedure debug_help; {routine to display various things}
/// routine to display various things
#[cfg(feature = "debugging")]
#[allow(unused_variables)]
pub(crate) fn debug_help(globals: &mut TeXGlobals) {
    // label breakpoint,exit;
    // var k,@!l,@!m,@!n:integer;
    // begin loop begin wake_up_terminal;
    loop {
        wake_up_terminal(globals);
        // print_nl("debug # (-1 to exit):"); update_terminal;
        print_nl(globals, strpool_str!("debug # (-1 to exit):"));
        update_terminal(globals);
        // @.debug \#@>
        // read(term_in,m);
        // if m<0 then return
        // else if m=0 then
        //   begin goto breakpoint;@\ {go to every label at least once}
        //   breakpoint: m:=0; @{'BREAKPOINT'@}@\
        //   end
        // else  begin read(term_in,n);
        //   case m of
        //   @t\4@>@<Numbered cases for |debug_help|@>@;
        //   othercases print("?")
        //   endcases;
        //   end;
        // end;
    }
    // exit:end;
}
// gubed

use crate::section_0004::TeXGlobals;
use crate::section_0034::wake_up_terminal;
use crate::section_0034::update_terminal;
use crate::section_0062::print_nl;