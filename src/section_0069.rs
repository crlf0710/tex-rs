//! @ Roman numerals are produced by the |print_roman_int| routine.  Readers
//! who like puzzles might enjoy trying to figure out how this tricky code
//! works; therefore no explanation will be given. Notice that 1990 yields
//! \.{mcmxc}, not \.{mxm}.
//
// @p procedure print_roman_int(@!n:integer);
#[allow(unused_variables)]
pub(crate) fn print_roman_int(globals: &mut TeXGlobals, n: integer) {
    todo!();
    // label exit;
    // var j,@!k: pool_pointer; {mysterious indices into |str_pool|}
    // @!u,@!v: nonnegative_integer; {mysterious numbers}
    // begin j:=str_start["m2d5c2l5x2v5i"]; v:=1000;
    // loop@+  begin while n>=v do
    //     begin print_char(so(str_pool[j])); n:=n-v;
    //     end;
    //   if n<=0 then return; {nonpositive input produces no output}
    //   k:=j+2; u:=v div (so(str_pool[k-1])-"0");
    //   if str_pool[k-1]=si("2") then
    //     begin k:=k+2; u:=u div (so(str_pool[k-1])-"0");
    //     end;
    //   if n+u>=v then
    //     begin print_char(so(str_pool[k])); n:=n+u;
    //     end
    //   else  begin j:=j+2; v:=v div (so(str_pool[j-1])-"0");
    //     end;
    //   end;
    // exit:end;
}

use crate::section_0004::TeXGlobals;
use crate::pascal::integer;
