//! @ The following procedure, which prints out the decimal representation of a
//! given integer |n|, has been written carefully so that it works properly
//! if |n=0| or if |(-n)| would cause overflow. It does not apply |mod| or |div|
//! to negative arguments, since such operations are not implemented consistently
//! by all \PASCAL\ compilers.
//
// @<Basic print...@>=
// procedure print_int(@!n:integer); {prints an integer in decimal form}
/// prints an integer in decimal form
#[allow(unused_variables)]
pub(crate) fn print_int(globals: &mut TeXGlobals, n: integer) {
    // var k:0..23; {index to current digit; we assume that $|n|<10^{23}$}
    // @!m:integer; {used to negate |n| in possibly dangerous cases}
    // begin k:=0;
    // if n<0 then
    //   begin print_char("-");
    //   if n>-100000000 then negate(n)
    //   else  begin m:=-1-n; n:=m div 10; m:=(m mod 10)+1; k:=1;
    //     if m<10 then dig[0]:=m
    //     else  begin dig[0]:=0; incr(n);
    //       end;
    //     end;
    //   end;
    // repeat dig[k]:=n mod 10; n:=n div 10; incr(k);
    // until n=0;
    // print_the_digs(k);
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
