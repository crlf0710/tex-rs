//! @ \TeX\ doesn't know exactly what to expect when |scan_something_internal|
//! begins.  For example, an integer or dimension or glue value could occur
//! immediately after `\.{\\hskip}'; and one can even say \.{\\the} with
//! respect to token lists in constructions like
//! `\.{\\xdef\\o\{\\the\\output\}}'.  On the other hand, only integers are
//! allowed after a construction like `\.{\\count}'. To handle the various
//! possibilities, |scan_something_internal| has a |level| parameter, which
//! tells the ``highest'' kind of quantity that |scan_something_internal| is
//! allowed to produce. Six levels are distinguished, namely |int_val|,
//! |dimen_val|, |glue_val|, |mu_val|, |ident_val|, and |tok_val|.
//!
//! The output of |scan_something_internal| (and of the other routines
//! |scan_int|, |scan_dimen|, and |scan_glue| below) is put into the global
//! variable |cur_val|, and its level is put into |cur_val_level|. The highest
//! values of |cur_val_level| are special: |mu_val| is used only when
//! |cur_val| points to something in a ``muskip'' register, or to one of the
//! three parameters \.{\\thinmuskip}, \.{\\medmuskip}, \.{\\thickmuskip};
//! |ident_val| is used only when |cur_val| points to a font identifier;
//! |tok_val| is used only when |cur_val| points to |null| or to the reference
//! count of a token list. The last two cases are allowed only when
//! |scan_something_internal| is called with |level=tok_val|.
//!
//! If the output is glue, |cur_val| will point to a glue specification, and
//! the reference count of that glue will have been updated to reflect this
//! reference; if the output is a nonempty token list, |cur_val| will point to
//! its reference count, but in this case the count will not have been updated.
//! Otherwise |cur_val| will contain the integer or scaled value in question.
//
// @d int_val=0 {integer values}
// @d dimen_val=1 {dimension values}
// @d glue_val=2 {glue specifications}
// @d mu_val=3 {math glue specifications}
// @d ident_val=4 {font identifier}
// @d tok_val=5 {token lists}
#[doc(hidden)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub(crate) enum cur_val_level_kind {
    /// integer values
    int_val = 0,
    /// dimension values
    dimen_val = 1,
    /// glue specifications
    glue_val = 2,
    /// math glue specifications
    mu_val = 3,
    /// font identifier
    ident_val = 4,
    /// token lists
    tok_val = 5,
}

impl From<u8> for cur_val_level_kind {
    fn from(val: u8) -> Self {
        use cur_val_level_kind::*;
        if val == int_val as u8 {
            return int_val;
        }
        if val == dimen_val as u8 {
            return dimen_val;
        }
        if val == glue_val as u8 {
            return glue_val;
        }
        if val == mu_val as u8 {
            return mu_val;
        }
        if val == ident_val as u8 {
            return ident_val;
        }
        if val == tok_val as u8 {
            return tok_val;
        }
        unreachable!()
    }
}

#[doc(inline)]
pub(crate) use cur_val_level_kind::*;

//
// @<Glob...@>=
// @!cur_val:integer; {value returned by numeric scanners}
/// value returned by numeric scanners
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_val: integer = 0;
// @!cur_val_level:int_val..tok_val; {the ``level'' of this value}
/// the "level" of this value
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_val_level: cur_val_level_kind = cur_val_level_kind::int_val;

#[globals_struct_use(TeXGlobals)]
use crate::section_0410::cur_val_level_kind;


#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
use crate::section_0101::small_number;
