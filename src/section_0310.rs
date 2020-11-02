//! @ Thus, the ``current input state'' can be very complicated indeed; there
//! can be many levels and each level can arise in a variety of ways. The
//! |show_context| procedure, which is used by \TeX's error-reporting routine to
//! print out the current input state on all levels down to the most recent
//! line of characters from an input file, illustrates most of these conventions.
//! The global variable |base_ptr| contains the lowest level that was
//! displayed by this procedure.
//!
//! @<Glob...@>=
//! @!base_ptr:0..stack_size; {shallowest level shown by |show_context|}
//!
