//! @ The global variable |force_eof| is normally |false|; it is set |true|
//! by an \.{\\endinput} command.
//!
//! @<Glob...@>=
//! @!force_eof:boolean; {should the next \.{\\input} be aborted early?}
//!
//! @ @<Read next line of file into |buffer|, or
//!   |goto restart| if the file has ended@>=
//! begin incr(line); first:=start;
//! if not force_eof then
//!   begin if input_ln(cur_file,true) then {not end of file}
//!     firm_up_the_line {this sets |limit|}
//!   else force_eof:=true;
//!   end;
//! if force_eof then
//!   begin print_char(")"); decr(open_parens);
//!   update_terminal; {show user that file has been read}
//!   force_eof:=false;
//!   end_file_reading; {resume previous level}
//!   check_outer_validity; goto restart;
//!   end;
//! if end_line_char_inactive then decr(limit)
//! else  buffer[limit]:=end_line_char;
//! first:=limit+1; loc:=start; {ready to read}
//! end
//!
//! @ If the user has set the |pausing| parameter to some positive value,
//! and if nonstop mode has not been selected, each line of input is displayed
//! on the terminal and the transcript file, followed by `\.{=>}'.
//! \TeX\ waits for a response. If the response is simply |carriage_return|, the
//! line is accepted as it stands, otherwise the line typed is
//! used instead of the line in the file.
//!
//! @p procedure firm_up_the_line;
//! var k:0..buf_size; {an index into |buffer|}
//! begin limit:=last;
//! if pausing>0 then if interaction>nonstop_mode then
//!   begin wake_up_terminal; print_ln;
//!   if start<limit then for k:=start to limit-1 do print(buffer[k]);
//!   first:=limit; prompt_input("=>"); {wait for user response}
//! @.=>@>
//!   if last>first then
//!     begin for k:=first to last-1 do {move line down in buffer}
//!       buffer[k+start-first]:=buffer[k];
//!     limit:=start+last-first;
//!     end;
//!   end;
//! end;
//!
//! @ Since |get_next| is used so frequently in \TeX, it is convenient
//! to define three related procedures that do a little more:
//!
//! \yskip\hang|get_token| not only sets |cur_cmd| and |cur_chr|, it
//! also sets |cur_tok|, a packed halfword version of the current token.
//!
//! \yskip\hang|get_x_token|, meaning ``get an expanded token,'' is like
//! |get_token|, but if the current token turns out to be a user-defined
//! control sequence (i.e., a macro call), or a conditional,
//! or something like \.{\\topmark} or \.{\\expandafter} or \.{\\csname},
//! it is eliminated from the input by beginning the expansion of the macro
//! or the evaluation of the conditional.
//!
//! \yskip\hang|x_token| is like |get_x_token| except that it assumes that
//! |get_next| has already been called.
//!
//! \yskip\noindent
//! In fact, these three procedures account for almost every use of |get_next|.
//!
//! @ No new control sequences will be defined except during a call of
//! |get_token|, or when \.{\\csname} compresses a token list, because
//! |no_new_control_sequence| is always |true| at other times.
//!
//! @p procedure get_token; {sets |cur_cmd|, |cur_chr|, |cur_tok|}
//! begin no_new_control_sequence:=false; get_next; no_new_control_sequence:=true;
//! @^inner loop@>
//! if cur_cs=0 then cur_tok:=(cur_cmd*@'400)+cur_chr
//! else cur_tok:=cs_token_flag+cur_cs;
//! end;
//!
