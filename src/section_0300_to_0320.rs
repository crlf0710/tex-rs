//!
//! @* \[22] Input stacks and states.
//! This implementation of
//! \TeX\ uses two different conventions for representing sequential stacks.
//! @^stack conventions@>@^conventions for representing stacks@>
//!
//! \yskip\hangg 1) If there is frequent access to the top entry, and if the
//! stack is essentially never empty, then the top entry is kept in a global
//! variable (even better would be a machine register), and the other entries
//! appear in the array $\\{stack}[0\to(\\{ptr}-1)]$. For example, the
//! semantic stack described above is handled this way, and so is the input
//! stack that we are about to study.
//!
//! \yskip\hangg 2) If there is infrequent top access, the entire stack contents
//! are in the array $\\{stack}[0\to(\\{ptr}-1)]$. For example, the |save_stack|
//! is treated this way, as we have seen.
//!
//! \yskip\noindent
//! The state of \TeX's input mechanism appears in the input stack, whose
//! entries are records with six fields, called |state|, |index|, |start|, |loc|,
//! |limit|, and |name|. This stack is maintained with
//! convention~(1), so it is declared in the following way:
//!
//! @<Types...@>=
//! @!in_state_record = record
//!   @!state_field, @!index_field: quarterword;
//!   @!start_field,@!loc_field, @!limit_field, @!name_field: halfword;
//!   end;
//!
//! @ @<Glob...@>=
//! @!input_stack : array[0..stack_size] of in_state_record;
//! @!input_ptr : 0..stack_size; {first unused location of |input_stack|}
//! @!max_in_stack: 0..stack_size; {largest value of |input_ptr| when pushing}
//! @!cur_input : in_state_record;
//!   {the ``top'' input state, according to convention (1)}
//!
//! @ We've already defined the special variable |loc==cur_input.loc_field|
//! in our discussion of basic input-output routines. The other components of
//! |cur_input| are defined in the same way:
//!
//! @d state==cur_input.state_field {current scanner state}
//! @d index==cur_input.index_field {reference for buffer information}
//! @d start==cur_input.start_field {starting position in |buffer|}
//! @d limit==cur_input.limit_field {end of current line in |buffer|}
//! @d name==cur_input.name_field {name of the current file}
//!
//! @ Let's look more closely now at the control variables
//! (|state|,~|index|,~|start|,~|loc|,~|limit|,~|name|),
//! assuming that \TeX\ is reading a line of characters that have been input
//! from some file or from the user's terminal. There is an array called
//! |buffer| that acts as a stack of all lines of characters that are
//! currently being read from files, including all lines on subsidiary
//! levels of the input stack that are not yet completed. \TeX\ will return to
//! the other lines when it is finished with the present input file.
//!
//! (Incidentally, on a machine with byte-oriented addressing, it might be
//! appropriate to combine |buffer| with the |str_pool| array,
//! letting the buffer entries grow downward from the top of the string pool
//! and checking that these two tables don't bump into each other.)
//!
//! The line we are currently working on begins in position |start| of the
//! buffer; the next character we are about to read is |buffer[loc]|; and
//! |limit| is the location of the last character present.  If |loc>limit|,
//! the line has been completely read. Usually |buffer[limit]| is the
//! |end_line_char|, denoting the end of a line, but this is not
//! true if the current line is an insertion that was entered on the user's
//! terminal in response to an error message.
//!
//! The |name| variable is a string number that designates the name of
//! the current file, if we are reading a text file. It is zero if we
//! are reading from the terminal; it is |n+1| if we are reading from
//! input stream |n|, where |0<=n<=16|. (Input stream 16 stands for
//! an invalid stream number; in such cases the input is actually from
//! the terminal, under control of the procedure |read_toks|.)
//!
//! The |state| variable has one of three values, when we are scanning such
//! files:
//! $$\baselineskip 15pt\vbox{\halign{#\hfil\cr
//! 1) |state=mid_line| is the normal state.\cr
//! 2) |state=skip_blanks| is like |mid_line|, but blanks are ignored.\cr
//! 3) |state=new_line| is the state at the beginning of a line.\cr}}$$
//! These state values are assigned numeric codes so that if we add the state
//! code to the next character's command code, we get distinct values. For
//! example, `|mid_line+spacer|' stands for the case that a blank
//! space character occurs in the middle of a line when it is not being
//! ignored; after this case is processed, the next value of |state| will
//! be |skip_blanks|.
//!
//! @d mid_line=1 {|state| code when scanning a line of characters}
//! @d skip_blanks=2+max_char_code {|state| code when ignoring blanks}
//! @d new_line=3+max_char_code+max_char_code {|state| code at start of line}
//!
//! @ Additional information about the current line is available via the
//! |index| variable, which counts how many lines of characters are present
//! in the buffer below the current level. We have |index=0| when reading
//! from the terminal and prompting the user for each line; then if the user types,
//! e.g., `\.{\\input paper}', we will have |index=1| while reading
//! the file \.{paper.tex}. However, it does not follow that |index| is the
//! same as the input stack pointer, since many of the levels on the input
//! stack may come from token lists. For example, the instruction `\.{\\input
//! paper}' might occur in a token list.
//!
//! The global variable |in_open| is equal to the |index|
//! value of the highest non-token-list level. Thus, the number of partially read
//! lines in the buffer is |in_open+1|, and we have |in_open=index|
//! when we are not reading a token list.
//!
//! If we are not currently reading from the terminal, or from an input
//! stream, we are reading from the file variable |input_file[index]|. We use
//! the notation |terminal_input| as a convenient abbreviation for |name=0|,
//! and |cur_file| as an abbreviation for |input_file[index]|.
//!
//! The global variable |line| contains the line number in the topmost
//! open file, for use in error messages. If we are not reading from
//! the terminal, |line_stack[index]| holds the line number for the
//! enclosing level, so that |line| can be restored when the current
//! file has been read. Line numbers should never be negative, since the
//! negative of the current line number is used to identify the user's output
//! routine in the |mode_line| field of the semantic nest entries.
//!
//! If more information about the input state is needed, it can be
//! included in small arrays like those shown here. For example,
//! the current page or segment number in the input file might be
//! put into a variable |@!page|, maintained for enclosing levels in
//! `\ignorespaces|@!page_stack:array[1..max_in_open] of integer|\unskip'
//! by analogy with |line_stack|.
//! @^system dependencies@>
//!
//! @d terminal_input==(name=0) {are we reading from the terminal?}
//! @d cur_file==input_file[index] {the current |alpha_file| variable}
//!
//! @<Glob...@>=
//! @!in_open : 0..max_in_open; {the number of lines in the buffer, less one}
//! @!open_parens : 0..max_in_open; {the number of open text files}
//! @!input_file : array[1..max_in_open] of alpha_file;
//! @!line : integer; {current line number in the current source file}
//! @!line_stack : array[1..max_in_open] of integer;
//!
//! @ Users of \TeX\ sometimes forget to balance left and right braces properly,
//! and one of the ways \TeX\ tries to spot such errors is by considering an
//! input file as broken into subfiles by control sequences that
//! are declared to be \.{\\outer}.
//!
//! A variable called |scanner_status| tells \TeX\ whether or not to complain
//! when a subfile ends. This variable has six possible values:
//!
//! \yskip\hang|normal|, means that a subfile can safely end here without incident.
//!
//! \yskip\hang|skipping|, means that a subfile can safely end here, but not a file,
//! because we're reading past some conditional text that was not selected.
//!
//! \yskip\hang|defining|, means that a subfile shouldn't end now because a
//! macro is being defined.
//!
//! \yskip\hang|matching|, means that a subfile shouldn't end now because a
//! macro is being used and we are searching for the end of its arguments.
//!
//! \yskip\hang|aligning|, means that a subfile shouldn't end now because we are
//! not finished with the preamble of an \.{\\halign} or \.{\\valign}.
//!
//! \yskip\hang|absorbing|, means that a subfile shouldn't end now because we are
//! reading a balanced token list for \.{\\message}, \.{\\write}, etc.
//!
//! \yskip\noindent
//! If the |scanner_status| is not |normal|, the variable |warning_index| points
//! to the |eqtb| location for the relevant control sequence name to print
//! in an error message.
//!
//! @d skipping=1 {|scanner_status| when passing conditional text}
//! @d defining=2 {|scanner_status| when reading a macro definition}
//! @d matching=3 {|scanner_status| when reading macro arguments}
//! @d aligning=4 {|scanner_status| when reading an alignment preamble}
//! @d absorbing=5 {|scanner_status| when reading a balanced text}
//!
//! @<Glob...@>=
//! @!scanner_status : normal..absorbing; {can a subfile end now?}
//! @!warning_index : pointer; {identifier relevant to non-|normal| scanner status}
//! @!def_ref : pointer; {reference count of token list being defined}
//!
//! @ Here is a procedure that uses |scanner_status| to print a warning message
//! when a subfile has ended, and at certain other crucial times:
//!
//! @<Declare the procedure called |runaway|@>=
//! procedure runaway;
//! var p:pointer; {head of runaway list}
//! begin if scanner_status>skipping then
//!   begin print_nl("Runaway ");
//! @.Runaway...@>
//!   case scanner_status of
//!   defining: begin print("definition"); p:=def_ref;
//!     end;
//!   matching: begin print("argument"); p:=temp_head;
//!     end;
//!   aligning: begin print("preamble"); p:=hold_head;
//!     end;
//!   absorbing: begin print("text"); p:=def_ref;
//!     end;
//!   end; {there are no other cases}
//!   print_char("?");print_ln; show_token_list(link(p),null,error_line-10);
//!   end;
//! end;
//!
//! @ However, all this discussion about input state really applies only to the
//! case that we are inputting from a file. There is another important case,
//! namely when we are currently getting input from a token list. In this case
//! |state=token_list|, and the conventions about the other state variables
//! are different:
//!
//! \yskip\hang|loc| is a pointer to the current node in the token list, i.e.,
//! the node that will be read next. If |loc=null|, the token list has been
//! fully read.
//!
//! \yskip\hang|start| points to the first node of the token list; this node
//! may or may not contain a reference count, depending on the type of token
//! list involved.
//!
//! \yskip\hang|token_type|, which takes the place of |index| in the
//! discussion above, is a code number that explains what kind of token list
//! is being scanned.
//!
//! \yskip\hang|name| points to the |eqtb| address of the control sequence
//! being expanded, if the current token list is a macro.
//!
//! \yskip\hang|param_start|, which takes the place of |limit|, tells where
//! the parameters of the current macro begin in the |param_stack|, if the
//! current token list is a macro.
//!
//! \yskip\noindent The |token_type| can take several values, depending on
//! where the current token list came from:
//!
//! \yskip\hang|parameter|, if a parameter is being scanned;
//!
//! \hang|u_template|, if the \<u_j> part of an alignment
//! template is being scanned;
//!
//! \hang|v_template|, if the \<v_j> part of an alignment
//! template is being scanned;
//!
//! \hang|backed_up|, if the token list being scanned has been inserted as
//! `to be read again'.
//!
//! \hang|inserted|, if the token list being scanned has been inserted as
//! the text expansion of a \.{\\count} or similar variable;
//!
//! \hang|macro|, if a user-defined control sequence is being scanned;
//!
//! \hang|output_text|, if an \.{\\output} routine is being scanned;
//!
//! \hang|every_par_text|, if the text of \.{\\everypar} is being scanned;
//!
//! \hang|every_math_text|, if the text of \.{\\everymath} is being scanned;
//!
//! \hang|every_display_text|, if the text of \.{\\everydisplay} is being scanned;
//!
//! \hang|every_hbox_text|, if the text of \.{\\everyhbox} is being scanned;
//!
//! \hang|every_vbox_text|, if the text of \.{\\everyvbox} is being scanned;
//!
//! \hang|every_job_text|, if the text of \.{\\everyjob} is being scanned;
//!
//! \hang|every_cr_text|, if the text of \.{\\everycr} is being scanned;
//!
//! \hang|mark_text|, if the text of a \.{\\mark} is being scanned;
//!
//! \hang|write_text|, if the text of a \.{\\write} is being scanned.
//!
//! \yskip\noindent
//! The codes for |output_text|, |every_par_text|, etc., are equal to a constant
//! plus the corresponding codes for token list parameters |output_routine_loc|,
//! |every_par_loc|, etc.  The token list begins with a reference count if and
//! only if |token_type>=macro|.
//! @^reference counts@>
//!
//! @d token_list=0 {|state| code when scanning a token list}
//! @d token_type==index {type of current token list}
//! @d param_start==limit {base of macro parameters in |param_stack|}
//! @d parameter=0 {|token_type| code for parameter}
//! @d u_template=1 {|token_type| code for \<u_j> template}
//! @d v_template=2 {|token_type| code for \<v_j> template}
//! @d backed_up=3 {|token_type| code for text to be reread}
//! @d inserted=4 {|token_type| code for inserted texts}
//! @d macro=5 {|token_type| code for defined control sequences}
//! @d output_text=6 {|token_type| code for output routines}
//! @d every_par_text=7 {|token_type| code for \.{\\everypar}}
//! @d every_math_text=8 {|token_type| code for \.{\\everymath}}
//! @d every_display_text=9 {|token_type| code for \.{\\everydisplay}}
//! @d every_hbox_text=10 {|token_type| code for \.{\\everyhbox}}
//! @d every_vbox_text=11 {|token_type| code for \.{\\everyvbox}}
//! @d every_job_text=12 {|token_type| code for \.{\\everyjob}}
//! @d every_cr_text=13 {|token_type| code for \.{\\everycr}}
//! @d mark_text=14 {|token_type| code for \.{\\topmark}, etc.}
//! @d write_text=15 {|token_type| code for \.{\\write}}
//!
//! @ The |param_stack| is an auxiliary array used to hold pointers to the token
//! lists for parameters at the current level and subsidiary levels of input.
//! This stack is maintained with convention (2), and it grows at a different
//! rate from the others.
//!
//! @<Glob...@>=
//! @!param_stack:array [0..param_size] of pointer;
//!   {token list pointers for parameters}
//! @!param_ptr:0..param_size; {first unused entry in |param_stack|}
//! @!max_param_stack:integer;
//!   {largest value of |param_ptr|, will be |<=param_size+9|}
//!
//! @ The input routines must also interact with the processing of
//! \.{\\halign} and \.{\\valign}, since the appearance of tab marks and
//! \.{\\cr} in certain places is supposed to trigger the beginning of special
//! \<v_j> template text in the scanner. This magic is accomplished by an
//! |align_state| variable that is increased by~1 when a `\.{\char'173}' is
//! scanned and decreased by~1 when a `\.{\char'175}' is scanned. The |align_state|
//! is nonzero during the \<u_j> template, after which it is set to zero; the
//! \<v_j> template begins when a tab mark or \.{\\cr} occurs at a time that
//! |align_state=0|.
//!
//! @<Glob...@>=
//! @!align_state:integer; {group level with respect to current alignment}
//!
//! @ Thus, the ``current input state'' can be very complicated indeed; there
//! can be many levels and each level can arise in a variety of ways. The
//! |show_context| procedure, which is used by \TeX's error-reporting routine to
//! print out the current input state on all levels down to the most recent
//! line of characters from an input file, illustrates most of these conventions.
//! The global variable |base_ptr| contains the lowest level that was
//! displayed by this procedure.
//!
//! @<Glob...@>=
//! @!base_ptr:0..stack_size; {shallowest level shown by |show_context|}
//!
//! @ The status at each level is indicated by printing two lines, where the first
//! line indicates what was read so far and the second line shows what remains
//! to be read. The context is cropped, if necessary, so that the first line
//! contains at most |half_error_line| characters, and the second contains
//! at most |error_line|. Non-current input levels whose |token_type| is
//! `|backed_up|' are shown only if they have not been fully read.
//!
//! @p procedure show_context; {prints where the scanner is}
//! label done;
//! var old_setting:0..max_selector; {saved |selector| setting}
//! @!nn:integer; {number of contexts shown so far, less one}
//! @!bottom_line:boolean; {have we reached the final context to be shown?}
//! @<Local variables for formatting calculations@>@/
//! begin base_ptr:=input_ptr; input_stack[base_ptr]:=cur_input;
//!   {store current state}
//! nn:=-1; bottom_line:=false;
//! loop@+begin cur_input:=input_stack[base_ptr]; {enter into the context}
//!   if (state<>token_list) then
//!     if (name>17) or (base_ptr=0) then bottom_line:=true;
//!   if (base_ptr=input_ptr)or bottom_line or(nn<error_context_lines) then
//!     @<Display the current context@>
//!   else if nn=error_context_lines then
//!     begin print_nl("..."); incr(nn); {omitted if |error_context_lines<0|}
//!     end;
//!   if bottom_line then goto done;
//!   decr(base_ptr);
//!   end;
//! done: cur_input:=input_stack[input_ptr]; {restore original state}
//! end;
//!
//! @ @<Display the current context@>=
//! begin if (base_ptr=input_ptr) or (state<>token_list) or
//!    (token_type<>backed_up) or (loc<>null) then
//!     {we omit backed-up token lists that have already been read}
//!   begin tally:=0; {get ready to count characters}
//!   old_setting:=selector;
//!   if state<>token_list then
//!     begin @<Print location of current line@>;
//!     @<Pseudoprint the line@>;
//!     end
//!   else  begin @<Print type of token list@>;
//!     @<Pseudoprint the token list@>;
//!     end;
//!   selector:=old_setting; {stop pseudoprinting}
//!   @<Print two lines using the tricky pseudoprinted information@>;
//!   incr(nn);
//!   end;
//! end
//!
//! @ This routine should be changed, if necessary, to give the best possible
//! indication of where the current line resides in the input file.
//! For example, on some systems it is best to print both a page and line number.
//! @^system dependencies@>
//!
//! @<Print location of current line@>=
//! if name<=17 then
//!   if terminal_input then
//!     if base_ptr=0 then print_nl("<*>") else print_nl("<insert> ")
//!   else  begin print_nl("<read ");
//!     if name=17 then print_char("*")@+else print_int(name-1);
//! @.*\relax@>
//!     print_char(">");
//!     end
//! else  begin print_nl("l."); print_int(line);
//!   end;
//! print_char(" ")
//!
//! @ @<Print type of token list@>=
//! case token_type of
//! parameter: print_nl("<argument> ");
//! u_template,v_template: print_nl("<template> ");
//! backed_up: if loc=null then print_nl("<recently read> ")
//!   else print_nl("<to be read again> ");
//! inserted: print_nl("<inserted text> ");
//! macro: begin print_ln; print_cs(name);
//!   end;
//! output_text: print_nl("<output> ");
//! every_par_text: print_nl("<everypar> ");
//! every_math_text: print_nl("<everymath> ");
//! every_display_text: print_nl("<everydisplay> ");
//! every_hbox_text: print_nl("<everyhbox> ");
//! every_vbox_text: print_nl("<everyvbox> ");
//! every_job_text: print_nl("<everyjob> ");
//! every_cr_text: print_nl("<everycr> ");
//! mark_text: print_nl("<mark> ");
//! write_text: print_nl("<write> ");
//! othercases print_nl("?") {this should never happen}
//! endcases
//!
//! @ Here it is necessary to explain a little trick. We don't want to store a long
//! string that corresponds to a token list, because that string might take up
//! lots of memory; and we are printing during a time when an error message is
//! being given, so we dare not do anything that might overflow one of \TeX's
//! tables. So `pseudoprinting' is the answer: We enter a mode of printing
//! that stores characters into a buffer of length |error_line|, where character
//! $k+1$ is placed into \hbox{|trick_buf[k mod error_line]|} if
//! |k<trick_count|, otherwise character |k| is dropped. Initially we set
//! |tally:=0| and |trick_count:=1000000|; then when we reach the
//! point where transition from line 1 to line 2 should occur, we
//! set |first_count:=tally| and |trick_count:=@tmax@>(error_line,
//! tally+1+error_line-half_error_line)|. At the end of the
//! pseudoprinting, the values of |first_count|, |tally|, and
//! |trick_count| give us all the information we need to print the two lines,
//! and all of the necessary text is in |trick_buf|.
//!
//! Namely, let |l| be the length of the descriptive information that appears
//! on the first line. The length of the context information gathered for that
//! line is |k=first_count|, and the length of the context information
//! gathered for line~2 is $m=\min(|tally|, |trick_count|)-k$. If |l+k<=h|,
//! where |h=half_error_line|, we print |trick_buf[0..k-1]| after the
//! descriptive information on line~1, and set |n:=l+k|; here |n| is the
//! length of line~1. If $l+k>h$, some cropping is necessary, so we set |n:=h|
//! and print `\.{...}' followed by
//! $$\hbox{|trick_buf[(l+k-h+3)..k-1]|,}$$
//! where subscripts of |trick_buf| are circular modulo |error_line|. The
//! second line consists of |n|~spaces followed by |trick_buf[k..(k+m-1)]|,
//! unless |n+m>error_line|; in the latter case, further cropping is done.
//! This is easier to program than to explain.
//!
//! @<Local variables for formatting...@>=
//! @!i:0..buf_size; {index into |buffer|}
//! @!j:0..buf_size; {end of current line in |buffer|}
//! @!l:0..half_error_line; {length of descriptive information on line 1}
//! @!m:integer; {context information gathered for line 2}
//! @!n:0..error_line; {length of line 1}
//! @!p: integer; {starting or ending place in |trick_buf|}
//! @!q: integer; {temporary index}
//!
//! @ The following code sets up the print routines so that they will gather
//! the desired information.
//!
//! @d begin_pseudoprint==
//!   begin l:=tally; tally:=0; selector:=pseudo;
//!   trick_count:=1000000;
//!   end
//! @d set_trick_count==
//!   begin first_count:=tally;
//!   trick_count:=tally+1+error_line-half_error_line;
//!   if trick_count<error_line then trick_count:=error_line;
//!   end
//!
//! @ And the following code uses the information after it has been gathered.
//!
//! @<Print two lines using the tricky pseudoprinted information@>=
//! if trick_count=1000000 then set_trick_count;
//!   {|set_trick_count| must be performed}
//! if tally<trick_count then m:=tally-first_count
//! else m:=trick_count-first_count; {context on line 2}
//! if l+first_count<=half_error_line then
//!   begin p:=0; n:=l+first_count;
//!   end
//! else  begin print("..."); p:=l+first_count-half_error_line+3;
//!   n:=half_error_line;
//!   end;
//! for q:=p to first_count-1 do print_char(trick_buf[q mod error_line]);
//! print_ln;
//! for q:=1 to n do print_char(" "); {print |n| spaces to begin line~2}
//! if m+n<=error_line then p:=first_count+m else p:=first_count+(error_line-n-3);
//! for q:=first_count to p-1 do print_char(trick_buf[q mod error_line]);
//! if m+n>error_line then print("...")
//!
//! @ But the trick is distracting us from our current goal, which is to
//! understand the input state. So let's concentrate on the data structures that
//! are being pseudoprinted as we finish up the |show_context| procedure.
//!
//! @<Pseudoprint the line@>=
//! begin_pseudoprint;
//! if buffer[limit]=end_line_char then j:=limit
//! else j:=limit+1; {determine the effective end of the line}
//! if j>0 then for i:=start to j-1 do
//!   begin if i=loc then set_trick_count;
//!   print(buffer[i]);
//!   end
//!
//! @ @<Pseudoprint the token list@>=
//! begin_pseudoprint;
//! if token_type<macro then show_token_list(start,loc,100000)
//! else show_token_list(link(start),loc,100000) {avoid reference count}
//!
//! @ Here is the missing piece of |show_token_list| that is activated when the
//! token beginning line~2 is about to be shown:
//!
//! @<Do magic computation@>=set_trick_count
//!
