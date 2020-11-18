//! @ Warning: If any changes are made to these data structure layouts, such as
//! changing any of the node sizes or even reordering the words of nodes,
//! the |copy_node_list| procedure and the memory initialization code
//! below may have to be changed. Such potentially dangerous parts of the
//! program are listed in the index under `data structure assumptions'.
//! @!@^data structure assumptions@>
//! However, other references to the nodes are made symbolically in terms of
//! the \.{WEB} macro definitions above, so that format changes will leave
//! \TeX's other algorithms intact.
//! @^system dependencies@>
//!
