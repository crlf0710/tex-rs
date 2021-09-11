//! @* \[14] Copying boxes.
//! Another recursive operation that acts on boxes is sometimes needed: The
//! procedure |copy_node_list| returns a pointer to another node list that has
//! the same structure and meaning as the original. Note that since glue
//! specifications and token lists have reference counts, we need not make
//! copies of them. Reference counts can never get too large to fit in a
//! halfword, since each pointer to a node is in a different memory address,
//! and the total number of memory addresses fits in a halfword.
//! @^recursion@>
//! @^reference counts@>
//!
//! (Well, there actually are also references from outside |mem|; if the
//! |save_stack| is made arbitrarily large, it would theoretically be possible
//! to break \TeX\ by overflowing a reference count. But who would want to do that?)
//
// @d add_token_ref(#)==incr(token_ref_count(#)) {new reference to a token list}
/// new reference to a token list
pub(crate) macro add_token_ref($globals:expr, $token:expr) {
    crate::section_0016::incr!(crate::section_0200::token_ref_count!($globals, $token))
}
// @d add_glue_ref(#)==incr(glue_ref_count(#)) {new reference to a glue spec}
/// new reference to a glue spec
#[allow(unused_macros)]
pub(crate) macro add_glue_ref($globals:expr, $glue:expr) {
    crate::section_0016::incr!(crate::section_0150::glue_ref_count!($globals, $glue))
}
