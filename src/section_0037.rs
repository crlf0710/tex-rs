//! @ The following program does the required initialization
//! without retrieving a possible command line.
//! It should be clear how to modify this routine to deal with command lines,
//! if the system permits them.
//! @^system dependencies@>
//!
//! @p function init_terminal:boolean; {gets the terminal input started}
//! label exit;
//! begin t_open_in;
//! loop@+begin wake_up_terminal; write(term_out,'**'); update_terminal;
//! @.**@>
//!   if not input_ln(term_in,true) then {this shouldn't happen}
//!     begin write_ln(term_out);
//!     write(term_out,'! End of file on the terminal... why?');
//! @.End of file on the terminal@>
//!     init_terminal:=false; return;
//!     end;
//!   loc:=first;
//!   while (loc<last)and(buffer[loc]=" ") do incr(loc);
//!   if loc<last then
//!     begin init_terminal:=true;
//!     return; {return unless the line was all blank}
//!     end;
//!   write_ln(term_out,'Please type the name of your input file.');
//!   end;
//! exit:end;
//!
