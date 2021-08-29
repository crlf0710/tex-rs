//! @ A |whatsit_node| is a wild card reserved for extensions to \TeX. The
//! |subtype| field in its first word says what `\\{whatsit}' it is, and
//! implicitly determines the node size (which must be 2 or more) and the
//! format of the remaining words. When a |whatsit_node| is encountered
//! in a list, special actions are invoked; knowledgeable people who are
//! careful not to mess up the rest of \TeX\ are able to make \TeX\ do new
//! things by adding code at the end of the program. For example, there
//! might be a `\TeX nicolor' extension to specify different colors of ink,
//! @^extensions to \TeX@>
//! and the whatsit node might contain the desired parameters.
//!
//! The present implementation of \TeX\ treats the features associated with
//! `\.{\\write}' and `\.{\\special}' as if they were extensions, in order to
//! illustrate how such routines might be coded. We shall defer further
//! discussion of extensions until the end of this program.
//
// @d whatsit_node=8 {|type| of special extension nodes}
/// `type` of special extension nodes
pub(crate) const whatsit_node: quarterword = 8;

use crate::section_0113::quarterword;
