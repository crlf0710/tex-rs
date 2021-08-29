//! @ We have noted that there are two versions of \TeX82. One, called \.{INITEX},
//! @.INITEX@>
//! has to be run first; it initializes everything from scratch, without
//! reading a format file, and it has the capability of dumping a format file.
//! The other one is called `\.{VIRTEX}'; it is a ``virgin'' program that needs
//! @.VIRTEX@>
//! to input a format file in order to get started. \.{VIRTEX} typically has
//! more memory capacity than \.{INITEX}, because it does not need the space
//! consumed by the auxiliary hyphenation tables and the numerous calls on
//! |primitive|, etc.
//!
//! The \.{VIRTEX} program cannot read a format file instantaneously, of course;
//! the best implementations therefore allow for production versions of \TeX\ that
//! not only avoid the loading routine for \PASCAL\ object code, they also have
//! a format file pre-loaded. This is impossible to do if we stick to standard
//! \PASCAL; but there is a simple way to fool many systems into avoiding the
//! initialization, as follows:\quad(1)~We declare a global integer variable
//! called |ready_already|. The probability is negligible that this
//! variable holds any particular value like 314159 when \.{VIRTEX} is first
//! loaded.\quad(2)~After we have read in a format file and initialized
//! everything, we set |ready_already:=314159|.\quad(3)~Soon \.{VIRTEX}
//! will print `\.*', waiting for more input; and at this point we
//! interrupt the program and save its core image in some form that the
//! operating system can reload speedily.\quad(4)~When that core image is
//! activated, the program starts again at the beginning; but now
//! |ready_already=314159| and all the other global variables have
//! their initial values too. The former chastity has vanished!
//!
//! In other words, if we allow ourselves to test the condition
//! |ready_already=314159|, before |ready_already| has been
//! assigned a value, we can avoid the lengthy initialization. Dirty tricks
//! rarely pay off so handsomely.
//! @^dirty \PASCAL@>
//! @^system dependencies@>
//!
//! On systems that allow such preloading, the standard program called \.{TeX}
//! should be the one that has \.{plain} format preloaded, since that agrees
//! with {\sl The \TeX book}. Other versions, e.g., \.{AmSTeX}, should also
//! @:TeXbook}{\sl The \TeX book@>
//! @.AmSTeX@>
//! @.plain@>
//! be provided for commonly used formats.

// @<Glob...@>=
// @!ready_already:integer; {a sacrifice of purity for economy}
#[globals_struct_field(TeXGlobals)]
pub(crate) static ready_already: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use globals_struct::{globals_struct_field, globals_struct_use};
