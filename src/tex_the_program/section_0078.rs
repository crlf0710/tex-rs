//! @ Since errors can be detected almost anywhere in \TeX, we want to declare the
//! error procedures near the beginning of the program. But the error procedures
//! in turn use some other procedures, which need to be declared |forward|
//! before we get to |error| itself.
//!
//! It is possible for |error| to be called recursively if some error arises
//! when |get_token| is being used to delete a token, and/or if some fatal error
//! occurs while \TeX\ is trying to fix a non-fatal one. But such recursion
//! @^recursion@>
//! is never more than two levels deep.
//!
//! @<Error handling...@>=
//! procedure@?normalize_selector; forward;@t\2@>@/
//! procedure@?get_token; forward;@t\2@>@/
//! procedure@?term_input; forward;@t\2@>@/
//! procedure@?show_context; forward;@t\2@>@/
//! procedure@?begin_file_reading; forward;@t\2@>@/
//! procedure@?open_log_file; forward;@t\2@>@/
//! procedure@?close_files_and_terminate; forward;@t\2@>@/
//! procedure@?clear_for_error_prompt; forward;@t\2@>@/
//! procedure@?give_err_help; forward;@t\2@>@/
//! @t\4\hskip-\fontdimen2\font@>@;@+@!debug@+procedure@?debug_help;
//!   forward;@;@+gubed
//!
