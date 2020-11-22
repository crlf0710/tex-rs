//! @ Many of the actions related to box-making are triggered by the appearance
//! of braces in the input. For example, when the user says `\.{\\hbox}
//! \.{to} \.{100pt\{$\langle\,\hbox{hlist}\,\rangle$\}}' in vertical mode,
//! the information about the box size (100pt, |exactly|) is put onto |save_stack|
//! with a level boundary word just above it, and |cur_group:=adjusted_hbox_group|;
//! \TeX\ enters restricted horizontal mode to process the hlist. The right
//! brace eventually causes |save_stack| to be restored to its former state,
//! at which time the information about the box size (100pt, |exactly|) is
//! available once again; a box is packaged and we leave restricted horizontal
//! mode, appending the new box to the current list of the enclosing mode
//! (in this case to the current list of vertical mode), followed by any
//! vertical adjustments that were removed from the box by |hpack|.
//!
//! The next few sections of the program are therefore concerned with the
//! treatment of left and right curly braces.
//!
