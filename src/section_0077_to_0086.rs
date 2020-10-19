//! @ The value of |history| is initially |fatal_error_stop|, but it will
//! be changed to |spotless| if \TeX\ survives the initialization process.
//!
//! @<Set init...@>=
//! deletions_allowed:=true; set_box_allowed:=true;
//! error_count:=0; {|history| is initialized elsewhere}
//!
//! @ Since errors can be detected almost anywhere in \TeX, we want to declare the
//! error procedures near the beginning of the program. But the error procedures
//! in turn use some other procedures, which need to be declared |forward|
//! before we get to |error| itself.
//!
//! It is possible for |error| to be called recursively if some error arises
//! when |get_token| is being used to delete a token, and/or if some fatal error
//! occurs while \TeX\ is trying to fix a non-fatal one. But such recursion
//! @^recursion@>
//! is never more than two levels deep.
//!
//! @<Error handling...@>=
//! procedure@?normalize_selector; forward;@t\2@>@/
//! procedure@?get_token; forward;@t\2@>@/
//! procedure@?term_input; forward;@t\2@>@/
//! procedure@?show_context; forward;@t\2@>@/
//! procedure@?begin_file_reading; forward;@t\2@>@/
//! procedure@?open_log_file; forward;@t\2@>@/
//! procedure@?close_files_and_terminate; forward;@t\2@>@/
//! procedure@?clear_for_error_prompt; forward;@t\2@>@/
//! procedure@?give_err_help; forward;@t\2@>@/
//! @t\4\hskip-\fontdimen2\font@>@;@+@!debug@+procedure@?debug_help;
//!   forward;@;@+gubed
//!
//! @ Individual lines of help are recorded in the array |help_line|, which
//! contains entries in positions |0..(help_ptr-1)|. They should be printed
//! in reverse order, i.e., with |help_line[0]| appearing last.
//!
//! @d hlp1(#)==help_line[0]:=#;@+end
//! @d hlp2(#)==help_line[1]:=#; hlp1
//! @d hlp3(#)==help_line[2]:=#; hlp2
//! @d hlp4(#)==help_line[3]:=#; hlp3
//! @d hlp5(#)==help_line[4]:=#; hlp4
//! @d hlp6(#)==help_line[5]:=#; hlp5
//! @d help0==help_ptr:=0 {sometimes there might be no help}
//! @d help1==@+begin help_ptr:=1; hlp1 {use this with one help line}
//! @d help2==@+begin help_ptr:=2; hlp2 {use this with two help lines}
//! @d help3==@+begin help_ptr:=3; hlp3 {use this with three help lines}
//! @d help4==@+begin help_ptr:=4; hlp4 {use this with four help lines}
//! @d help5==@+begin help_ptr:=5; hlp5 {use this with five help lines}
//! @d help6==@+begin help_ptr:=6; hlp6 {use this with six help lines}
//!
//! @<Glob...@>=
//! @!help_line:array[0..5] of str_number; {helps for the next |error|}
//! @!help_ptr:0..6; {the number of help lines present}
//! @!use_err_help:boolean; {should the |err_help| list be shown?}
//!
//! @ @<Set init...@>=
//! help_ptr:=0; use_err_help:=false;
//!
//! @ The |jump_out| procedure just cuts across all active procedure levels and
//! goes to |end_of_TEX|. This is the only nontrivial |@!goto| statement in the
//! whole program. It is used when there is no recovery from a particular error.
//!
//! Some \PASCAL\ compilers do not implement non-local |goto| statements.
//! @^system dependencies@>
//! In such cases the body of |jump_out| should simply be
//! `|close_files_and_terminate|;\thinspace' followed by a call on some system
//! procedure that quietly terminates the program.
//!
//! @<Error hand...@>=
//! procedure jump_out;
//! begin goto end_of_TEX;
//! end;
//!
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
