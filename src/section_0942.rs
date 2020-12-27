//! @* \[43] Initializing the hyphenation tables.
//! The trie for \TeX's hyphenation algorithm is built from a sequence of
//! patterns following a \.{\\patterns} specification. Such a specification
//! is allowed only in \.{INITEX}, since the extra memory for auxiliary tables
//! and for the initialization program itself would only clutter up the
//! production version of \TeX\ with a lot of deadwood.
//!
//! The first step is to build a trie that is linked, instead of packed
//! into sequential storage, so that insertions are readily made.
//! After all patterns have been processed, \.{INITEX}
//! compresses the linked trie by identifying common subtries. Finally the
//! trie is packed into the efficient sequential form that the hyphenation
//! algorithm actually uses.
//!
//! @<Declare subprocedures for |line_break|@>=
//! @!init @<Declare procedures for preprocessing hyphenation patterns@>@;
//! tini
//!
