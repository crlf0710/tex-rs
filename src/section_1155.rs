//! @ The |set_math_char| procedure creates a new noad appropriate to a given
//! math code, and appends it to the current mlist. However, if the math code
//! is sufficiently large, the |cur_chr| is treated as an active character and
//! nothing is appended.
//
// @<Declare act...@>=
// procedure set_math_char(@!c:integer);
pub(crate) fn set_math_char(globals: &mut TeXGlobals, c: integer) -> TeXResult<()> {
    // var p:pointer; {the new noad}
    // begin if c>=@'100000 then
    if c > 0o100000 {
        // @<Treat |cur_chr|...@>
        todo!("treat cur_chr");
    }
    // else  begin p:=new_noad; math_type(nucleus(p)):=math_char;
    else {
        /// the new noad
        let p: pointer;
        p = new_noad(globals)?;
        math_type!(globals, nucleus!(p)) = math_char as _;
        // character(nucleus(p)):=qi(c mod 256);
        // fam(nucleus(p)):=(c div 256) mod 16;
        // if c>=var_code then
        //   begin if fam_in_range then fam(nucleus(p)):=cur_fam;
        //   type(p):=ord_noad;
        //   end
        // else  type(p):=ord_noad+(c div @'10000);
        // link(tail):=p; tail:=p;
        // end;
        todo!("");
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0681::math_char;
use crate::section_0686::new_noad;