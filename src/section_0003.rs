//! ` `
//!
//! @ Different \PASCAL s have slightly different conventions, and the present
//! @!@:PASCAL H}{\ph@>
//! program expresses \TeX\ in terms of the \PASCAL\ that was
//! available to the author in 1982. Constructions that apply to
//! this particular compiler, which we shall call \ph, should help the
//! reader see how to make an appropriate interface for other systems
//! if necessary. (\ph\ is Charles Hedrick's modification of a compiler
//! @^Hedrick, Charles Locke@>
//! for the DECsystem-10 that was originally developed at the University of
//! Hamburg; cf.\ {\sl SOFTWARE---Practice \AM\ Experience \bf6} (1976),
//! 29--42. The \TeX\ program below is intended to be adaptable, without
//! extensive changes, to most other versions of \PASCAL, so it does not fully
//! use the admirable features of \ph. Indeed, a conscious effort has been
//! made here to avoid using several idiosyncratic features of standard
//! \PASCAL\ itself, so that most of the code can be translated mechanically
//! into other high-level languages. For example, the `\&{with}' and `\\{new}'
//! features are not used, nor are pointer types, set types, or enumerated
//! scalar types; there are no `\&{var}' parameters, except in the case of files;
//! there are no tag fields on variant records; there are no assignments
//! |real:=integer|; no procedures are declared local to other procedures.)
//!
//! The portions of this program that involve system-dependent code, where
//! changes might be necessary because of differences between \PASCAL\ compilers
//! and/or differences between
//! operating systems, can be identified by looking at the sections whose
//! numbers are listed under `system dependencies' in the index. Furthermore,
//! the index entries for `dirty \PASCAL' list all places where the restrictions
//! of \PASCAL\ have not been followed perfectly, for one reason or another.
//! @!@^system dependencies@>
//! @!@^dirty \PASCAL@>
//!
//! Incidentally, \PASCAL's standard |round| function can be problematical,
//! because it disagrees with the IEEE floating-point standard.
//! Many implementors have
//! therefore chosen to substitute their own home-grown rounding procedure.

migration_complete!();
