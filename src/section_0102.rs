//! @ The following function is used to create a scaled integer from a given decimal
//! fraction $(.d_0d_1\ldots d_{k-1})$, where |0<=k<=17|. The digit $d_i$ is
//! given in |dig[i]|, and the calculation produces a correctly rounded result.
//
// @p function round_decimals(@!k:small_number) : scaled;
//   {converts a decimal fraction}
pub(crate) fn round_decimals(globals: &mut TeXGlobals, mut k: small_number) -> scaled {
    // var a:integer; {the accumulator}
    /// the accumulator
    let mut a: integer;
    // begin a:=0;
    a = 0;
    // while k>0 do
    while k > 0 {
        // begin decr(k); a:=(a+dig[k]*two) div 10;
        decr!(k);
        a = (a + globals.dig[k.get() as usize].get() as integer * two.inner()) / 10;
        // end;
    }
    // round_decimals:=(a+1) div 2;
    scaled::new_from_inner((a + 1) / 2)
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0101::two;
