//! @ The |scan_int| routine is used also to scan the integer part of a
//! fraction; for example, the `\.3' in `\.{3.14159}' will be found by
//! |scan_int|. The |scan_dimen| routine assumes that |cur_tok=point_token|
//! after the integer part of such a fraction has been scanned by |scan_int|,
//! and that the decimal point has been backed up to be scanned again.
//
// @p procedure scan_int; {sets |cur_val| to an integer}
/// sets `cur_val` to an integer
#[allow(unused_variables)]
#[allow(unused_assignments, non_snake_case)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn scan_int(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label done;
    // var negative:boolean; {should the answer be negated?}
    /// should the answer be negated?
    let mut negative: boolean;
    // @!m:integer; {|@t$2^{31}$@> div radix|, the threshold of danger}
    // @!d:small_number; {the digit just scanned}
    // @!vacuous:boolean; {have no digits appeared?}
    // @!OK_so_far:boolean; {has an error message been issued?}
    /// has an error message been issued?
    let OK_so_far: boolean;
    // begin radix:=0; OK_so_far:=true;@/
    globals.radix = 0.into();
    OK_so_far = true;
    // @<Get the next non-blank non-sign token; set |negative| appropriately@>;
    Set_the_next_non_blank_non_sign_token__set_negative_appropriately!(globals, negative);
    // if cur_tok=alpha_token then @<Scan an alphabetic character code into |cur_val|@>
    if globals.cur_tok.get() == alpha_token {
        Scan_an_alphabetic_character_code_into_cur_val!(globals);
    }
    // else if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
    else if globals.cur_cmd >= min_internal && globals.cur_cmd <= max_internal {
        // scan_something_internal(int_val,false)
        scan_something_internal(globals, (int_val as u8).into(), false)?;
    }
    // else @<Scan a numeric constant@>;
    else {
        Scan_a_numeric_constant!(globals);
    }
    // if negative then negate(cur_val);
    if negative {
        negate!(globals.cur_val);
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::pascal::boolean;
use crate::section_0208::min_internal;
use crate::section_0209::max_internal;
use crate::section_0438::alpha_token;
use crate::section_0410::int_val;
use crate::section_0413::scan_something_internal;

