//! ` `
//!
//! @ The present implementation has a long ancestry, beginning in the summer
//! of~1977, when Michael~F. Plass and Frank~M. Liang designed and coded
//! a prototype
//! @^Plass, Michael Frederick@>
//! @^Liang, Franklin Mark@>
//! @^Knuth, Donald Ervin@>
//! based on some specifications that the author had made in May of that year.
//! This original proto\TeX\ included macro definitions and elementary
//! manipulations on boxes and glue, but it did not have line-breaking,
//! page-breaking, mathematical formulas, alignment routines, error recovery,
//! or the present semantic nest; furthermore,
//! it used character lists instead of token lists, so that a control sequence
//! like \.{\\halign} was represented by a list of seven characters. A
//! complete version of \TeX\ was designed and coded by the author in late
//! 1977 and early 1978; that program, like its prototype, was written in the
//! {\mc SAIL} language, for which an excellent debugging system was
//! available. Preliminary plans to convert the {\mc SAIL} code into a form
//! somewhat like the present ``web'' were developed by Luis Trabb~Pardo and
//! @^Trabb Pardo, Luis Isidoro@>
//! the author at the beginning of 1979, and a complete implementation was
//! created by Ignacio~A. Zabala in 1979 and 1980. The \TeX82 program, which
//! @^Zabala Salelles, Ignacio Andr\'es@>
//! was written by the author during the latter part of 1981 and the early
//! part of 1982, also incorporates ideas from the 1979 implementation of
//! @^Guibas, Leonidas Ioannis@>
//! @^Sedgewick, Robert@>
//! @^Wyatt, Douglas Kirk@>
//! \TeX\ in {\mc MESA} that was written by Leonidas Guibas, Robert Sedgewick,
//! and Douglas Wyatt at the Xerox Palo Alto Research Center.  Several hundred
//! refinements were introduced into \TeX82 based on the experiences gained with
//! the original implementations, so that essentially every part of the system
//! has been substantially improved. After the appearance of ``Version 0'' in
//! September 1982, this program benefited greatly from the comments of
//! many other people, notably David~R. Fuchs and Howard~W. Trickey.
//! A final revision in September 1989 extended the input character set to
//! eight-bit codes and introduced the ability to hyphenate words from
//! different languages, based on some ideas of Michael~J. Ferguson.
//! @^Fuchs, David Raymond@>
//! @^Trickey, Howard Wellington@>
//! @^Ferguson, Michael John@>
//!
//! No doubt there still is plenty of room for improvement, but the author
//! is firmly committed to keeping \TeX82 ``frozen'' from now on; stability
//! and reliability are to be its main virtues.
//!
//! On the other hand, the \.{WEB} description can be extended without changing
//! the core of \TeX82 itself, and the program has been designed so that such
//! extensions are not extremely difficult to make.
//! The |banner| string defined here should be changed whenever \TeX\
//! undergoes any modifications, so that it will be clear which version of
//! \TeX\ might be the guilty party when a problem arises.
//! @^extensions to \TeX@>
//! @^system dependencies@>
//!
//! If this program is changed, the resulting system should not be called
//! `\TeX'; the official name `\TeX' by itself is reserved
//! for software systems that are fully compatible with each other.
//! A special test suite called the ``\.{TRIP} test'' is available for
//! helping to determine whether a particular implementation deserves to be
//! known as `\TeX' [cf.~Stanford Computer Science report CS1027,
//! November 1984].

// @d banner=='This is TeX, Version 3.141592653' {printed when \TeX\ starts}

/// printed when `TeX` starts
pub(crate) const banner: &'static str = "This is TeX-rs, Version 3.141592653";

crate::migration_complete!();
