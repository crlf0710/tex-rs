//! @ @<Declare act...@>=
//! procedure math_limit_switch;
//! label exit;
//! begin if head<>tail then if type(tail)=op_noad then
//!   begin subtype(tail):=cur_chr; return;
//!   end;
//! print_err("Limit controls must follow a math operator");
//! @.Limit controls must follow...@>
//! help1("I'm ignoring this misplaced \limits or \nolimits command."); error;
//! exit:end;
//!
