//! \[1] Introduction.
//!
//! This is \TeX, a document compiler intended to produce typesetting of high
//! quality.
//! The \PASCAL\ program that follows is the definition of \TeX82, a standard
//! @:PASCAL}{\PASCAL@>
//! @!@:TeX82}{\TeX82@>
//! version of \TeX\ that is designed to be highly portable so that identical output
//! will be obtainable on a great variety of computers.//!
//! The main purpose of the following program is to explain the algorithms of \TeX\
//! as clearly as possible. As a result, the program will not necessarily be very
//! efficient when a particular \PASCAL\ compiler has translated it into a
//! particular machine language. However, the program has been written so that it
//! can be tuned to run efficiently in a wide variety of operating environments
//! by making comparatively few changes. Such flexibility is possible because
//! the documentation that follows is written in the \.{WEB} language, which is
//! at a higher level than \PASCAL; the preprocessing step that converts \.{WEB}
//! to \PASCAL\ is able to introduce most of the necessary refinements.
//! Semi-automatic translation to other languages is also feasible, because the
//! program below does not make extensive use of features that are peculiar to
//! \PASCAL.
//!
//! A large piece of software like \TeX\ has inherent complexity that cannot
//! be reduced below a certain level of difficulty, although each individual
//! part is fairly simple by itself. The \.{WEB} language is intended to make
//! the algorithms as readable as possible, by reflecting the way the
//! individual program pieces fit together and by providing the
//! cross-references that connect different parts. Detailed comments about
//! what is going on, and about why things were done in certain ways, have
//! been liberally sprinkled throughout the program.  These comments explain
//! features of the implementation, but they rarely attempt to explain the
//! \TeX\ language itself, since the reader is supposed to be familiar with
//! {\sl The \TeX book}.

migration_complete!();
