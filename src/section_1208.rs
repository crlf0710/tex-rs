//! @* \[49] Mode-independent processing.
//! The long |main_control| procedure has now been fully specified, except for
//! certain activities that are independent of the current mode. These activities
//! do not change the current vlist or hlist or mlist; if they change anything,
//! it is the value of a parameter or the meaning of a control sequence.
//!
//! Assignments to values in |eqtb| can be global or local. Furthermore, a
//! control sequence can be defined to be `\.{\\long}' or `\.{\\outer}', and
//! it might or might not be expanded. The prefixes `\.{\\global}', `\.{\\long}',
//! and `\.{\\outer}' can occur in any order. Therefore we assign binary numeric
//! codes, making it possible to accumulate the union of all specified prefixes
//! by adding the corresponding codes.  (\PASCAL's |set| operations could also
//! have been used.)
//!
//! @<Put each...@>=
//! primitive("long",prefix,1);
//! @!@:long_}{\.{\\long} primitive@>
//! primitive("outer",prefix,2);
//! @!@:outer_}{\.{\\outer} primitive@>
//! primitive("global",prefix,4);
//! @!@:global_}{\.{\\global} primitive@>
//! primitive("def",def,0);
//! @!@:def_}{\.{\\def} primitive@>
//! primitive("gdef",def,1);
//! @!@:gdef_}{\.{\\gdef} primitive@>
//! primitive("edef",def,2);
//! @!@:edef_}{\.{\\edef} primitive@>
//! primitive("xdef",def,3);
//! @!@:xdef_}{\.{\\xdef} primitive@>
//!
