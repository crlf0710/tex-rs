//! @* \[20] Token lists.
//! A \TeX\ token is either a character or a control sequence, and it is
//! @^token@>
//! represented internally in one of two ways: (1)~A character whose ASCII
//! code number is |c| and whose command code is |m| is represented as the
//! number $2^8m+c$; the command code is in the range |1<=m<=14|. (2)~A control
//! sequence whose |eqtb| address is |p| is represented as the number
//! |cs_token_flag+p|. Here |cs_token_flag=@t$2^{12}-1$@>| is larger than
//! $2^8m+c$, yet it is small enough that |cs_token_flag+p< max_halfword|;
//! thus, a token fits comfortably in a halfword.
//!
//! A token |t| represents a |left_brace| command if and only if
//! |t<left_brace_limit|; it represents a |right_brace| command if and only if
//! we have |left_brace_limit<=t<right_brace_limit|; and it represents a |match| or
//! |end_match| command if and only if |match_token<=t<=end_match_token|.
//! The following definitions take care of these token-oriented constants
//! and a few others.

// @d cs_token_flag==@'7777 {amount added to the |eqtb| location in a
//   token that stands for a control sequence; is a multiple of~256, less~1}
#[cfg(not(feature = "unicode_support"))]
pub(crate) const cs_token_flag: cur_tok_type = cur_tok_type::new(0o7777);
#[cfg(feature = "unicode_support")]
pub(crate) const cs_token_flag: cur_tok_type = cur_tok_type::new(0x0FFF_FFFF);

// @d left_brace_token=@'0400 {$2^8\cdot|left_brace|$}
// @d left_brace_limit=@'1000 {$2^8\cdot(|left_brace|+1)$}
// @d right_brace_token=@'1000 {$2^8\cdot|right_brace|$}
// @d right_brace_limit=@'1400 {$2^8\cdot(|right_brace|+1)$}
/// `cur_tok_type_cmd_multiplier`·(`right_brace` + 1)
pub(crate) const right_brace_limit: cur_tok_type_repr = cur_tok_type_cmd_multiplier * (right_brace + 1) as cur_tok_type_repr;
// @d math_shift_token=@'1400 {$2^8\cdot|math_shift|$}
// @d tab_token=@'2000 {$2^8\cdot|tab_mark|$}
// @d out_param_token=@'2400 {$2^8\cdot|out_param|$}
// @d space_token=@'5040 {$2^8\cdot|spacer|+|" "|$}
/// `cur_tok_type_cmd_multiplier`·`spacer`+`" "`
pub(crate) const space_token: cur_tok_type_repr = cur_tok_type_cmd_multiplier * spacer as cur_tok_type_repr + b' ' as cur_tok_type_repr;
// @d letter_token=@'5400 {$2^8\cdot|letter|$}
/// `cur_tok_type_cmd_multiplier`·`letter`
pub(crate) const letter_token: cur_tok_type_repr = cur_tok_type_cmd_multiplier * letter as cur_tok_type_repr;
// @d other_token=@'6000 {$2^8\cdot|other_char|$}
/// `cur_tok_type_cmd_multiplier`·`other_char`
pub(crate) const other_token: cur_tok_type_repr = cur_tok_type_cmd_multiplier * other_char as cur_tok_type_repr;
// @d match_token=@'6400 {$2^8\cdot|match|$}
// @d end_match_token=@'7000 {$2^8\cdot|end_match|$}
/// `cur_tok_type_cmd_multiplier`·`end_match`
pub(crate) const end_match_token: cur_tok_type_repr = cur_tok_type_cmd_multiplier * end_match as cur_tok_type_repr;

use crate::section_0297::cur_tok_type;
use crate::section_0297::cur_tok_type_repr;
use crate::section_0297::cur_tok_type_cmd_multiplier;
use crate::section_0207::letter;
use crate::section_0207::other_char;
use crate::section_0207::right_brace;
use crate::section_0207::spacer;
use crate::section_0207::end_match;