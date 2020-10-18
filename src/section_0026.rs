//! @ Most of what we need to do with respect to input and output can be handled
//! by the I/O facilities that are standard in \PASCAL, i.e., the routines
//! called |get|, |put|, |eof|, and so on. But
//! standard \PASCAL\ does not allow file variables to be associated with file
//! names that are determined at run time, so it cannot be used to implement
//! \TeX; some sort of extension to \PASCAL's ordinary |reset| and |rewrite|
//! is crucial for our purposes. We shall assume that |name_of_file| is a variable
//! of an appropriate type such that the \PASCAL\ run-time system being used to
//! implement \TeX\ can open a file whose external name is specified by
//! |name_of_file|.
//! @^system dependencies@>
//!
//! @<Glob...@>=
//! @!name_of_file:packed array[1..file_name_size] of char;@;@/
//!   {on some systems this may be a \&{record} variable}
//! @!name_length:0..file_name_size;@/{this many characters are actually
//!   relevant in |name_of_file| (the rest are blank)}
//!
