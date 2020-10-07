//! @* \[6] Reporting errors.
//! When something anomalous is detected, \TeX\ typically does something like this:
//! $$\vbox{\halign{#\hfil\cr
//! |print_err("Something anomalous has been detected");|\cr
//! |help3("This is the first line of my offer to help.")|\cr
//! |("This is the second line. I'm trying to")|\cr
//! |("explain the best way for you to proceed.");|\cr
//! |error;|\cr}}$$
//! A two-line help message would be given using |help2|, etc.; these informal
//! helps should use simple vocabulary that complements the words used in the
//! official error message that was printed. (Outside the U.S.A., the help
//! messages should preferably be translated into the local vernacular. Each
//! line of help is at most 60 characters long, in the present implementation,
//! so that |max_print_line| will not be exceeded.)
//!
//! The |print_err| procedure supplies a `\.!' before the official message,
//! and makes sure that the terminal is awake if a stop is going to occur.
//! The |error| procedure supplies a `\..' after the official message, then it
//! shows the location of the error; and if |interaction=error_stop_mode|,
//! it also enters into a dialog with the user, during which time the help
//! message may be printed.
//! @^system dependencies@>
//!
