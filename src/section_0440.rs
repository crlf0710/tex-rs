//! @ The |scan_int| routine is used also to scan the integer part of a
//! fraction; for example, the `\.3' in `\.{3.14159}' will be found by
//! |scan_int|. The |scan_dimen| routine assumes that |cur_tok=point_token|
//! after the integer part of such a fraction has been scanned by |scan_int|,
//! and that the decimal point has been backed up to be scanned again.
//
// @p procedure scan_int; {sets |cur_val| to an integer}
/// sets `cur_val` to an integer
#[allow(unused_variables)]
pub(crate) fn scan_int(globals: &mut TeXGlobals) {
// label done;
// var negative:boolean; {should the answer be negated?}
// @!m:integer; {|@t$2^{31}$@> div radix|, the threshold of danger}
// @!d:small_number; {the digit just scanned}
// @!vacuous:boolean; {have no digits appeared?}
// @!OK_so_far:boolean; {has an error message been issued?}
// begin radix:=0; OK_so_far:=true;@/
// @<Get the next non-blank non-sign token; set |negative| appropriately@>;
// if cur_tok=alpha_token then @<Scan an alphabetic character code into |cur_val|@>
// else if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
//   scan_something_internal(int_val,false)
// else @<Scan a numeric constant@>;
// if negative then negate(cur_val);
// end;
    todo!();
}

use crate::section_0004::TeXGlobals;
