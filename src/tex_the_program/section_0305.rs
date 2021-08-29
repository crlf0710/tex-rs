//! @ Users of \TeX\ sometimes forget to balance left and right braces properly,
//! and one of the ways \TeX\ tries to spot such errors is by considering an
//! input file as broken into subfiles by control sequences that
//! are declared to be \.{\\outer}.
//!
//! A variable called |scanner_status| tells \TeX\ whether or not to complain
//! when a subfile ends. This variable has six possible values:
//!
//! \yskip\hang|normal|, means that a subfile can safely end here without incident.
//!
//! \yskip\hang|skipping|, means that a subfile can safely end here, but not a file,
//! because we're reading past some conditional text that was not selected.
//!
//! \yskip\hang|defining|, means that a subfile shouldn't end now because a
//! macro is being defined.
//!
//! \yskip\hang|matching|, means that a subfile shouldn't end now because a
//! macro is being used and we are searching for the end of its arguments.
//!
//! \yskip\hang|aligning|, means that a subfile shouldn't end now because we are
//! not finished with the preamble of an \.{\\halign} or \.{\\valign}.
//!
//! \yskip\hang|absorbing|, means that a subfile shouldn't end now because we are
//! reading a balanced token list for \.{\\message}, \.{\\write}, etc.
//!
//! \yskip\noindent
//! If the |scanner_status| is not |normal|, the variable |warning_index| points
//! to the |eqtb| location for the relevant control sequence name to print
//! in an error message.
//
// @d skipping=1 {|scanner_status| when passing conditional text}
// @d defining=2 {|scanner_status| when reading a macro definition}
// @d matching=3 {|scanner_status| when reading macro arguments}
// @d aligning=4 {|scanner_status| when reading an alignment preamble}
// @d absorbing=5 {|scanner_status| when reading a balanced text}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub(crate) enum scanner_status_kind {
    normal = 0,
    /// `scanner_status` when passing conditional text
    skipping=1,
    /// `scanner_status` when reading a macro definition
    defining=2,
    /// `scanner_status` when reading macro arguments
    matching=3,
    /// `scanner_status` when reading an alignment preamble
    aligning=4,
    /// `scanner_status` when reading a balanced text
    absorbing=5,
}

pub(crate) use scanner_status_kind::*;

// @<Glob...@>=
// @!scanner_status : normal..absorbing; {can a subfile end now?}
/// can a subfile end now?
#[globals_struct_field(TeXGlobals)]
pub(crate) static scanner_status: scanner_status_kind = scanner_status_kind::normal;

#[globals_struct_use(TeXGlobals)]
use crate::section_0305::scanner_status_kind;

// @!warning_index : pointer; {identifier relevant to non-|normal| scanner status}
/// identifier relevant to non-`normal` scanner status
#[globals_struct_field(TeXGlobals)]
pub(crate) static warning_index: pointer = null;

// @!def_ref : pointer; {reference count of token list being defined}
/// reference count of token list being defined
#[globals_struct_field(TeXGlobals)]
pub(crate) static def_ref: pointer = pointer::default();


use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};

