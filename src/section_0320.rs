//! @ Here is the missing piece of |show_token_list| that is activated when the
//! token beginning line~2 is about to be shown:
//
// @<Do magic computation@>=set_trick_count
macro_rules! Do_magic_computation {
    ($globals:expr) => {{
        set_trick_count!($globals);
    }}
}

