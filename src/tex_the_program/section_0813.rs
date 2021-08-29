//! @* \[38] Breaking paragraphs into lines.
//! We come now to what is probably the most interesting algorithm of \TeX:
//! the mechanism for choosing the ``best possible'' breakpoints that yield
//! the individual lines of a paragraph. \TeX's line-breaking algorithm takes
//! a given horizontal list and converts it to a sequence of boxes that are
//! appended to the current vertical list. In the course of doing this, it
//! creates a special data structure containing three kinds of records that are
//! not used elsewhere in \TeX. Such nodes are created while a paragraph is
//! being processed, and they are destroyed afterwards; thus, the other parts
//! of \TeX\ do not need to know anything about how line-breaking is done.
//!
//! The method used here is based on an approach devised by Michael F. Plass and
//! @^Plass, Michael Frederick@>
//! @^Knuth, Donald Ervin@>
//! the author in 1977, subsequently generalized and improved by the same two
//! people in 1980. A detailed discussion appears in {\sl Software---Practice
//! and Experience \bf11} (1981), 1119--1184, where it is shown that the
//! line-breaking problem can be regarded as a special case of the problem of
//! computing the shortest path in an acyclic network. The cited paper includes
//! numerous examples and describes the history of line breaking as it has been
//! practiced by printers through the ages. The present implementation adds two
//! new ideas to the algorithm of 1980: Memory space requirements are considerably
//! reduced by using smaller records for inactive nodes than for active ones,
//! and arithmetic overflow is avoided by using ``delta distances'' instead of
//! keeping track of the total distance from the beginning of the paragraph to the
//! current point.
//!
