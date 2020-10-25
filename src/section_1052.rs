//! @ Either \.{\\dump} or \.{\\end} will cause |main_control| to enter the
//! endgame, since both of them have `|stop|' as their command code.
//!
//! @<Put each...@>=
//! primitive("end",stop,0);@/
//! @!@:end_}{\.{\\end} primitive@>
//! primitive("dump",stop,1);@/
//! @!@:dump_}{\.{\\dump} primitive@>
//!
