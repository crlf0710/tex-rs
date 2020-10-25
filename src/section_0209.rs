//! @ The next codes are special; they all relate to mode-independent
//! assignment of values to \TeX's internal registers or tables.
//! Codes that are |max_internal| or less represent internal quantities
//! that might be expanded by `\.{\\the}'.
//
// @d toks_register=71 {token list register ( \.{\\toks} )}
// @d assign_toks=72 {special token list ( \.{\\output}, \.{\\everypar}, etc.~)}
// @d assign_int=73 {user-defined integer ( \.{\\tolerance}, \.{\\day}, etc.~)}
// @d assign_dimen=74 {user-defined length ( \.{\\hsize}, etc.~)}
// @d assign_glue=75 {user-defined glue ( \.{\\baselineskip}, etc.~)}
// @d assign_mu_glue=76 {user-defined muglue ( \.{\\thinmuskip}, etc.~)}
// @d assign_font_dimen=77 {user-defined font dimension ( \.{\\fontdimen} )}
// @d assign_font_int=78 {user-defined font integer ( \.{\\hyphenchar},
//   \.{\\skewchar} )}
// @d set_aux=79 {specify state info ( \.{\\spacefactor}, \.{\\prevdepth} )}
// @d set_prev_graf=80 {specify state info ( \.{\\prevgraf} )}
// @d set_page_dimen=81 {specify state info ( \.{\\pagegoal}, etc.~)}
// @d set_page_int=82 {specify state info ( \.{\\deadcycles},
//   \.{\\insertpenalties} )}
// @d set_box_dimen=83 {change dimension of box ( \.{\\wd}, \.{\\ht}, \.{\\dp} )}
// @d set_shape=84 {specify fancy paragraph shape ( \.{\\parshape} )}
// @d def_code=85 {define a character code ( \.{\\catcode}, etc.~)}
// @d def_family=86 {declare math fonts ( \.{\\textfont}, etc.~)}
// @d set_font=87 {set current font ( font identifiers )}
// @d def_font=88 {define a font file ( \.{\\font} )}
// @d register=89 {internal register ( \.{\\count}, \.{\\dimen}, etc.~)}
// @d max_internal=89 {the largest code that can follow \.{\\the}}
// @d advance=90 {advance a register or parameter ( \.{\\advance} )}
// @d multiply=91 {multiply a register or parameter ( \.{\\multiply} )}
// @d divide=92 {divide a register or parameter ( \.{\\divide} )}
// @d prefix=93 {qualify a definition ( \.{\\global}, \.{\\long}, \.{\\outer} )}
// @d let=94 {assign a command code ( \.{\\let}, \.{\\futurelet} )}
// @d shorthand_def=95 {code definition ( \.{\\chardef}, \.{\\countdef}, etc.~)}
// @d read_to_cs=96 {read into a control sequence ( \.{\\read} )}
// @d def=97 {macro definition ( \.{\\def}, \.{\\gdef}, \.{\\xdef}, \.{\\edef} )}
// @d set_box=98 {set a box ( \.{\\setbox} )}
// @d hyph_data=99 {hyphenation data ( \.{\\hyphenation}, \.{\\patterns} )}
// @d set_interaction=100 {define level of interaction ( \.{\\batchmode}, etc.~)}
// @d max_command=100 {the largest command code seen at |big_switch|}
/// the largest command code seen at `big_switch`
pub(crate) const max_command: quarterword = max_command_TYPENUM::U8;
pub(crate) type max_command_TYPENUM = U100;
pub(crate) type max_command_POS_TYPENUM = P100;

use crate::section_0113::quarterword;
use typenum::Unsigned;
use typenum::{P100, U100};
