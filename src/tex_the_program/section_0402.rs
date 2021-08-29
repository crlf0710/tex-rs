//! @* \[26] Basic scanning subroutines.
//! Let's turn now to some procedures that \TeX\ calls upon frequently to digest
//! certain kinds of patterns in the input. Most of these are quite simple;
//! some are quite elaborate. Almost all of the routines call |get_x_token|,
//! which can cause them to be invoked recursively.
//! @^stomach@>
//! @^recursion@>
//!
