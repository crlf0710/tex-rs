//! @* \[29] File names.
//! It's time now to fret about file names.  Besides the fact that different
//! operating systems treat files in different ways, we must cope with the
//! fact that completely different naming conventions are used by different
//! groups of people. The following programs show what is required for one
//! particular operating system; similar routines for other systems are not
//! difficult to devise.
//! @^fingers@>
//! @^system dependencies@>
//!
//! \TeX\ assumes that a file name has three parts: the name proper; its
//! ``extension''; and a ``file area'' where it is found in an external file
//! system.  The extension of an input file or a write file is assumed to be
//! `\.{.tex}' unless otherwise specified; it is `\.{.log}' on the
//! transcript file that records each run of \TeX; it is `\.{.tfm}' on the font
//! metric files that describe characters in the fonts \TeX\ uses; it is
//! `\.{.dvi}' on the output files that specify typesetting information; and it
//! is `\.{.fmt}' on the format files written by \.{INITEX} to initialize \TeX.
//! The file area can be arbitrary on input files, but files are usually
//! output to the user's current area.  If an input file cannot be
//! found on the specified area, \TeX\ will look for it on a special system
//! area; this special area is intended for commonly used input files like
//! \.{webmac.tex}.
//!
//! Simple uses of \TeX\ refer only to file names that have no explicit
//! extension or area. For example, a person usually says `\.{\\input} \.{paper}'
//! or `\.{\\font\\tenrm} \.= \.{helvetica}' instead of `\.{\\input}
//! \.{paper.new}' or `\.{\\font\\tenrm} \.= \.{<csd.knuth>test}'. Simple file
//! names are best, because they make the \TeX\ source files portable;
//! whenever a file name consists entirely of letters and digits, it should be
//! treated in the same way by all implementations of \TeX. However, users
//! need the ability to refer to other files in their environment, especially
//! when responding to error messages concerning unopenable files; therefore
//! we want to let them use the syntax that appears in their favorite
//! operating system.
//!
//! The following procedures don't allow spaces to be part of
//! file names; but some users seem to like names that are spaced-out.
//! System-dependent changes to allow such things should probably
//! be made with reluctance, and only when an entire file name that
//! includes spaces is ``quoted'' somehow.
//!
