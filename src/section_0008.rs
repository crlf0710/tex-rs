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
const _ : () = ();


// FIXME: Using short name to workaround https://github.com/dtolnay/linkme/issues/35
#[distributed_slice]
pub(crate) static INIT_TBLENTRY: [fn(&mut TeXGlobals)] = [..];

macro_rules! Initialize_table_entries_done_by_INITEX_only {
    ($globals:expr) => {
        for f in crate::section_0008::INIT_TBLENTRY {
            f($globals);
        }
    }
}

// @<Initialize whatever...@>=
macro_rules! Initialize_whatever_TeX_might_access {
    ($globals:expr) => {
        // @<Set initial values of key variables@>@/
        // @!init @<Initialize table entries (done by \.{INITEX} only)@>@;@+tini
        region_initex! {
            ($globals) {
                Initialize_table_entries_done_by_INITEX_only!($globals);
            }
        }
    }
}

use linkme::distributed_slice;
use crate::section_0004::TeXGlobals;
