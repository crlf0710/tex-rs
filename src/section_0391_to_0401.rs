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
