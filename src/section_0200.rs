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
        info!($globals, $val)
    }
}

// @p procedure delete_token_ref(@!p:pointer); {|p| points to the reference count
//   of a token list that is losing one reference}
// begin if token_ref_count(p)=null then flush_list(p)
// else decr(token_ref_count(p));
// end;
//
