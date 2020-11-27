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
