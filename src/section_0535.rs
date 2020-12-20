//! @ Sometimes |open_log_file| is called at awkward moments when \TeX\ is
//! unable to print error messages or even to |show_context|.
//! The |prompt_file_name| routine can result in a |fatal_error|, but the |error|
//! routine will not be invoked because |log_opened| will be false.
//!
//! The normal idea of |batch_mode| is that nothing at all should be written
//! on the terminal. However, in the unusual case that
//! no log file could be opened, we make an exception and allow
//! an explanatory message to be seen.
//!
//! Incidentally, the program always refers to the log file as a `\.{transcript
//! file}', because some systems cannot use the extension `\.{.log}' for
//! this file.
//!
//! @<Try to get a different log file name@>=
//! begin selector:=term_only;
//! prompt_file_name("transcript file name",".log");
//! end
//!
