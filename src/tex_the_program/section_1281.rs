//! @ If \.{\\errmessage} occurs often in |scroll_mode|, without user-defined
//! \.{\\errhelp}, we don't want to give a long help message each time. So we
//! give a verbose explanation only once.
//!
//! @<Glob...@>=
//! @!long_help_seen:boolean; {has the long \.{\\errmessage} help been used?}
//!
