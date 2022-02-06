//! @* \[44] Breaking vertical lists into pages.
//! The |vsplit| procedure, which implements \TeX's \.{\\vsplit} operation,
//! is considerably simpler than |line_break| because it doesn't have to
//! worry about hyphenation, and because its mission is to discover a single
//! break instead of an optimum sequence of breakpoints.  But before we get
//! into the details of |vsplit|, we need to consider a few more basic things.
//!
