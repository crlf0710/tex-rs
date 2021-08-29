//! @* \[53] Extensions.
//! The program above includes a bunch of ``hooks'' that allow further
//! capabilities to be added without upsetting \TeX's basic structure.
//! Most of these hooks are concerned with ``whatsit'' nodes, which are
//! intended to be used for special purposes; whenever a new extension to
//! \TeX\ involves a new kind of whatsit node, a corresponding change needs
//! to be made to the routines below that deal with such nodes,
//! but it will usually be unnecessary to make many changes to the
//! other parts of this program.
//!
//! In order to demonstrate how extensions can be made, we shall treat
//! `\.{\\write}', `\.{\\openout}', `\.{\\closeout}', `\.{\\immediate}',
//! `\.{\\special}', and `\.{\\setlanguage}' as if they were extensions.
//! These commands are actually primitives of \TeX, and they should
//! appear in all implementations of the system; but let's try to imagine
//! that they aren't. Then the program below illustrates how a person
//! could add them.
//!
//! Sometimes, of course, an extension will require changes to \TeX\ itself;
//! no system of hooks could be complete enough for all conceivable extensions.
//! The features associated with `\.{\\write}' are almost all confined to the
//! following paragraphs, but there are small parts of the |print_ln| and
//! |print_char| procedures that were introduced specifically to \.{\\write}
//! characters. Furthermore one of the token lists recognized by the scanner
//! is a |write_text|; and there are a few other miscellaneous places where we
//! have already provided for some aspect of \.{\\write}.  The goal of a \TeX\
//! extender should be to minimize alterations to the standard parts of the
//! program, and to avoid them completely if possible. He or she should also
//! be quite sure that there's no easy way to accomplish the desired goals
//! with the standard features that \TeX\ already has. ``Think thrice before
//! extending,'' because that may save a lot of work, and it will also keep
//! incompatible extensions of \TeX\ from proliferating.
//! @^system dependencies@>
//! @^extensions to \TeX@>
//!
