//! @* \[3] Input and output.
//! The bane of portability is the fact that different operating systems treat
//! input and output quite differently, perhaps because computer scientists
//! have not given sufficient attention to this problem. People have felt somehow
//! that input and output are not part of ``real'' programming. Well, it is true
//! that some kinds of programming are more fun than others. With existing
//! input/output conventions being so diverse and so messy, the only sources of
//! joy in such parts of the code are the rare occasions when one can find a
//! way to make the program a little less bad than it might have been. We have
//! two choices, either to attack I/O now and get it over with, or to postpone
//! I/O until near the end. Neither prospect is very attractive, so let's
//! get it over with.
//!
//! The basic operations we need to do are (1)~inputting and outputting of
//! text, to or from a file or the user's terminal; (2)~inputting and
//! outputting of eight-bit bytes, to or from a file; (3)~instructing the
//! operating system to initiate (``open'') or to terminate (``close'') input or
//! output from a specified file; (4)~testing whether the end of an input
//! file has been reached.
//!
//! \TeX\ needs to deal with two kinds of files.
//! We shall use the term |alpha_file| for a file that contains textual data,
//! and the term |byte_file| for a file that contains eight-bit binary information.
//! These two types turn out to be the same on many computers, but
//! sometimes there is a significant distinction, so we shall be careful to
//! distinguish between them. Standard protocols for transferring
//! such files from computer to computer, via high-speed networks, are
//! now becoming available to more and more communities of users.
//!
//! The program actually makes use also of a third kind of file, called a
//! |word_file|, when dumping and reloading base information for its own
//! initialization.  We shall define a word file later; but it will be possible
//! for us to specify simple operations on word files before they are defined.
// @<Types...@>=
// @!eight_bits=0..255; {unsigned one-byte quantity}

/// unsigned one-byte quantity
pub(crate) type eight_bits = u8;

// @!alpha_file=packed file of text_char; {files that contain textual data}

pub(crate) type alpha_file = packed_file_of_text_char;

// @!byte_file=packed file of eight_bits; {files that contain binary data}

pub(crate) type byte_file = packed_file_of<eight_bits>;

use crate::io_support::{packed_file_of, packed_file_of_text_char};
