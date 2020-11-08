//! @ A control sequence that has been \.{\\def}'ed by the user is expanded by
//! \TeX's |macro_call| procedure.
//!
//! Before we get into the details of |macro_call|, however, let's consider the
//! treatment of primitives like \.{\\topmark}, since they are essentially
//! macros without parameters. The token lists for such marks are kept in a
//! global array of five pointers; we refer to the individual entries of this
//! array by symbolic names |top_mark|, etc. The value of |top_mark| is either
//! |null| or a pointer to the reference count of a token list.
//!
//! @d top_mark_code=0 {the mark in effect at the previous page break}
//! @d first_mark_code=1 {the first mark between |top_mark| and |bot_mark|}
//! @d bot_mark_code=2 {the mark in effect at the current page break}
//! @d split_first_mark_code=3 {the first mark found by \.{\\vsplit}}
//! @d split_bot_mark_code=4 {the last mark found by \.{\\vsplit}}
//! @d top_mark==cur_mark[top_mark_code]
//! @d first_mark==cur_mark[first_mark_code]
//! @d bot_mark==cur_mark[bot_mark_code]
//! @d split_first_mark==cur_mark[split_first_mark_code]
//! @d split_bot_mark==cur_mark[split_bot_mark_code]
//!
//! @<Glob...@>=
//! @!cur_mark:array[top_mark_code..split_bot_mark_code] of pointer;
//!   {token lists for marks}
//!
//! @ @<Set init...@>=
//! top_mark:=null; first_mark:=null; bot_mark:=null;
//! split_first_mark:=null; split_bot_mark:=null;
//!
//! @ @<Put each...@>=
//! primitive("topmark",top_bot_mark,top_mark_code);
//! @!@:top_mark_}{\.{\\topmark} primitive@>
//! primitive("firstmark",top_bot_mark,first_mark_code);
//! @!@:first_mark_}{\.{\\firstmark} primitive@>
//! primitive("botmark",top_bot_mark,bot_mark_code);
//! @!@:bot_mark_}{\.{\\botmark} primitive@>
//! primitive("splitfirstmark",top_bot_mark,split_first_mark_code);
//! @!@:split_first_mark_}{\.{\\splitfirstmark} primitive@>
//! primitive("splitbotmark",top_bot_mark,split_bot_mark_code);
//! @!@:split_bot_mark_}{\.{\\splitbotmark} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! top_bot_mark: case chr_code of
//!   first_mark_code: print_esc("firstmark");
//!   bot_mark_code: print_esc("botmark");
//!   split_first_mark_code: print_esc("splitfirstmark");
//!   split_bot_mark_code: print_esc("splitbotmark");
//!   othercases print_esc("topmark")
//!   endcases;
//!
//! @ The following code is activated when |cur_cmd=top_bot_mark| and
//! when |cur_chr| is a code like |top_mark_code|.
//!
//! @<Insert the \(a)appropriate mark text into the scanner@>=
//! begin if cur_mark[cur_chr]<>null then
//!   begin_token_list(cur_mark[cur_chr],mark_text);
//! end
//!
//! @ Now let's consider |macro_call| itself, which is invoked when \TeX\ is
//! scanning a control sequence whose |cur_cmd| is either |call|, |long_call|,
//! |outer_call|, or |long_outer_call|.  The control sequence definition
//! appears in the token list whose reference count is in location |cur_chr|
//! of |mem|.
//!
//! The global variable |long_state| will be set to |call| or to |long_call|,
//! depending on whether or not the control sequence disallows \.{\\par}
//! in its parameters. The |get_next| routine will set |long_state| to
//! |outer_call| and emit \.{\\par}, if a file ends or if an \.{\\outer}
//! control sequence occurs in the midst of an argument.
//!
//! @<Glob...@>=
//! @!long_state:call..long_outer_call; {governs the acceptance of \.{\\par}}
//!
//! @ The parameters, if any, must be scanned before the macro is expanded.
//! Parameters are token lists without reference counts. They are placed on
//! an auxiliary stack called |pstack| while they are being scanned, since
//! the |param_stack| may be losing entries during the matching process.
//! (Note that |param_stack| can't be gaining entries, since |macro_call| is
//! the only routine that puts anything onto |param_stack|, and it
//! is not recursive.)
//!
//! @<Glob...@>=
//! @!pstack:array[0..8] of pointer; {arguments supplied to a macro}
//!
//! @ After parameter scanning is complete, the parameters are moved to the
//! |param_stack|. Then the macro body is fed to the scanner; in other words,
//! |macro_call| places the defined text of the control sequence at the
//! top of\/ \TeX's input stack, so that |get_next| will proceed to read it
//! next.
//!
//! The global variable |cur_cs| contains the |eqtb| address of the control sequence
//! being expanded, when |macro_call| begins. If this control sequence has not been
//! declared \.{\\long}, i.e., if its command code in the |eq_type| field is
//! not |long_call| or |long_outer_call|, its parameters are not allowed to contain
//! the control sequence \.{\\par}. If an illegal \.{\\par} appears, the macro
//! call is aborted, and the \.{\\par} will be rescanned.
//!
//! @<Declare the procedure called |macro_call|@>=
//! procedure macro_call; {invokes a user-defined control sequence}
//! label exit, continue, done, done1, found;
//! var r:pointer; {current node in the macro's token list}
//! @!p:pointer; {current node in parameter token list being built}
//! @!q:pointer; {new node being put into the token list}
//! @!s:pointer; {backup pointer for parameter matching}
//! @!t:pointer; {cycle pointer for backup recovery}
//! @!u,@!v:pointer; {auxiliary pointers for backup recovery}
//! @!rbrace_ptr:pointer; {one step before the last |right_brace| token}
//! @!n:small_number; {the number of parameters scanned}
//! @!unbalance:halfword; {unmatched left braces in current parameter}
//! @!m:halfword; {the number of tokens or groups (usually)}
//! @!ref_count:pointer; {start of the token list}
//! @!save_scanner_status:small_number; {|scanner_status| upon entry}
//! @!save_warning_index:pointer; {|warning_index| upon entry}
//! @!match_chr:ASCII_code; {character used in parameter}
//! begin save_scanner_status:=scanner_status; save_warning_index:=warning_index;
//! warning_index:=cur_cs; ref_count:=cur_chr; r:=link(ref_count); n:=0;
//! if tracing_macros>0 then @<Show the text of the macro being expanded@>;
//! if info(r)<>end_match_token then
//!   @<Scan the parameters and make |link(r)| point to the macro body; but
//!     |return| if an illegal \.{\\par} is detected@>;
//! @<Feed the macro body and its parameters to the scanner@>;
//! exit:scanner_status:=save_scanner_status; warning_index:=save_warning_index;
//! end;
//!
//! @ Before we put a new token list on the input stack, it is wise to clean off
//! all token lists that have recently been depleted. Then a user macro that ends
//! with a call to itself will not require unbounded stack space.
//!
//! @<Feed the macro body and its parameters to the scanner@>=
//! while (state=token_list)and(loc=null)and(token_type<>v_template) do
//!   end_token_list; {conserve stack space}
//! begin_token_list(ref_count,macro); name:=warning_index; loc:=link(r);
//! if n>0 then
//!   begin if param_ptr+n>max_param_stack then
//!     begin max_param_stack:=param_ptr+n;
//!     if max_param_stack>param_size then
//!       overflow("parameter stack size",param_size);
//! @:TeX capacity exceeded parameter stack size}{\quad parameter stack size@>
//!     end;
//!   for m:=0 to n-1 do param_stack[param_ptr+m]:=pstack[m];
//!   param_ptr:=param_ptr+n;
//!   end
//!
//! @ At this point, the reader will find it advisable to review the explanation
//! of token list format that was presented earlier, since many aspects of that
//! format are of importance chiefly in the |macro_call| routine.
//!
//! The token list might begin with a string of compulsory tokens before the
//! first |match| or |end_match|. In that case the macro name is supposed to be
//! followed by those tokens; the following program will set |s=null| to
//! represent this restriction. Otherwise |s| will be set to the first token of
//! a string that will delimit the next parameter.
//!
//! @<Scan the parameters and make |link(r)| point to the macro body...@>=
//! begin scanner_status:=matching; unbalance:=0;
//! long_state:=eq_type(cur_cs);
//! if long_state>=outer_call then long_state:=long_state-2;
//! repeat link(temp_head):=null;
//! if (info(r)>match_token+255)or(info(r)<match_token) then s:=null
//! else  begin match_chr:=info(r)-match_token; s:=link(r); r:=s;
//!   p:=temp_head; m:=0;
//!   end;
//! @<Scan a parameter until its delimiter string has been found; or, if |s=null|,
//!   simply scan the delimiter string@>;@/
//! {now |info(r)| is a token whose command code is either |match| or |end_match|}
//! until info(r)=end_match_token;
//! end
//!
//! @ If |info(r)| is a |match| or |end_match| command, it cannot be equal to
//! any token found by |get_token|. Therefore an undelimited parameter---i.e.,
//! a |match| that is immediately followed by |match| or |end_match|---will
//! always fail the test `|cur_tok=info(r)|' in the following algorithm.
//!
//! @<Scan a parameter until its delimiter string has been found; or, ...@>=
//! continue: get_token; {set |cur_tok| to the next token of input}
//! if cur_tok=info(r) then
//!   @<Advance \(r)|r|; |goto found| if the parameter delimiter has been
//!     fully matched, otherwise |goto continue|@>;
//! @<Contribute the recently matched tokens to the current parameter, and
//!   |goto continue| if a partial match is still in effect;
//!   but abort if |s=null|@>;
//! if cur_tok=par_token then if long_state<>long_call then
//!   @<Report a runaway argument and abort@>;
//! if cur_tok<right_brace_limit then
//!   if cur_tok<left_brace_limit then
//!     @<Contribute an entire group to the current parameter@>
//!   else @<Report an extra right brace and |goto continue|@>
//! else @<Store the current token, but |goto continue| if it is
//!    a blank space that would become an undelimited parameter@>;
//! incr(m);
//! if info(r)>end_match_token then goto continue;
//! if info(r)<match_token then goto continue;
//! found: if s<>null then @<Tidy up the parameter just scanned, and tuck it away@>
//!
//! @ @<Store the current token, but |goto continue| if it is...@>=
//! begin if cur_tok=space_token then
//!   if info(r)<=end_match_token then
//!     if info(r)>=match_token then goto continue;
//! store_new_token(cur_tok);
//! end
//!
//! @ A slightly subtle point arises here: When the parameter delimiter ends
//! with `\.{\#\{}', the token list will have a left brace both before and
//! after the |end_match|\kern-.4pt. Only one of these should affect the
//! |align_state|, but both will be scanned, so we must make a correction.
//!
//! @<Advance \(r)|r|; |goto found| if the parameter delimiter has been fully...@>=
//! begin r:=link(r);
//! if (info(r)>=match_token)and(info(r)<=end_match_token) then
//!   begin if cur_tok<left_brace_limit then decr(align_state);
//!   goto found;
//!   end
//! else goto continue;
//! end
//!
//! @ @<Report an extra right brace and |goto continue|@>=
//! begin back_input; print_err("Argument of "); sprint_cs(warning_index);
//! @.Argument of \\x has...@>
//! print(" has an extra }");
//! help6("I've run across a `}' that doesn't seem to match anything.")@/
//!   ("For example, `\def\a#1{...}' and `\a}' would produce")@/
//!   ("this error. If you simply proceed now, the `\par' that")@/
//!   ("I've just inserted will cause me to report a runaway")@/
//!   ("argument that might be the root of the problem. But if")@/
//!   ("your `}' was spurious, just type `2' and it will go away.");
//! incr(align_state); long_state:=call; cur_tok:=par_token; ins_error;
//! goto continue;
//! end {a white lie; the \.{\\par} won't always trigger a runaway}
//!
//! @ If |long_state=outer_call|, a runaway argument has already been reported.
//!
//! @<Report a runaway argument and abort@>=
//! begin if long_state=call then
//!   begin runaway; print_err("Paragraph ended before ");
//! @.Paragraph ended before...@>
//!   sprint_cs(warning_index); print(" was complete");
//!   help3("I suspect you've forgotten a `}', causing me to apply this")@/
//!     ("control sequence to too much text. How can we recover?")@/
//!     ("My plan is to forget the whole thing and hope for the best.");
//!   back_error;
//!   end;
//! pstack[n]:=link(temp_head); align_state:=align_state-unbalance;
//! for m:=0 to n do flush_list(pstack[m]);
//! return;
//! end
//!
//! @ When the following code becomes active, we have matched tokens from |s| to
//! the predecessor of |r|, and we have found that |cur_tok<>info(r)|. An
//! interesting situation now presents itself: If the parameter is to be
//! delimited by a string such as `\.{ab}', and if we have scanned `\.{aa}',
//! we want to contribute one `\.a' to the current parameter and resume
//! looking for a `\.b'. The program must account for such partial matches and
//! for others that can be quite complex.  But most of the time we have |s=r|
//! and nothing needs to be done.
//!
//! Incidentally, it is possible for \.{\\par} tokens to sneak in to certain
//! parameters of non-\.{\\long} macros. For example, consider a case like
//! `\.{\\def\\a\#1\\par!\{...\}}' where the first \.{\\par} is not followed
//! by an exclamation point. In such situations it does not seem appropriate
//! to prohibit the \.{\\par}, so \TeX\ keeps quiet about this bending of
//! the rules.
//!
//! @<Contribute the recently matched tokens to the current parameter...@>=
//! if s<>r then
//!   if s=null then @<Report an improper use of the macro and abort@>
//!   else  begin t:=s;
//!     repeat store_new_token(info(t)); incr(m); u:=link(t); v:=s;
//!     loop@+  begin if u=r then
//!         if cur_tok<>info(v) then goto done
//!         else  begin r:=link(v); goto continue;
//!           end;
//!       if info(u)<>info(v) then goto done;
//!       u:=link(u); v:=link(v);
//!       end;
//!     done: t:=link(t);
//!     until t=r;
//!     r:=s; {at this point, no tokens are recently matched}
//!     end
//!
//! @ @<Report an improper use...@>=
//! begin print_err("Use of "); sprint_cs(warning_index);
//! @.Use of x doesn't match...@>
//! print(" doesn't match its definition");
//! help4("If you say, e.g., `\def\a1{...}', then you must always")@/
//!   ("put `1' after `\a', since control sequence names are")@/
//!   ("made up of letters only. The macro here has not been")@/
//!   ("followed by the required stuff, so I'm ignoring it.");
//! error; return;
//! end
//!
//! @ @<Contribute an entire group to the current parameter@>=
//! begin unbalance:=1;
//! @^inner loop@>
//! loop@+  begin fast_store_new_token(cur_tok); get_token;
//!   if cur_tok=par_token then if long_state<>long_call then
//!     @<Report a runaway argument and abort@>;
//!   if cur_tok<right_brace_limit then
//!     if cur_tok<left_brace_limit then incr(unbalance)
//!     else  begin decr(unbalance);
//!       if unbalance=0 then goto done1;
//!       end;
//!   end;
//! done1: rbrace_ptr:=p; store_new_token(cur_tok);
//! end
//!
//! @ If the parameter consists of a single group enclosed in braces, we must
//! strip off the enclosing braces. That's why |rbrace_ptr| was introduced.
//!
//! @<Tidy up the parameter just scanned, and tuck it away@>=
//! begin if (m=1)and(info(p)<right_brace_limit)and(p<>temp_head) then
//!   begin link(rbrace_ptr):=null; free_avail(p);
//!   p:=link(temp_head); pstack[n]:=link(p); free_avail(p);
//!   end
//! else pstack[n]:=link(temp_head);
//! incr(n);
//! if tracing_macros>0 then
//!   begin begin_diagnostic; print_nl(match_chr); print_int(n);
//!   print("<-"); show_token_list(pstack[n-1],null,1000);
//!   end_diagnostic(false);
//!   end;
//! end
//!
//! @ @<Show the text of the macro being expanded@>=
//! begin begin_diagnostic; print_ln; print_cs(warning_index);
//! token_show(ref_count); end_diagnostic(false);
//! end
//!
