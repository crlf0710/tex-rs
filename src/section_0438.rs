//! @ An integer number can be preceded by any number of spaces and `\.+' or
//! `\.-' signs. Then comes either a decimal constant (i.e., radix 10), an
//! octal constant (i.e., radix 8, preceded by~\.\'), a hexadecimal constant
//! (radix 16, preceded by~\."), an alphabetic constant (preceded by~\.\`), or
//! an internal variable. After scanning is complete,
//! |cur_val| will contain the answer, which must be at most
//! $2^{31}-1=2147483647$ in absolute value. The value of |radix| is set to
//! 10, 8, or 16 in the cases of decimal, octal, or hexadecimal constants,
//! otherwise |radix| is set to zero. An optional space follows a constant.
//
// @d octal_token=other_token+"'" {apostrophe, indicates an octal constant}
/// apostrophe, indicates an octal constant
pub(crate) const octal_token: cur_tok_type_repr = other_token + b'\'' as cur_tok_type_repr;
// @d hex_token=other_token+"""" {double quote, indicates a hex constant}
/// double quote, indicates a hex constant
pub(crate) const hex_token: cur_tok_type_repr = other_token + b'\"' as cur_tok_type_repr;
// @d alpha_token=other_token+"`" {reverse apostrophe, precedes alpha constants}
/// reverse apostrophe, precedes alpha constants
pub(crate) const alpha_token: cur_tok_type_repr = other_token + b'`' as cur_tok_type_repr;
// @d point_token=other_token+"." {decimal point}
// @d continental_point_token=other_token+"," {decimal point, Eurostyle}
//
// @<Glob...@>=
// @!radix:small_number; {|scan_int| sets this to 8, 10, 16, or zero}
/// `scan_int` sets this to 8, 10, 16, or zero
#[globals_struct_field(TeXGlobals)]
pub(crate) static radix: small_number = small_number::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::small_number;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
use crate::section_0297::cur_tok_type_repr;
use crate::section_0289::other_token;

