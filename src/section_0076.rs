//! @ A global variable |deletions_allowed| is set |false| if the |get_next|
//! routine is active when |error| is called; this ensures that |get_next|
//! and related routines like |get_token| will never be called recursively.
//! A similar interlock is provided by |set_box_allowed|.
//! @^recursion@>
//!
//! The global variable |history| records the worst level of error that
//! has been detected. It has four possible values: |spotless|, |warning_issued|,
//! |error_message_issued|, and |fatal_error_stop|.
//!
//! Another global variable, |error_count|, is increased by one when an
//! |error| occurs without an interactive dialog, and it is reset to zero at
//! the end of every paragraph.  If |error_count| reaches 100, \TeX\ decides
//! that there is no point in continuing further.

// @d spotless=0 {|history| value when nothing has been amiss yet}
// @d warning_issued=1 {|history| value when |begin_diagnostic| has been called}
// @d error_message_issued=2 {|history| value when |error| has been called}
// @d fatal_error_stop=3 {|history| value when termination was premature}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub(crate) enum history_kind {
    /// `history` value when nothing has been amiss yet
    spotless = 0,
    /// `history` value when `begin_diagnostic` has been called
    warning_issued = 1,
    /// `history` value when `error` has been called
    error_message_issued = 2,
    /// `history` value when termination was premature
    fatal_error_stop = 3,
}

#[doc(inline)]
pub(crate) use history_kind::*;

// @<Glob...@>=
// @!deletions_allowed:boolean; {is it safe for |error| to call |get_token|?}
// @!set_box_allowed:boolean; {is it safe to do a \.{\\setbox} assignment?}

// @!history:spotless..fatal_error_stop; {has the source input been clean so far?}
#[globals_struct_field(TeXGlobals)]
/// has the source input been clean so far?
pub(crate) static history: history_kind = fatal_error_stop;

// @!error_count:-1..100; {the number of scrolled errors since the
//   last paragraph ended}
//

#[globals_struct_use(TeXGlobals)]
use crate::section_0076::history_kind::{self, fatal_error_stop};

use globals_struct::{globals_struct_field, globals_struct_use};
