//! @ First, however, we shall consider two non-recursive procedures that do
//! simpler tasks. The first of these, |delete_token_ref|, is called when
//! a pointer to a token list's reference count is being removed. This means
//! that the token list should disappear if the reference count was |null|,
//! otherwise the count should be decreased by one.
//! @^reference counts@>

// @d token_ref_count(#) == info(#) {reference count preceding a token list}
/// reference count preceding a token list
macro_rules! token_ref_count {
    ($globals:expr, $val:expr) => {
        info_inner!($globals, $val)
    };
}

// @p procedure delete_token_ref(@!p:pointer); {|p| points to the reference count
//   of a token list that is losing one reference}

/// `p` points to the reference count of a token list that is losing one reference
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn delete_token_ref(globals: &mut TeXGlobals, p: pointer) {
    // begin if token_ref_count(p)=null then flush_list(p)
    if token_ref_count!(globals, p) == null {
        flush_list(globals, p);
    }
    // else decr(token_ref_count(p));
    else {
        decr!(token_ref_count!(globals, p));
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0123::flush_list;

