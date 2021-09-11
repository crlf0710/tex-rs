//! @ This program has two important variations: (1) There is a long and slow
//! version called \.{INITEX}, which does the extra calculations needed to
//! @.INITEX@>
//! initialize \TeX's internal tables; and (2)~there is a shorter and faster
//! production version, which cuts the initialization to a bare minimum.
//! Parts of the program that are needed in (1) but not in (2) are delimited by
//! the codewords `$|init|\ldots|tini|$'.
//
// @d init== {change this to `$\\{init}\equiv\.{@@\{}$' in the production version}
// @d tini== {change this to `$\\{tini}\equiv\.{@@\}}$' in the production version}
// @f init==begin
// @f tini==end
const _: () = ();

pub(crate) macro Set_initial_values_of_key_variables($globals:expr) {
    crate::section_0074::Set_initial_values_of_key_variables_0074!($globals);
    crate::section_0215::Set_initial_values_of_key_variables_0215!($globals);
    crate::section_0254::Set_initial_values_of_key_variables_0254!($globals);
    crate::section_0272::Set_initial_values_of_key_variables_0272!($globals);
    crate::section_0521::Set_initial_values_of_key_variables_0521!($globals);
    crate::section_0593::Set_initial_values_of_key_variables_0593!($globals);
    crate::section_0596::Set_initial_values_of_key_variables_0596!($globals);
}

// FIXME: Using short name to workaround https://github.com/dtolnay/linkme/issues/35
#[distributed_slice]
pub(crate) static INIT_TBLENTRY: [fn(&mut TeXGlobals)] = [..];

pub(crate) macro Initialize_table_entries_done_by_INITEX_only($globals:expr) {
    for f in crate::section_0008::INIT_TBLENTRY {
        f($globals);
    }
}

// @<Initialize whatever...@>=
pub(crate) macro Initialize_whatever_TeX_might_access($globals:expr) {
    // @<Set initial values of key variables@>@/
    Set_initial_values_of_key_variables!($globals);
    // @!init @<Initialize table entries (done by \.{INITEX} only)@>@;@+tini
    crate::region_initex! {
        Initialize_table_entries_done_by_INITEX_only!($globals);
    }
}

use crate::section_0004::TeXGlobals;
use linkme::distributed_slice;
