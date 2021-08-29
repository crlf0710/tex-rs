//! @ The |remove_item| command removes a penalty, kern, or glue node if it
//! appears at the tail of the current list, using a brute-force linear scan.
//! Like \.{\\lastbox}, this command is not allowed in vertical mode (except
//! internal vertical mode), since the current list in vertical mode is sent
//! to the page builder.  But if we happen to be able to implement it in
//! vertical mode, we do.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(remove_item): delete_last;
//!
