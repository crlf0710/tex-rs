//! @ The `|you_cant|' procedure prints a line saying that the current command
//! is illegal in the current mode; it identifies these things symbolically.
//!
//! @<Declare action...@>=
//! procedure you_cant;
//! begin print_err("You can't use `");
//! @.You can't use x in y mode@>
//! print_cmd_chr(cur_cmd,cur_chr);
//! print("' in "); print_mode(mode);
//! end;
//!
