//! @* \[20] Token lists.
//! A \TeX\ token is either a character or a control sequence, and it is
//! @^token@>
//! represented internally in one of two ways: (1)~A character whose ASCII
//! code number is |c| and whose command code is |m| is represented as the
//! number $2^8m+c$; the command code is in the range |1<=m<=14|. (2)~A control
//! sequence whose |eqtb| address is |p| is represented as the number
//! |cs_token_flag+p|. Here |cs_token_flag=@t$2^{12}-1$@>| is larger than
//! $2^8m+c$, yet it is small enough that |cs_token_flag+p< max_halfword|;
//! thus, a token fits comfortably in a halfword.
//!
//! A token |t| represents a |left_brace| command if and only if
//! |t<left_brace_limit|; it represents a |right_brace| command if and only if
//! we have |left_brace_limit<=t<right_brace_limit|; and it represents a |match| or
//! |end_match| command if and only if |match_token<=t<=end_match_token|.
//! The following definitions take care of these token-oriented constants
//! and a few others.
//!
//! @d cs_token_flag==@'7777 {amount added to the |eqtb| location in a
//!   token that stands for a control sequence; is a multiple of~256, less~1}
//! @d left_brace_token=@'0400 {$2^8\cdot|left_brace|$}
//! @d left_brace_limit=@'1000 {$2^8\cdot(|left_brace|+1)$}
//! @d right_brace_token=@'1000 {$2^8\cdot|right_brace|$}
//! @d right_brace_limit=@'1400 {$2^8\cdot(|right_brace|+1)$}
//! @d math_shift_token=@'1400 {$2^8\cdot|math_shift|$}
//! @d tab_token=@'2000 {$2^8\cdot|tab_mark|$}
//! @d out_param_token=@'2400 {$2^8\cdot|out_param|$}
//! @d space_token=@'5040 {$2^8\cdot|spacer|+|" "|$}
//! @d letter_token=@'5400 {$2^8\cdot|letter|$}
//! @d other_token=@'6000 {$2^8\cdot|other_char|$}
//! @d match_token=@'6400 {$2^8\cdot|match|$}
//! @d end_match_token=@'7000 {$2^8\cdot|end_match|$}
//!
//! @ @<Check the ``constant''...@>=
//! if cs_token_flag+undefined_control_sequence>max_halfword then bad:=21;
//!
//! @ A token list is a singly linked list of one-word nodes in |mem|, where
//! each word contains a token and a link. Macro definitions, output-routine
//! definitions, marks, \.{\\write} texts, and a few other things
//! are remembered by \TeX\ in the form
//! of token lists, usually preceded by a node with a reference count in its
//! |token_ref_count| field. The token stored in location |p| is called
//! |info(p)|.
//!
//! Three special commands appear in the token lists of macro definitions.
//! When |m=match|, it means that \TeX\ should scan a parameter
//! for the current macro; when |m=end_match|, it means that parameter
//! matching should end and \TeX\ should start reading the macro text; and
//! when |m=out_param|, it means that \TeX\ should insert parameter
//! number |c| into the text at this point.
//!
//! The enclosing \.{\char'173} and \.{\char'175} characters of a macro
//! definition are omitted, but the final right brace of an output routine
//! is included at the end of its token list.
//!
//! Here is an example macro definition that illustrates these conventions.
//! After \TeX\ processes the text
//! $$\.{\\def\\mac a\#1\#2 \\b \{\#1\\-a \#\#1\#2 \#2\}}$$
//! the definition of \.{\\mac} is represented as a token list containing
//! $$\def\,{\hskip2pt}
//! \vbox{\halign{\hfil#\hfil\cr
//! (reference count), |letter|\,\.a, |match|\,\#, |match|\,\#, |spacer|\,\.\ ,
//! \.{\\b}, |end_match|,\cr
//! |out_param|\,1, \.{\\-}, |letter|\,\.a, |spacer|\,\.\ , |mac_param|\,\#,
//! |other_char|\,\.1,\cr
//! |out_param|\,2, |spacer|\,\.\ , |out_param|\,2.\cr}}$$
//! The procedure |scan_toks| builds such token lists, and |macro_call|
//! does the parameter matching.
//! @^reference counts@>
//!
//! Examples such as
//! $$\.{\\def\\m\{\\def\\m\{a\}\ b\}}$$
//! explain why reference counts would be needed even if \TeX\ had no \.{\\let}
//! operation: When the token list for \.{\\m} is being read, the redefinition of
//! \.{\\m} changes the |eqtb| entry before the token list has been fully
//! consumed, so we dare not simply destroy a token list when its
//! control sequence is being redefined.
//!
//! If the parameter-matching part of a definition ends with `\.{\#\{}',
//! the corresponding token list will have `\.\{' just before the `|end_match|'
//! and also at the very end. The first `\.\{' is used to delimit the parameter; the
//! second one keeps the first from disappearing.
//!
//! @ The procedure |show_token_list|, which prints a symbolic form of
//! the token list that starts at a given node |p|, illustrates these
//! conventions. The token list being displayed should not begin with a reference
//! count. However, the procedure is intended to be robust, so that if the
//! memory links are awry or if |p| is not really a pointer to a token list,
//! nothing catastrophic will happen.
//!
//! An additional parameter |q| is also given; this parameter is either null
//! or it points to a node in the token list where a certain magic computation
//! takes place that will be explained later. (Basically, |q| is non-null when
//! we are printing the two-line context information at the time of an error
//! message; |q| marks the place corresponding to where the second line
//! should begin.)
//!
//! For example, if |p| points to the node containing the first \.a in the
//! token list above, then |show_token_list| will print the string
//! $$\hbox{`\.{a\#1\#2\ \\b\ ->\#1\\-a\ \#\#1\#2\ \#2}';}$$
//! and if |q| points to the node containing the second \.a,
//! the magic computation will be performed just before the second \.a is printed.
//!
//! The generation will stop, and `\.{\\ETC.}' will be printed, if the length
//! of printing exceeds a given limit~|l|. Anomalous entries are printed in the
//! form of control sequences that are not followed by a blank space, e.g.,
//! `\.{\\BAD.}'; this cannot be confused with actual control sequences because
//! a real control sequence named \.{BAD} would come out `\.{\\BAD\ }'.
//!
//! @<Declare the procedure called |show_token_list|@>=
//! procedure show_token_list(@!p,@!q:integer;@!l:integer);
//! label exit;
//! var m,@!c:integer; {pieces of a token}
//! @!match_chr:ASCII_code; {character used in a `|match|'}
//! @!n:ASCII_code; {the highest parameter number, as an ASCII digit}
//! begin match_chr:="#"; n:="0"; tally:=0;
//! while (p<>null) and (tally<l) do
//!   begin if p=q then @<Do magic computation@>;
//!   @<Display token |p|, and |return| if there are problems@>;
//!   p:=link(p);
//!   end;
//! if p<>null then print_esc("ETC.");
//! @.ETC@>
//! exit:
//! end;
//!
//! @ @<Display token |p|...@>=
//! if (p<hi_mem_min) or (p>mem_end) then
//!   begin print_esc("CLOBBERED."); return;
//! @.CLOBBERED@>
//!   end;
//! if info(p)>=cs_token_flag then print_cs(info(p)-cs_token_flag)
//! else  begin m:=info(p) div @'400; c:=info(p) mod @'400;
//!   if info(p)<0 then print_esc("BAD.")
//! @.BAD@>
//!   else @<Display the token $(|m|,|c|)$@>;
//!   end
//!
//! @ The procedure usually ``learns'' the character code used for macro
//! parameters by seeing one in a |match| command before it runs into any
//! |out_param| commands.
//!
//! @<Display the token ...@>=
//! case m of
//! left_brace,right_brace,math_shift,tab_mark,sup_mark,sub_mark,spacer,
//!   letter,other_char: print(c);
//! mac_param: begin print(c); print(c);
//!   end;
//! out_param: begin print(match_chr);
//!   if c<=9 then print_char(c+"0")
//!   else  begin print_char("!"); return;
//!     end;
//!   end;
//! match: begin match_chr:=c; print(c); incr(n); print_char(n);
//!   if n>"9" then return;
//!   end;
//! end_match: print("->");
//! @.->@>
//! othercases print_esc("BAD.")
//! @.BAD@>
//! endcases
//!
//! @ Here's the way we sometimes want to display a token list, given a pointer
//! to its reference count; the pointer may be null.
//!
//! @p procedure token_show(@!p:pointer);
//! begin if p<>null then show_token_list(link(p),null,10000000);
//! end;
//!
//! @ The |print_meaning| subroutine displays |cur_cmd| and |cur_chr| in
//! symbolic form, including the expansion of a macro or mark.
//!
//! @p procedure print_meaning;
//! begin print_cmd_chr(cur_cmd,cur_chr);
//! if cur_cmd>=call then
//!   begin print_char(":"); print_ln; token_show(cur_chr);
//!   end
//! else if cur_cmd=top_bot_mark then
//!   begin print_char(":"); print_ln;
//!   token_show(cur_mark[cur_chr]);
//!   end;
//! end;
//!
