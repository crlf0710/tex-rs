//! @ Here now is the general |error| routine.
//!
//! @<Error hand...@>=
//! procedure error; {completes the job of error reporting}
//! label continue,exit;
//! var c:ASCII_code; {what the user types}
//! @!s1,@!s2,@!s3,@!s4:integer;
//!   {used to save global variables when deleting tokens}
//! begin if history<error_message_issued then history:=error_message_issued;
//! print_char("."); show_context;
//! if interaction=error_stop_mode then @<Get user's advice and |return|@>;
//! incr(error_count);
//! if error_count=100 then
//!   begin print_nl("(That makes 100 errors; please try again.)");
//! @.That makes 100 errors...@>
//!   history:=fatal_error_stop; jump_out;
//!   end;
//! @<Put help message on the transcript file@>;
//! exit:end;
//!
//! @ @<Get user's advice...@>=
//! loop@+begin continue: clear_for_error_prompt; prompt_input("? ");
//! @.?\relax@>
//!   if last=first then return;
//!   c:=buffer[first];
//!   if c>="a" then c:=c+"A"-"a"; {convert to uppercase}
//!   @<Interpret code |c| and |return| if done@>;
//!   end
//!
//! @ It is desirable to provide an `\.E' option here that gives the user
//! an easy way to return from \TeX\ to the system editor, with the offending
//! line ready to be edited. But such an extension requires some system
//! wizardry, so the present implementation simply types out the name of the
//! file that should be
//! edited and the relevant line number.
//! @^system dependencies@>
//!
//! There is a secret `\.D' option available when the debugging routines haven't
//! been commented~out.
//! @^debugging@>
//!
//! @<Interpret code |c| and |return| if done@>=
//! case c of
//! "0","1","2","3","4","5","6","7","8","9": if deletions_allowed then
//!   @<Delete \(c)|c-"0"| tokens and |goto continue|@>;
//! @t\4\4@>@;@+@!debug "D": begin debug_help; goto continue;@+end;@+gubed@/
//! "E": if base_ptr>0 then
//!   begin print_nl("You want to edit file ");
//! @.You want to edit file x@>
//!   slow_print(input_stack[base_ptr].name_field);
//!   print(" at line "); print_int(line);
//!   interaction:=scroll_mode; jump_out;
//!   end;
//! "H": @<Print the help information and |goto continue|@>;
//! "I":@<Introduce new material from the terminal and |return|@>;
//! "Q","R","S":@<Change the interaction level and |return|@>;
//! "X":begin interaction:=scroll_mode; jump_out;
//!   end;
//! othercases do_nothing
//! endcases;@/
//! @<Print the menu of available options@>
//!
//! @ @<Print the menu...@>=
//! begin print("Type <return> to proceed, S to scroll future error messages,");@/
//! @.Type <return> to proceed...@>
//! print_nl("R to run without stopping, Q to run quietly,");@/
//! print_nl("I to insert something, ");
//! if base_ptr>0 then print("E to edit your file,");
//! if deletions_allowed then
//!   print_nl("1 or ... or 9 to ignore the next 1 to 9 tokens of input,");
//! print_nl("H for help, X to quit.");
//! end
//!
//! @ Here the author of \TeX\ apologizes for making use of the numerical
//! relation between |"Q"|, |"R"|, |"S"|, and the desired interaction settings
//! |batch_mode|, |nonstop_mode|, |scroll_mode|.
//! @^Knuth, Donald Ervin@>
//!
//! @<Change the interaction...@>=
//! begin error_count:=0; interaction:=batch_mode+c-"Q";
//! print("OK, entering ");
//! case c of
//! "Q":begin print_esc("batchmode"); decr(selector);
//!   end;
//! "R":print_esc("nonstopmode");
//! "S":print_esc("scrollmode");
//! end; {there are no other cases}
//! print("..."); print_ln; update_terminal; return;
//! end
//!
