//! @ Here is the missing piece of |show_token_list| that is activated when the
//! token beginning line~2 is about to be shown:
//
// @<Do magic computation@>=set_trick_count
pub(crate) macro Do_magic_computation($globals:expr) {{
    crate::section_0316::set_trick_count!($globals);
}}
