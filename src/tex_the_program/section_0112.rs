//! @ The operation of adding or subtracting |min_quarterword| occurs quite
//! frequently in \TeX, so it is convenient to abbreviate this operation
//! by using the macros |qi| and |qo| for input and output to and from
//! quarterword format.
//!
//! The inner loop of \TeX\ will run faster with respect to compilers
//! that don't optimize expressions like `|x+0|' and `|x-0|', if these
//! macros are simplified in the obvious way when |min_quarterword=0|.
//! @^inner loop@>@^system dependencies@>
//
// @d qi(#)==#+min_quarterword
//   {to put an |eight_bits| item into a quarterword}
/// to put an `eight_bits` item into a quarterword
pub(crate) macro qi($val:expr) {
    $val + crate::section_0110::min_quarterword
}
// @d qo(#)==#-min_quarterword
//   {to take an |eight_bits| item out of a quarterword}
/// to take an `eight_bits` item out of a quarterword
pub(crate) macro qo($val:expr) {
    $val - crate::section_0110::min_quarterword
}
// @d hi(#)==#+min_halfword
//   {to put a sixteen-bit item into a halfword}
/// to put a sixteen-bit item into a halfword
pub(crate) macro hi($val:expr) {
    $val + crate::section_0110::min_halfword
}
// @d ho(#)==#-min_halfword
//   {to take a sixteen-bit item from a halfword}
//
