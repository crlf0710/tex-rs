//! @ The following 48-way switch accomplishes the scanning quickly, assuming
//! that a decent \PASCAL\ compiler has translated the code. Note that the numeric
//! values for |mid_line|, |skip_blanks|, and |new_line| are spaced
//! apart from each other by |max_char_code+1|, so we can add a character's
//! command code to the state to get a single number that characterizes both.
//!
//! @d any_state_plus(#) == mid_line+#,skip_blanks+#,new_line+#
//!
//! @<Change state if necessary...@>=
//! case state+cur_cmd of
//! @<Cases where character is ignored@>: goto switch;
//! any_state_plus(escape): @<Scan a control sequence
//!   and set |state:=skip_blanks| or |mid_line|@>;
//! any_state_plus(active_char): @<Process an active-character control sequence
//!   and set |state:=mid_line|@>;
//! any_state_plus(sup_mark): @<If this |sup_mark| starts an expanded character
//!   like~\.{\^\^A} or~\.{\^\^df}, then |goto reswitch|,
//!   otherwise set |state:=mid_line|@>;
//! any_state_plus(invalid_char): @<Decry the invalid character and
//!   |goto restart|@>;
//! @t\4@>@<Handle situations involving spaces, braces, changes of state@>@;
//! othercases do_nothing
//! endcases
//!
//! @ @<Cases where character is ignored@>=
//! any_state_plus(ignore),skip_blanks+spacer,new_line+spacer
//!
//! @ We go to |restart| instead of to |switch|, because |state| might equal
//! |token_list| after the error has been dealt with
//! (cf.\ |clear_for_error_prompt|).
//!
//! @<Decry the invalid...@>=
//! begin print_err("Text line contains an invalid character");
//! @.Text line contains...@>
//! help2("A funny symbol that I can't read has just been input.")@/
//! ("Continue, and I'll forget that it ever happened.");@/
//! deletions_allowed:=false; error; deletions_allowed:=true;
//! goto restart;
//! end
//!
//! @ @d add_delims_to(#)==#+math_shift,#+tab_mark,#+mac_param,
//!   #+sub_mark,#+letter,#+other_char
//!
//! @<Handle situations involving spaces, braces, changes of state@>=
//! mid_line+spacer:@<Enter |skip_blanks| state, emit a space@>;
//! mid_line+car_ret:@<Finish line, emit a space@>;
//! skip_blanks+car_ret,any_state_plus(comment):
//!   @<Finish line, |goto switch|@>;
//! new_line+car_ret:@<Finish line, emit a \.{\\par}@>;
//! mid_line+left_brace: incr(align_state);
//! skip_blanks+left_brace,new_line+left_brace: begin
//!   state:=mid_line; incr(align_state);
//!   end;
//! mid_line+right_brace: decr(align_state);
//! skip_blanks+right_brace,new_line+right_brace: begin
//!   state:=mid_line; decr(align_state);
//!   end;
//! add_delims_to(skip_blanks),add_delims_to(new_line): state:=mid_line;
//!
//! @ When a character of type |spacer| gets through, its character code is
//! changed to $\.{"\ "}=@'40$. This means that the ASCII codes for tab and space,
//! and for the space inserted at the end of a line, will
//! be treated alike when macro parameters are being matched. We do this
//! since such characters are indistinguishable on most computer terminal displays.
//!
//! @<Finish line, emit a space@>=
//! begin loc:=limit+1; cur_cmd:=spacer; cur_chr:=" ";
//! end
//!
//! @ The following code is performed only when |cur_cmd=spacer|.
//!
//! @<Enter |skip_blanks| state, emit a space@>=
//! begin state:=skip_blanks; cur_chr:=" ";
//! end
//!
//! @ @<Finish line, |goto switch|@>=
//! begin loc:=limit+1; goto switch;
//! end
//!
//! @ @<Finish line, emit a \.{\\par}@>=
//! begin loc:=limit+1; cur_cs:=par_loc; cur_cmd:=eq_type(cur_cs);
//! cur_chr:=equiv(cur_cs);
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
//! @ Notice that a code like \.{\^\^8} becomes \.x if not followed by a hex digit.
//!
//! @d is_hex(#)==(((#>="0")and(#<="9"))or((#>="a")and(#<="f")))
//! @d hex_to_cur_chr==
//!   if c<="9" then cur_chr:=c-"0" @+else cur_chr:=c-"a"+10;
//!   if cc<="9" then cur_chr:=16*cur_chr+cc-"0"
//!   else cur_chr:=16*cur_chr+cc-"a"+10
//!
//! @<If this |sup_mark| starts an expanded character...@>=
//! begin if cur_chr=buffer[loc] then if loc<limit then
//!   begin c:=buffer[loc+1]; @+if c<@'200 then {yes we have an expanded char}
//!     begin loc:=loc+2;
//!     if is_hex(c) then if loc<=limit then
//!       begin cc:=buffer[loc]; @+if is_hex(cc) then
//!         begin incr(loc); hex_to_cur_chr; goto reswitch;
//!         end;
//!       end;
//!     if c<@'100 then cur_chr:=c+@'100 @+else cur_chr:=c-@'100;
//!     goto reswitch;
//!     end;
//!   end;
//! state:=mid_line;
//! end
//!
//! @ @<Process an active-character...@>=
//! begin cur_cs:=cur_chr+active_base;
//! cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs); state:=mid_line;
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
//! @ Control sequence names are scanned only when they appear in some line of
//! a file; once they have been scanned the first time, their |eqtb| location
//! serves as a unique identification, so \TeX\ doesn't need to refer to the
//! original name any more except when it prints the equivalent in symbolic form.
//!
//! The program that scans a control sequence has been written carefully
//! in order to avoid the blowups that might otherwise occur if a malicious
//! user tried something like `\.{\\catcode\'15=0}'. The algorithm might
//! look at |buffer[limit+1]|, but it never looks at |buffer[limit+2]|.
//!
//! If expanded characters like `\.{\^\^A}' or `\.{\^\^df}'
//! appear in or just following
//! a control sequence name, they are converted to single characters in the
//! buffer and the process is repeated, slowly but surely.
//!
//! @<Scan a control...@>=
//! begin if loc>limit then cur_cs:=null_cs {|state| is irrelevant in this case}
//! else  begin start_cs: k:=loc; cur_chr:=buffer[k]; cat:=cat_code(cur_chr);
//!   incr(k);
//!   if cat=letter then state:=skip_blanks
//!   else if cat=spacer then state:=skip_blanks
//!   else state:=mid_line;
//!   if (cat=letter)and(k<=limit) then
//!     @<Scan ahead in the buffer until finding a nonletter;
//!     if an expanded code is encountered, reduce it
//!     and |goto start_cs|; otherwise if a multiletter control
//!     sequence is found, adjust |cur_cs| and |loc|, and
//!     |goto found|@>
//!   else @<If an expanded code is present, reduce it and |goto start_cs|@>;
//!   cur_cs:=single_base+buffer[loc]; incr(loc);
//!   end;
//! found: cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
//! @ Whenever we reach the following piece of code, we will have
//! |cur_chr=buffer[k-1]| and |k<=limit+1| and |cat=cat_code(cur_chr)|. If an
//! expanded code like \.{\^\^A} or \.{\^\^df} appears in |buffer[(k-1)..(k+1)]|
//! or |buffer[(k-1)..(k+2)]|, we
//! will store the corresponding code in |buffer[k-1]| and shift the rest of
//! the buffer left two or three places.
//!
//! @<If an expanded...@>=
//! begin if buffer[k]=cur_chr then @+if cat=sup_mark then @+if k<limit then
//!   begin c:=buffer[k+1]; @+if c<@'200 then {yes, one is indeed present}
//!     begin d:=2;
//!     if is_hex(c) then @+if k+2<=limit then
//!       begin cc:=buffer[k+2]; @+if is_hex(cc) then incr(d);
//!       end;
//!     if d>2 then
//!       begin hex_to_cur_chr; buffer[k-1]:=cur_chr;
//!       end
//!     else if c<@'100 then buffer[k-1]:=c+@'100
//!     else buffer[k-1]:=c-@'100;
//!     limit:=limit-d; first:=first-d;
//!     while k<=limit do
//!       begin buffer[k]:=buffer[k+d]; incr(k);
//!       end;
//!     goto start_cs;
//!     end;
//!   end;
//! end
//!
//! @ @<Scan ahead in the buffer...@>=
//! begin repeat cur_chr:=buffer[k]; cat:=cat_code(cur_chr); incr(k);
//! until (cat<>letter)or(k>limit);
//! @<If an expanded...@>;
//! if cat<>letter then decr(k);
//!   {now |k| points to first nonletter}
//! if k>loc+1 then {multiletter control sequence has been scanned}
//!   begin cur_cs:=id_lookup(loc,k-loc); loc:=k; goto found;
//!   end;
//! end
//!