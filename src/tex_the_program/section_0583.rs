//! @* \[31] Device-independent file format.
//! The most important output produced by a run of \TeX\ is the ``device
//! independent'' (\.{DVI}) file that specifies where characters and rules
//! are to appear on printed pages. The form of these files was designed by
//! David R. Fuchs in 1979. Almost any reasonable typesetting device can be
//! @^Fuchs, David Raymond@>
//! @:DVI_files}{\.{DVI} files@>
//! driven by a program that takes \.{DVI} files as input, and dozens of such
//! \.{DVI}-to-whatever programs have been written. Thus, it is possible to
//! print the output of \TeX\ on many different kinds of equipment, using \TeX\
//! as a device-independent ``front end.''
//!
//! A \.{DVI} file is a stream of 8-bit bytes, which may be regarded as a
//! series of commands in a machine-like language. The first byte of each command
//! is the operation code, and this code is followed by zero or more bytes
//! that provide parameters to the command. The parameters themselves may consist
//! of several consecutive bytes; for example, the `|set_rule|' command has two
//! parameters, each of which is four bytes long. Parameters are usually
//! regarded as nonnegative integers; but four-byte-long parameters,
//! and shorter parameters that denote distances, can be
//! either positive or negative. Such parameters are given in two's complement
//! notation. For example, a two-byte-long distance parameter has a value between
//! $-2^{15}$ and $2^{15}-1$. As in \.{TFM} files, numbers that occupy
//! more than one byte position appear in BigEndian order.
//!
//! A \.{DVI} file consists of a ``preamble,'' followed by a sequence of one
//! or more ``pages,'' followed by a ``postamble.'' The preamble is simply a
//! |pre| command, with its parameters that define the dimensions used in the
//! file; this must come first.  Each ``page'' consists of a |bop| command,
//! followed by any number of other commands that tell where characters are to
//! be placed on a physical page, followed by an |eop| command. The pages
//! appear in the order that \TeX\ generated them. If we ignore |nop| commands
//! and \\{fnt\_def} commands (which are allowed between any two commands in
//! the file), each |eop| command is immediately followed by a |bop| command,
//! or by a |post| command; in the latter case, there are no more pages in the
//! file, and the remaining bytes form the postamble.  Further details about
//! the postamble will be explained later.
//!
//! Some parameters in \.{DVI} commands are ``pointers.'' These are four-byte
//! quantities that give the location number of some other byte in the file;
//! the first byte is number~0, then comes number~1, and so on. For example,
//! one of the parameters of a |bop| command points to the previous |bop|;
//! this makes it feasible to read the pages in backwards order, in case the
//! results are being directed to a device that stacks its output face up.
//! Suppose the preamble of a \.{DVI} file occupies bytes 0 to 99. Now if the
//! first page occupies bytes 100 to 999, say, and if the second
//! page occupies bytes 1000 to 1999, then the |bop| that starts in byte 1000
//! points to 100 and the |bop| that starts in byte 2000 points to 1000. (The
//! very first |bop|, i.e., the one starting in byte 100, has a pointer of~$-1$.)
//!
