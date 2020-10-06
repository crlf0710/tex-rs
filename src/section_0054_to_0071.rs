//! @* \[5] On-line and off-line printing.
//! Messages that are sent to a user's terminal and to the transcript-log file
//! are produced by several `|print|' procedures. These procedures will
//! direct their output to a variety of places, based on the setting of
//! the global variable |selector|, which has the following possible
//! values:
//!
//! \yskip
//! \hang |term_and_log|, the normal setting, prints on the terminal and on the
//!   transcript file.
//!
//! \hang |log_only|, prints only on the transcript file.
//!
//! \hang |term_only|, prints only on the terminal.
//!
//! \hang |no_print|, doesn't print at all. This is used only in rare cases
//!   before the transcript file is open.
//!
//! \hang |pseudo|, puts output into a cyclic buffer that is used
//!   by the |show_context| routine; when we get to that routine we shall discuss
//!   the reasoning behind this curious mode.
//!
//! \hang |new_string|, appends the output to the current string in the
//!   string pool.
//!
//! \hang 0 to 15, prints on one of the sixteen files for \.{\\write} output.
//!
//! \yskip
//! \noindent The symbolic names `|term_and_log|', etc., have been assigned
//! numeric codes that satisfy the convenient relations |no_print+1=term_only|,
//! |no_print+2=log_only|, |term_only+2=log_only+1=term_and_log|.
//!
//! Three additional global variables, |tally| and |term_offset| and
//! |file_offset|, record the number of characters that have been printed
//! since they were most recently cleared to zero. We use |tally| to record
//! the length of (possibly very long) stretches of printing; |term_offset|
//! and |file_offset|, on the other hand, keep track of how many characters
//! have appeared so far on the current line that has been output to the
//! terminal or to the transcript file, respectively.
//!
//! @d no_print=16 {|selector| setting that makes data disappear}
//! @d term_only=17 {printing is destined for the terminal only}
//! @d log_only=18 {printing is destined for the transcript file only}
//! @d term_and_log=19 {normal |selector| setting}
//! @d pseudo=20 {special |selector| setting for |show_context|}
//! @d new_string=21 {printing is deflected to the string pool}
//! @d max_selector=21 {highest selector setting}
//!
//! @<Glob...@>=
//! @!log_file : alpha_file; {transcript of \TeX\ session}
//! @!selector : 0..max_selector; {where to print a message}
//! @!dig : array[0..22] of 0..15; {digits in a number being output}
//! @!tally : integer; {the number of characters recently printed}
//! @!term_offset : 0..max_print_line;
//!   {the number of characters on the current terminal line}
//! @!file_offset : 0..max_print_line;
//!   {the number of characters on the current file line}
//! @!trick_buf:array[0..error_line] of ASCII_code; {circular buffer for
//!   pseudoprinting}
//! @!trick_count: integer; {threshold for pseudoprinting, explained later}
//! @!first_count: integer; {another variable for pseudoprinting}
//!
//! @ @<Initialize the output routines@>=
//! selector:=term_only; tally:=0; term_offset:=0; file_offset:=0;
//!
//! @ Macro abbreviations for output to the terminal and to the log file are
//! defined here for convenience. Some systems need special conventions
//! for terminal output, and it is possible to adhere to those conventions
//! by changing |wterm|, |wterm_ln|, and |wterm_cr| in this section.
//! @^system dependencies@>
//!
//! @d wterm(#)==write(term_out,#)
//! @d wterm_ln(#)==write_ln(term_out,#)
//! @d wterm_cr==write_ln(term_out)
//! @d wlog(#)==write(log_file,#)
//! @d wlog_ln(#)==write_ln(log_file,#)
//! @d wlog_cr==write_ln(log_file)
//!
//! @ To end a line of text output, we call |print_ln|.
//!
//! @<Basic print...@>=
//! procedure print_ln; {prints an end-of-line}
//! begin case selector of
//! term_and_log: begin wterm_cr; wlog_cr;
//!   term_offset:=0; file_offset:=0;
//!   end;
//! log_only: begin wlog_cr; file_offset:=0;
//!   end;
//! term_only: begin wterm_cr; term_offset:=0;
//!   end;
//! no_print,pseudo,new_string: do_nothing;
//! othercases write_ln(write_file[selector])
//! endcases;@/
//! end; {|tally| is not affected}
//!
//! @ The |print_char| procedure sends one character to the desired destination,
//! using the |xchr| array to map it into an external character compatible with
//! |input_ln|. All printing comes through |print_ln| or |print_char|.
//!
//! @<Basic printing...@>=
//! procedure print_char(@!s:ASCII_code); {prints a single character}
//! label exit;
//! begin if @<Character |s| is the current new-line character@> then
//!  if selector<pseudo then
//!   begin print_ln; return;
//!   end;
//! case selector of
//! term_and_log: begin wterm(xchr[s]); wlog(xchr[s]);
//!   incr(term_offset); incr(file_offset);
//!   if term_offset=max_print_line then
//!     begin wterm_cr; term_offset:=0;
//!     end;
//!   if file_offset=max_print_line then
//!     begin wlog_cr; file_offset:=0;
//!     end;
//!   end;
//! log_only: begin wlog(xchr[s]); incr(file_offset);
//!   if file_offset=max_print_line then print_ln;
//!   end;
//! term_only: begin wterm(xchr[s]); incr(term_offset);
//!   if term_offset=max_print_line then print_ln;
//!   end;
//! no_print: do_nothing;
//! pseudo: if tally<trick_count then trick_buf[tally mod error_line]:=s;
//! new_string: begin if pool_ptr<pool_size then append_char(s);
//!   end; {we drop characters if the string space is full}
//! othercases write(write_file[selector],xchr[s])
//! endcases;@/
//! incr(tally);
//! exit:end;
//!
//! @ An entire string is output by calling |print|. Note that if we are outputting
//! the single standard ASCII character \.c, we could call |print("c")|, since
//! |"c"=99| is the number of a single-character string, as explained above. But
//! |print_char("c")| is quicker, so \TeX\ goes directly to the |print_char|
//! routine when it knows that this is safe. (The present implementation
//! assumes that it is always safe to print a visible ASCII character.)
//! @^system dependencies@>
//!
//! @<Basic print...@>=
//! procedure print(@!s:integer); {prints string |s|}
//! label exit;
//! var j:pool_pointer; {current character code position}
//! @!nl:integer; {new-line character to restore}
//! begin if s>=str_ptr then s:="???" {this can't happen}
//! @.???@>
//! else if s<256 then
//!   if s<0 then s:="???" {can't happen}
//!   else begin if selector>pseudo then
//!       begin print_char(s); return; {internal strings are not expanded}
//!       end;
//!     if (@<Character |s| is the current new-line character@>) then
//!       if selector<pseudo then
//!         begin print_ln; return;
//!         end;
//!     nl:=new_line_char; new_line_char:=-1;
//!       {temporarily disable new-line character}
//!     j:=str_start[s];
//!     while j<str_start[s+1] do
//!       begin print_char(so(str_pool[j])); incr(j);
//!       end;
//!     new_line_char:=nl; return;
//!     end;
//! j:=str_start[s];
//! while j<str_start[s+1] do
//!   begin print_char(so(str_pool[j])); incr(j);
//!   end;
//! exit:end;
//!
//! @ Control sequence names, file names, and strings constructed with
//! \.{\\string} might contain |ASCII_code| values that can't
//! be printed using |print_char|. Therefore we use |slow_print| for them:
//!
//! @<Basic print...@>=
//! procedure slow_print(@!s:integer); {prints string |s|}
//! var j:pool_pointer; {current character code position}
//! begin if (s>=str_ptr) or (s<256) then print(s)
//! else begin j:=str_start[s];
//!   while j<str_start[s+1] do
//!     begin print(so(str_pool[j])); incr(j);
//!     end;
//!   end;
//! end;
//!
//! @ Here is the very first thing that \TeX\ prints: a headline that identifies
//! the version number and format package. The |term_offset| variable is temporarily
//! incorrect, but the discrepancy is not serious since we assume that the banner
//! and format identifier together will occupy at most |max_print_line|
//! character positions.
//!
//! @<Initialize the output...@>=
//! wterm(banner);
//! if format_ident=0 then wterm_ln(' (no format preloaded)')
//! else  begin slow_print(format_ident); print_ln;
//!   end;
//! update_terminal;
//!
//! @ The procedure |print_nl| is like |print|, but it makes sure that the
//! string appears at the beginning of a new line.
//!
//! @<Basic print...@>=
//! procedure print_nl(@!s:str_number); {prints string |s| at beginning of line}
//! begin if ((term_offset>0)and(odd(selector)))or@|
//!   ((file_offset>0)and(selector>=log_only)) then print_ln;
//! print(s);
//! end;
//!
//! @ The procedure |print_esc| prints a string that is preceded by
//! the user's escape character (which is usually a backslash).
//!
//! @<Basic print...@>=
//! procedure print_esc(@!s:str_number); {prints escape character, then |s|}
//! var c:integer; {the escape character code}
//! begin  @<Set variable |c| to the current escape character@>;
//! if c>=0 then if c<256 then print(c);
//! slow_print(s);
//! end;
//!
//! @ An array of digits in the range |0..15| is printed by |print_the_digs|.
//!
//! @<Basic print...@>=
//! procedure print_the_digs(@!k:eight_bits);
//!   {prints |dig[k-1]|$\,\ldots\,$|dig[0]|}
//! begin while k>0 do
//!   begin decr(k);
//!   if dig[k]<10 then print_char("0"+dig[k])
//!   else print_char("A"-10+dig[k]);
//!   end;
//! end;
//!
//! @ The following procedure, which prints out the decimal representation of a
//! given integer |n|, has been written carefully so that it works properly
//! if |n=0| or if |(-n)| would cause overflow. It does not apply |mod| or |div|
//! to negative arguments, since such operations are not implemented consistently
//! by all \PASCAL\ compilers.
//!
//! @<Basic print...@>=
//! procedure print_int(@!n:integer); {prints an integer in decimal form}
//! var k:0..23; {index to current digit; we assume that $|n|<10^{23}$}
//! @!m:integer; {used to negate |n| in possibly dangerous cases}
//! begin k:=0;
//! if n<0 then
//!   begin print_char("-");
//!   if n>-100000000 then negate(n)
//!   else  begin m:=-1-n; n:=m div 10; m:=(m mod 10)+1; k:=1;
//!     if m<10 then dig[0]:=m
//!     else  begin dig[0]:=0; incr(n);
//!       end;
//!     end;
//!   end;
//! repeat dig[k]:=n mod 10; n:=n div 10; incr(k);
//! until n=0;
//! print_the_digs(k);
//! end;
//!
//! @ Here is a trivial procedure to print two digits; it is usually called with
//! a parameter in the range |0<=n<=99|.
//!
//! @p procedure print_two(@!n:integer); {prints two least significant digits}
//! begin n:=abs(n) mod 100; print_char("0"+(n div 10));
//! print_char("0"+(n mod 10));
//! end;
//!
//! @ Hexadecimal printing of nonnegative integers is accomplished by |print_hex|.
//!
//! @p procedure print_hex(@!n:integer);
//!   {prints a positive integer in hexadecimal form}
//! var k:0..22; {index to current digit; we assume that $0\L n<16^{22}$}
//! begin k:=0; print_char("""");
//! repeat dig[k]:=n mod 16; n:=n div 16; incr(k);
//! until n=0;
//! print_the_digs(k);
//! end;
//!
//! @ Old versions of \TeX\ needed a procedure called |print_ASCII| whose function
//! is now subsumed by |print|. We retain the old name here as a possible aid to
//! future software arch\ae ologists.
//!
//! @d print_ASCII == print
//!
//! @ Roman numerals are produced by the |print_roman_int| routine.  Readers
//! who like puzzles might enjoy trying to figure out how this tricky code
//! works; therefore no explanation will be given. Notice that 1990 yields
//! \.{mcmxc}, not \.{mxm}.
//!
//! @p procedure print_roman_int(@!n:integer);
//! label exit;
//! var j,@!k: pool_pointer; {mysterious indices into |str_pool|}
//! @!u,@!v: nonnegative_integer; {mysterious numbers}
//! begin j:=str_start["m2d5c2l5x2v5i"]; v:=1000;
//! loop@+  begin while n>=v do
//!     begin print_char(so(str_pool[j])); n:=n-v;
//!     end;
//!   if n<=0 then return; {nonpositive input produces no output}
//!   k:=j+2; u:=v div (so(str_pool[k-1])-"0");
//!   if str_pool[k-1]=si("2") then
//!     begin k:=k+2; u:=u div (so(str_pool[k-1])-"0");
//!     end;
//!   if n+u>=v then
//!     begin print_char(so(str_pool[k])); n:=n+u;
//!     end
//!   else  begin j:=j+2; v:=v div (so(str_pool[j-1])-"0");
//!     end;
//!   end;
//! exit:end;
//!
//! @ The |print| subroutine will not print a string that is still being
//! created. The following procedure will.
//!
//! @p procedure print_current_string; {prints a yet-unmade string}
//! var j:pool_pointer; {points to current character code}
//! begin j:=str_start[str_ptr];
//! while j<pool_ptr do
//!   begin print_char(so(str_pool[j])); incr(j);
//!   end;
//! end;
//!
//! @ Here is a procedure that asks the user to type a line of input,
//! assuming that the |selector| setting is either |term_only| or |term_and_log|.
//! The input is placed into locations |first| through |last-1| of the
//! |buffer| array, and echoed on the transcript file if appropriate.
//!
//! This procedure is never called when |interaction<scroll_mode|.
//!
//! @d prompt_input(#)==begin wake_up_terminal; print(#); term_input;
//!     end {prints a string and gets a line of input}
//!
//! @p procedure term_input; {gets a line from the terminal}
//! var k:0..buf_size; {index into |buffer|}
//! begin update_terminal; {now the user sees the prompt for sure}
//! if not input_ln(term_in,true) then fatal_error("End of file on the terminal!");
//! @.End of file on the terminal@>
//! term_offset:=0; {the user's line ended with \<\rm return>}
//! decr(selector); {prepare to echo the input}
//! if last<>first then for k:=first to last-1 do print(buffer[k]);
//! print_ln; incr(selector); {restore previous status}
//! end;
//!
