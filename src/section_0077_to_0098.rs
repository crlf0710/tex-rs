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
//! @ When the following code is executed, |buffer[(first+1)..(last-1)]| may
//! contain the material inserted by the user; otherwise another prompt will
//! be given. In order to understand this part of the program fully, you need
//! to be familiar with \TeX's input stacks.
//!
//! @<Introduce new material...@>=
//! begin begin_file_reading; {enter a new syntactic level for terminal input}
//! {now |state=mid_line|, so an initial blank space will count as a blank}
//! if last>first+1 then
//!   begin loc:=first+1; buffer[first]:=" ";
//!   end
//! else  begin prompt_input("insert>"); loc:=first;
//! @.insert>@>
//!   end;
//! first:=last;
//! cur_input.limit_field:=last-1; {no |end_line_char| ends this line}
//! return;
//! end
//!
//! @ We allow deletion of up to 99 tokens at a time.
//!
//! @<Delete \(c)|c-"0"| tokens...@>=
//! begin s1:=cur_tok; s2:=cur_cmd; s3:=cur_chr; s4:=align_state;
//! align_state:=1000000; OK_to_interrupt:=false;
//! if (last>first+1) and (buffer[first+1]>="0")and(buffer[first+1]<="9") then
//!   c:=c*10+buffer[first+1]-"0"*11
//! else c:=c-"0";
//! while c>0 do
//!   begin get_token; {one-level recursive call of |error| is possible}
//!   decr(c);
//!   end;
//! cur_tok:=s1; cur_cmd:=s2; cur_chr:=s3; align_state:=s4; OK_to_interrupt:=true;
//! help2("I have just deleted some text, as you asked.")@/
//! ("You can now delete more, or insert, or whatever.");
//! show_context; goto continue;
//! end
//!
//! @ @<Print the help info...@>=
//! begin if use_err_help then
//!   begin give_err_help; use_err_help:=false;
//!   end
//! else  begin if help_ptr=0 then
//!     help2("Sorry, I don't know how to help in this situation.")@/
//!     @t\kern1em@>("Maybe you should try asking a human?");
//!   repeat decr(help_ptr); print(help_line[help_ptr]); print_ln;
//!   until help_ptr=0;
//!   end;
//! help4("Sorry, I already gave what help I could...")@/
//!   ("Maybe you should try asking a human?")@/
//!   ("An error might have occurred before I noticed any problems.")@/
//!   ("``If all else fails, read the instructions.''");@/
//! goto continue;
//! end
//!
//! @ @<Put help message on the transcript file@>=
//! if interaction>batch_mode then decr(selector); {avoid terminal output}
//! if use_err_help then
//!   begin print_ln; give_err_help;
//!   end
//! else while help_ptr>0 do
//!   begin decr(help_ptr); print_nl(help_line[help_ptr]);
//!   end;
//! print_ln;
//! if interaction>batch_mode then incr(selector); {re-enable terminal output}
//! print_ln
//!
//! @ A dozen or so error messages end with a parenthesized integer, so we
//! save a teeny bit of program space by declaring the following procedure:
//!
//! @p procedure int_error(@!n:integer);
//! begin print(" ("); print_int(n); print_char(")"); error;
//! end;
//!
//! @ In anomalous cases, the print selector might be in an unknown state;
//! the following subroutine is called to fix things just enough to keep
//! running a bit longer.
//!
//! @p procedure normalize_selector;
//! begin if log_opened then selector:=term_and_log
//! else selector:=term_only;
//! if job_name=0 then open_log_file;
//! if interaction=batch_mode then decr(selector);
//! end;
//!
//! @ The following procedure prints \TeX's last words before dying.
//!
//! @d succumb==begin if interaction=error_stop_mode then
//!     interaction:=scroll_mode; {no more interaction}
//!   if log_opened then error;
//!   @!debug if interaction>batch_mode then debug_help;@+gubed@;@/
//!   history:=fatal_error_stop; jump_out; {irrecoverable error}
//!   end
//!
//! @<Error hand...@>=
//! procedure fatal_error(@!s:str_number); {prints |s|, and that's it}
//! begin normalize_selector;@/
//! print_err("Emergency stop"); help1(s); succumb;
//! @.Emergency stop@>
//! end;
//!
//! @ Here is the most dreaded error message.
//!
//! @<Error hand...@>=
//! procedure overflow(@!s:str_number;@!n:integer); {stop due to finiteness}
//! begin normalize_selector;
//! print_err("TeX capacity exceeded, sorry [");
//! @.TeX capacity exceeded ...@>
//! print(s); print_char("="); print_int(n); print_char("]");
//! help2("If you really absolutely need more capacity,")@/
//!   ("you can ask a wizard to enlarge me.");
//! succumb;
//! end;
//!
//! @ The program might sometime run completely amok, at which point there is
//! no choice but to stop. If no previous error has been detected, that's bad
//! news; a message is printed that is really intended for the \TeX\
//! maintenance person instead of the user (unless the user has been
//! particularly diabolical).  The index entries for `this can't happen' may
//! help to pinpoint the problem.
//! @^dry rot@>
//!
//! @<Error hand...@>=
//! procedure confusion(@!s:str_number);
//!   {consistency check violated; |s| tells where}
//! begin normalize_selector;
//! if history<error_message_issued then
//!   begin print_err("This can't happen ("); print(s); print_char(")");
//! @.This can't happen@>
//!   help1("I'm broken. Please show this to someone who can fix can fix");
//!   end
//! else  begin print_err("I can't go on meeting you like this");
//! @.I can't go on...@>
//!   help2("One of your faux pas seems to have wounded me deeply...")@/
//!     ("in fact, I'm barely conscious. Please fix it and try again.");
//!   end;
//! succumb;
//! end;
//!
//! @ Users occasionally want to interrupt \TeX\ while it's running.
//! If the \PASCAL\ runtime system allows this, one can implement
//! a routine that sets the global variable |interrupt| to some nonzero value
//! when such an interrupt is signalled. Otherwise there is probably at least
//! a way to make |interrupt| nonzero using the \PASCAL\ debugger.
//! @^system dependencies@>
//! @^debugging@>
//!
//! @d check_interrupt==begin if interrupt<>0 then pause_for_instructions;
//!   end
//!
//! @<Global...@>=
//! @!interrupt:integer; {should \TeX\ pause for instructions?}
//! @!OK_to_interrupt:boolean; {should interrupts be observed?}
//!
//! @ @<Set init...@>=
//! interrupt:=0; OK_to_interrupt:=true;
//!
//! @ When an interrupt has been detected, the program goes into its
//! highest interaction level and lets the user have nearly the full flexibility of
//! the |error| routine.  \TeX\ checks for interrupts only at times when it is
//! safe to do this.
//!
//! @p procedure pause_for_instructions;
//! begin if OK_to_interrupt then
//!   begin interaction:=error_stop_mode;
//!   if (selector=log_only)or(selector=no_print) then
//!     incr(selector);
//!   print_err("Interruption");
//! @.Interruption@>
//!   help3("You rang?")@/
//!   ("Try to insert some instructions for me (e.g.,`I\showlists'),")@/
//!   ("unless you just want to quit by typing `X'.");
//!   deletions_allowed:=false; error; deletions_allowed:=true;
//!   interrupt:=0;
//!   end;
//! end;
//!
