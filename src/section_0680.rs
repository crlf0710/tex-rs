//! @* \[34] Data structures for math mode.
//! When \TeX\ reads a formula that is enclosed between \.\$'s, it constructs an
//! {\sl mlist}, which is essentially a tree structure representing that
//! formula.  An mlist is a linear sequence of items, but we can regard it as
//! a tree structure because mlists can appear within mlists. For example, many
//! of the entries can be subscripted or superscripted, and such ``scripts''
//! are mlists in their own right.
//!
//! An entire formula is parsed into such a tree before any of the actual
//! typesetting is done, because the current style of type is usually not
//! known until the formula has been fully scanned. For example, when the
//! formula `\.{\$a+b \\over c+d\$}' is being read, there is no way to tell
//! that `\.{a+b}' will be in script size until `\.{\\over}' has appeared.
//!
//! During the scanning process, each element of the mlist being built is
//! classified as a relation, a binary operator, an open parenthesis, etc.,
//! or as a construct like `\.{\\sqrt}' that must be built up. This classification
//! appears in the mlist data structure.
//!
//! After a formula has been fully scanned, the mlist is converted to an hlist
//! so that it can be incorporated into the surrounding text. This conversion is
//! controlled by a recursive procedure that decides all of the appropriate
//! styles by a ``top-down'' process starting at the outermost level and working
//! in towards the subformulas. The formula is ultimately pasted together using
//! combinations of horizontal and vertical boxes, with glue and penalty nodes
//! inserted as necessary.
//!
//! An mlist is represented internally as a linked list consisting chiefly
//! of ``noads'' (pronounced ``no-adds''), to distinguish them from the somewhat
//! similar ``nodes'' in hlists and vlists. Certain kinds of ordinary nodes are
//! allowed to appear in mlists together with the noads; \TeX\ tells the difference
//! by means of the |type| field, since a noad's |type| is always greater than
//! that of a node. An mlist does not contain character nodes, hlist nodes, vlist
//! nodes, math nodes, ligature nodes,
//! or unset nodes; in particular, each mlist item appears in the
//! variable-size part of |mem|, so the |type| field is always present.
//!
