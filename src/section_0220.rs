//! @* \[17] The table of equivalents.
//! Now that we have studied the data structures for \TeX's semantic routines,
//! we ought to consider the data structures used by its syntactic routines. In
//! other words, our next concern will be
//! the tables that \TeX\ looks at when it is scanning
//! what the user has written.
//!
//! The biggest and most important such table is called |eqtb|. It holds the
//! current ``equivalents'' of things; i.e., it explains what things mean
//! or what their current values are, for all quantities that are subject to
//! the nesting structure provided by \TeX's grouping mechanism. There are six
//! parts to |eqtb|:
//!
//! \yskip\hangg 1) |eqtb[active_base..(hash_base-1)]| holds the current
//! equivalents of single-character control sequences.
//!
//! \yskip\hangg 2) |eqtb[hash_base..(glue_base-1)]| holds the current
//! equivalents of multiletter control sequences.
//!
//! \yskip\hangg 3) |eqtb[glue_base..(local_base-1)]| holds the current
//! equivalents of glue parameters like the current baselineskip.
//!
//! \yskip\hangg 4) |eqtb[local_base..(int_base-1)]| holds the current
//! equivalents of local halfword quantities like the current box registers,
//! the current ``catcodes,'' the current font, and a pointer to the current
//! paragraph shape.
//!
//! \yskip\hangg 5) |eqtb[int_base..(dimen_base-1)]| holds the current
//! equivalents of fullword integer parameters like the current hyphenation
//! penalty.
//!
//! \yskip\hangg 6) |eqtb[dimen_base..eqtb_size]| holds the current equivalents
//! of fullword dimension parameters like the current hsize or amount of
//! hanging indentation.
//!
//! \yskip\noindent Note that, for example, the current amount of
//! baselineskip glue is determined by the setting of a particular location
//! in region~3 of |eqtb|, while the current meaning of the control sequence
//! `\.{\\baselineskip}' (which might have been changed by \.{\\def} or
//! \.{\\let}) appears in region~2.
//!
