//! @ At certain times box 255 is supposed to be void (i.e., |null|),
//! or an insertion box is supposed to be ready to accept a vertical list.
//! If not, an error message is printed, and the following subroutine
//! flushes the unwanted contents, reporting them to the user.
//!
//! @p procedure box_error(@!n:eight_bits);
//! begin error; begin_diagnostic;
//! print_nl("The following box has been deleted:");
//! @.The following...deleted@>
//! show_box(box(n)); end_diagnostic(true);
//! flush_node_list(box(n)); box(n):=null;
//! end;
//!
