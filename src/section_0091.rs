//! @ A dozen or so error messages end with a parenthesized integer, so we
//! save a teeny bit of program space by declaring the following procedure:
//!
//! @p procedure int_error(@!n:integer);
//! begin print(" ("); print_int(n); print_char(")"); error;
//! end;
//!
