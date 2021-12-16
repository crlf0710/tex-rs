//! @ Roman numerals are produced by the |print_roman_int| routine.  Readers
//! who like puzzles might enjoy trying to figure out how this tricky code
//! works; therefore no explanation will be given. Notice that 1990 yields
//! \.{mcmxc}, not \.{mxm}.
//
// @p procedure print_roman_int(@!n:integer);
pub(crate) fn print_roman_int(globals: &mut TeXGlobals, mut n: integer) {
    // label exit;
    // var j,@!k: pool_pointer; {mysterious indices into |str_pool|}
    /// mysterious indices into `CHARS`
    let (mut j, mut k);
    // @!u,@!v: nonnegative_integer; {mysterious numbers}
    /// mysterious numbers
    let (mut u, mut v);
    // begin j:=str_start["m2d5c2l5x2v5i"]; v:=1000;
    const CHARS: &[u8] = b" m2d5c2l5x2v5i";
    j = 1;
    v = 1000;
    // loop@+  begin while n>=v do
    loop {
        while n >= v {
            // begin print_char(so(str_pool[j])); n:=n-v;
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code::from(CHARS[j] as integer),
            );
            n -= v;
            // end;
        }
        // if n<=0 then return; {nonpositive input produces no output}
        /// nonpositive input produces no output
        if n <= 0 {
            return;
        }
        // k:=j+2; u:=v div (so(str_pool[k-1])-"0");
        k = j + 2;
        u = v / ((CHARS[k - 1] - b'0') as integer);
        // if str_pool[k-1]=si("2") then
        if CHARS[k - 1] == b'2' {
            // begin k:=k+2; u:=u div (so(str_pool[k-1])-"0");
            k += 2;
            u = u / ((CHARS[k - 1] - b'0') as integer);
            // end;
        }
        // if n+u>=v then
        if n + u >= v {
            // begin print_char(so(str_pool[k])); n:=n+u;
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code::from(CHARS[k] as integer),
            );
            n += u;
            // end
        }
        // else  begin j:=j+2; v:=v div (so(str_pool[j-1])-"0");
        else {
            j += 2;
            v = v / ((CHARS[j - 1] - b'0') as integer);
            // end;
        }
        // end;
    }
    // exit:end;
}

use crate::pascal::integer;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0058::print_char;
