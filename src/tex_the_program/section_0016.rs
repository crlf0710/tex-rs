//! @ Here are some macros for common programming idioms.

// @d incr(#) == #:=#+1 {increase a variable by unity}
/// increase a variable by unity
pub(crate) macro incr($val:expr) {
    $val += 1
}
// @d decr(#) == #:=#-1 {decrease a variable by unity}
/// decrease a variable by unity
#[allow(unused_macros)]
pub(crate) macro decr($val:expr) {
    $val -= 1
}
// @d negate(#) == #:=-# {change the sign of a variable}
/// change the sign of a variable
pub(crate) macro negate($val:expr) {
    $val = -$val
}
// @d loop == @+ while true do@+ {repeat over and over until a |goto| happens}
// @f loop == xclause
//   {\.{WEB}'s |xclause| acts like `\ignorespaces|while true do|\unskip'}
// @d do_nothing == {empty statement}
/// empty statement
pub(crate) macro do_nothing() {}
// @d return == goto exit {terminate a procedure call}
// @f return == nil
// @d empty=0 {symbolic name for a null constant}
