//! @* \[27] Building token lists.
//! The token lists for macros and for other things like \.{\\mark} and \.{\\output}
//! and \.{\\write} are produced by a procedure called |scan_toks|.
//!
//! Before we get into the details of |scan_toks|, let's consider a much
//! simpler task, that of converting the current string into a token list.
//! The |str_toks| function does this; it classifies spaces as type |spacer|
//! and everything else as type |other_char|.
//!
//! The token list created by |str_toks| begins at |link(temp_head)| and ends
//! at the value |p| that is returned. (If |p=temp_head|, the list is empty.)
//!
//! @p function str_toks(@!b:pool_pointer):pointer;
//!   {changes the string |str_pool[b..pool_ptr]| to a token list}
//! var p:pointer; {tail of the token list}
//! @!q:pointer; {new node being added to the token list via |store_new_token|}
//! @!t:halfword; {token being appended}
//! @!k:pool_pointer; {index into |str_pool|}
//! begin str_room(1);
//! p:=temp_head; link(p):=null; k:=b;
//! while k<pool_ptr do
//!   begin t:=so(str_pool[k]);
//!   if t=" " then t:=space_token
//!   else t:=other_token+t;
//!   fast_store_new_token(t);
//!   incr(k);
//!   end;
//! pool_ptr:=b; str_toks:=p;
//! end;
//!
//! @ The main reason for wanting |str_toks| is the next function,
//! |the_toks|, which has similar input/output characteristics.
//!
//! This procedure is supposed to scan something like `\.{\\skip\\count12}',
//! i.e., whatever can follow `\.{\\the}', and it constructs a token list
//! containing something like `\.{-3.0pt minus 0.5fill}'.
//!
//! @p function the_toks:pointer;
//! var old_setting:0..max_selector; {holds |selector| setting}
//! @!p,@!q,@!r:pointer; {used for copying a token list}
//! @!b:pool_pointer; {base of temporary string}
//! begin get_x_token; scan_something_internal(tok_val,false);
//! if cur_val_level>=ident_val then @<Copy the token list@>
//! else begin old_setting:=selector; selector:=new_string; b:=pool_ptr;
//!   case cur_val_level of
//!   int_val:print_int(cur_val);
//!   dimen_val:begin print_scaled(cur_val); print("pt");
//!     end;
//!   glue_val: begin print_spec(cur_val,"pt"); delete_glue_ref(cur_val);
//!     end;
//!   mu_val: begin print_spec(cur_val,"mu"); delete_glue_ref(cur_val);
//!     end;
//!   end; {there are no other cases}
//!   selector:=old_setting; the_toks:=str_toks(b);
//!   end;
//! end;
//!
//! @ @<Copy the token list@>=
//! begin p:=temp_head; link(p):=null;
//! if cur_val_level=ident_val then store_new_token(cs_token_flag+cur_val)
//! else if cur_val<>null then
//!   begin r:=link(cur_val); {do not copy the reference count}
//!   while r<>null do
//!     begin fast_store_new_token(info(r)); r:=link(r);
//!     end;
//!   end;
//! the_toks:=p;
//! end
//!
//! @ Here's part of the |expand| subroutine that we are now ready to complete:
//!
//! @p procedure ins_the_toks;
//! begin link(garbage):=the_toks; ins_list(link(temp_head));
//! end;
//!
//! @ The primitives \.{\\number}, \.{\\romannumeral}, \.{\\string}, \.{\\meaning},
//! \.{\\fontname}, and \.{\\jobname} are defined as follows.
//!
//! @d number_code=0 {command code for \.{\\number}}
//! @d roman_numeral_code=1 {command code for \.{\\romannumeral}}
//! @d string_code=2 {command code for \.{\\string}}
//! @d meaning_code=3 {command code for \.{\\meaning}}
//! @d font_name_code=4 {command code for \.{\\fontname}}
//! @d job_name_code=5 {command code for \.{\\jobname}}
//!
//! @<Put each...@>=
//! primitive("number",convert,number_code);@/
//! @!@:number_}{\.{\\number} primitive@>
//! primitive("romannumeral",convert,roman_numeral_code);@/
//! @!@:roman_numeral_}{\.{\\romannumeral} primitive@>
//! primitive("string",convert,string_code);@/
//! @!@:string_}{\.{\\string} primitive@>
//! primitive("meaning",convert,meaning_code);@/
//! @!@:meaning_}{\.{\\meaning} primitive@>
//! primitive("fontname",convert,font_name_code);@/
//! @!@:font_name_}{\.{\\fontname} primitive@>
//! primitive("jobname",convert,job_name_code);@/
//! @!@:job_name_}{\.{\\jobname} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! convert: case chr_code of
//!   number_code: print_esc("number");
//!   roman_numeral_code: print_esc("romannumeral");
//!   string_code: print_esc("string");
//!   meaning_code: print_esc("meaning");
//!   font_name_code: print_esc("fontname");
//!   othercases print_esc("jobname")
//!   endcases;
//!
//! @ The procedure |conv_toks| uses |str_toks| to insert the token list
//! for |convert| functions into the scanner; `\.{\\outer}' control sequences
//! are allowed to follow `\.{\\string}' and `\.{\\meaning}'.
//!
//! @p procedure conv_toks;
//! var old_setting:0..max_selector; {holds |selector| setting}
//! @!c:number_code..job_name_code; {desired type of conversion}
//! @!save_scanner_status:small_number; {|scanner_status| upon entry}
//! @!b:pool_pointer; {base of temporary string}
//! begin c:=cur_chr; @<Scan the argument for command |c|@>;
//! old_setting:=selector; selector:=new_string; b:=pool_ptr;
//! @<Print the result of command |c|@>;
//! selector:=old_setting; link(garbage):=str_toks(b); ins_list(link(temp_head));
//! end;
//!
//! @ @<Scan the argument for command |c|@>=
//! case c of
//! number_code,roman_numeral_code: scan_int;
//! string_code, meaning_code: begin save_scanner_status:=scanner_status;
//!   scanner_status:=normal; get_token; scanner_status:=save_scanner_status;
//!   end;
//! font_name_code: scan_font_ident;
//! job_name_code: if job_name=0 then open_log_file;
//! end {there are no other cases}
//!
//! @ @<Print the result of command |c|@>=
//! case c of
//! number_code: print_int(cur_val);
//! roman_numeral_code: print_roman_int(cur_val);
//! string_code:if cur_cs<>0 then sprint_cs(cur_cs)
//!   else print_char(cur_chr);
//! meaning_code: print_meaning;
//! font_name_code: begin print(font_name[cur_val]);
//!   if font_size[cur_val]<>font_dsize[cur_val] then
//!     begin print(" at "); print_scaled(font_size[cur_val]);
//!     print("pt");
//!     end;
//!   end;
//! job_name_code: print(job_name);
//! end {there are no other cases}
//!
//! @ Now we can't postpone the difficulties any longer; we must bravely tackle
//! |scan_toks|. This function returns a pointer to the tail of a new token
//! list, and it also makes |def_ref| point to the reference count at the
//! head of that list.
//!
//! There are two boolean parameters, |macro_def| and |xpand|. If |macro_def|
//! is true, the goal is to create the token list for a macro definition;
//! otherwise the goal is to create the token list for some other \TeX\
//! primitive: \.{\\mark}, \.{\\output}, \.{\\everypar}, \.{\\lowercase},
//! \.{\\uppercase}, \.{\\message}, \.{\\errmessage}, \.{\\write}, or
//! \.{\\special}. In the latter cases a left brace must be scanned next; this
//! left brace will not be part of the token list, nor will the matching right
//! brace that comes at the end. If |xpand| is false, the token list will
//! simply be copied from the input using |get_token|. Otherwise all expandable
//! tokens will be expanded until unexpandable tokens are left, except that
//! the results of expanding `\.{\\the}' are not expanded further.
//! If both |macro_def| and |xpand| are true, the expansion applies
//! only to the macro body (i.e., to the material following the first
//! |left_brace| character).
//!
//! The value of |cur_cs| when |scan_toks| begins should be the |eqtb|
//! address of the control sequence to display in ``runaway'' error
//! messages.
//!
//! @p function scan_toks(@!macro_def,@!xpand:boolean):pointer;
//! label found,done,done1,done2;
//! var t:halfword; {token representing the highest parameter number}
//! @!s:halfword; {saved token}
//! @!p:pointer; {tail of the token list being built}
//! @!q:pointer; {new node being added to the token list via |store_new_token|}
//! @!unbalance:halfword; {number of unmatched left braces}
//! @!hash_brace:halfword; {possible `\.{\#\{}' token}
//! begin if macro_def then scanner_status:=defining
//! @+else scanner_status:=absorbing;
//! warning_index:=cur_cs; def_ref:=get_avail; token_ref_count(def_ref):=null;
//! p:=def_ref; hash_brace:=0; t:=zero_token;
//! if macro_def then @<Scan and build the parameter part of the macro definition@>
//! else scan_left_brace; {remove the compulsory left brace}
//! @<Scan and build the body of the token list; |goto found| when finished@>;
//! found: scanner_status:=normal;
//! if hash_brace<>0 then store_new_token(hash_brace);
//! scan_toks:=p;
//! end;
//!
//! @ @<Scan and build the parameter part...@>=
//! begin loop begin get_token; {set |cur_cmd|, |cur_chr|, |cur_tok|}
//!   if cur_tok<right_brace_limit then goto done1;
//!   if cur_cmd=mac_param then
//!     @<If the next character is a parameter number, make |cur_tok|
//!       a |match| token; but if it is a left brace, store
//!       `|left_brace|, |end_match|', set |hash_brace|, and |goto done|@>;
//!   store_new_token(cur_tok);
//!   end;
//! done1: store_new_token(end_match_token);
//! if cur_cmd=right_brace then
//!   @<Express shock at the missing left brace; |goto found|@>;
//! done: end
//!
//! @ @<Express shock...@>=
//! begin print_err("Missing { inserted"); incr(align_state);
//! @.Missing \{ inserted@>
//! help2("Where was the left brace? You said something like `\def\a}',")@/
//!   ("which I'm going to interpret as `\def\a{}'."); error; goto found;
//! end
//!
//! @ @<If the next character is a parameter number...@>=
//! begin s:=match_token+cur_chr; get_token;
//! if cur_cmd=left_brace then
//!   begin hash_brace:=cur_tok;
//!   store_new_token(cur_tok); store_new_token(end_match_token);
//!   goto done;
//!   end;
//! if t=zero_token+9 then
//!   begin print_err("You already have nine parameters");
//! @.You already have nine...@>
//!   help1("I'm going to ignore the # sign you just used."); error;
//!   end
//! else  begin incr(t);
//!   if cur_tok<>t then
//!     begin print_err("Parameters must be numbered consecutively");
//! @.Parameters...consecutively@>
//!     help2("I've inserted the digit you should have used after the #.")@/
//!       ("Type `1' to delete what you did use."); back_error;
//!     end;
//!   cur_tok:=s;
//!   end;
//! end
//!
//! @ @<Scan and build the body of the token list; |goto found| when finished@>=
//! unbalance:=1;
//! loop@+  begin if xpand then @<Expand the next part of the input@>
//!   else get_token;
//!   if cur_tok<right_brace_limit then
//!     if cur_cmd<right_brace then incr(unbalance)
//!     else  begin decr(unbalance);
//!       if unbalance=0 then goto found;
//!       end
//!   else if cur_cmd=mac_param then
//!     if macro_def then @<Look for parameter number or \.{\#\#}@>;
//!   store_new_token(cur_tok);
//!   end
//!
//! @ Here we insert an entire token list created by |the_toks| without
//! expanding it further.
//!
//! @<Expand the next part of the input@>=
//! begin loop begin get_next;
//!   if cur_cmd<=max_command then goto done2;
//!   if cur_cmd<>the then expand
//!   else  begin q:=the_toks;
//!     if link(temp_head)<>null then
//!       begin link(p):=link(temp_head); p:=q;
//!       end;
//!     end;
//!   end;
//! done2: x_token
//! end
//!
//! @ @<Look for parameter number...@>=
//! begin s:=cur_tok;
//! if xpand then get_x_token else get_token;
//! if cur_cmd<>mac_param then
//!   if (cur_tok<=zero_token)or(cur_tok>t) then
//!     begin print_err("Illegal parameter number in definition of ");
//! @.Illegal parameter number...@>
//!     sprint_cs(warning_index);
//!     help3("You meant to type ## instead of #, right?")@/
//!     ("Or maybe a } was forgotten somewhere earlier, and things")@/
//!     ("are all screwed up? I'm going to assume that you meant ##.");
//!     back_error; cur_tok:=s;
//!     end
//!   else cur_tok:=out_param_token-"0"+cur_chr;
//! end
//!
//! @ Another way to create a token list is via the \.{\\read} command. The
//! sixteen files potentially usable for reading appear in the following
//! global variables. The value of |read_open[n]| will be |closed| if
//! stream number |n| has not been opened or if it has been fully read;
//! |just_open| if an \.{\\openin} but not a \.{\\read} has been done;
//! and |normal| if it is open and ready to read the next line.
//!
//! @d closed=2 {not open, or at end of file}
//! @d just_open=1 {newly opened, first line not yet read}
//!
//! @<Glob...@>=
//! @!read_file:array[0..15] of alpha_file; {used for \.{\\read}}
//! @!read_open:array[0..16] of normal..closed; {state of |read_file[n]|}
//!
//! @ @<Set init...@>=
//! for k:=0 to 16 do read_open[k]:=closed;
//!
//! @ The |read_toks| procedure constructs a token list like that for any
//! macro definition, and makes |cur_val| point to it. Parameter |r| points
//! to the control sequence that will receive this token list.
//!
//! @p procedure read_toks(@!n:integer;@!r:pointer);
//! label done;
//! var p:pointer; {tail of the token list}
//! @!q:pointer; {new node being added to the token list via |store_new_token|}
//! @!s:integer; {saved value of |align_state|}
//! @!m:small_number; {stream number}
//! begin scanner_status:=defining; warning_index:=r;
//! def_ref:=get_avail; token_ref_count(def_ref):=null;
//! p:=def_ref; {the reference count}
//! store_new_token(end_match_token);
//! if (n<0)or(n>15) then m:=16@+else m:=n;
//! s:=align_state; align_state:=1000000; {disable tab marks, etc.}
//! repeat @<Input and store tokens from the next line of the file@>;
//! until align_state=1000000;
//! cur_val:=def_ref; scanner_status:=normal; align_state:=s;
//! end;
//!
//! @ @<Input and store tokens from the next line of the file@>=
//! begin_file_reading; name:=m+1;
//! if read_open[m]=closed then @<Input for \.{\\read} from the terminal@>
//! else if read_open[m]=just_open then @<Input the first line of |read_file[m]|@>
//! else @<Input the next line of |read_file[m]|@>;
//! limit:=last;
//! if end_line_char_inactive then decr(limit)
//! else  buffer[limit]:=end_line_char;
//! first:=limit+1; loc:=start; state:=new_line;@/
//! loop@+  begin get_token;
//!   if cur_tok=0 then goto done;
//!     {|cur_cmd=cur_chr=0| will occur at the end of the line}
//!   if align_state<1000000 then {unmatched `\.\}' aborts the line}
//!     begin repeat get_token; until cur_tok=0;
//!     align_state:=1000000; goto done;
//!     end;
//!   store_new_token(cur_tok);
//!   end;
//! done: end_file_reading
//!
//! @ Here we input on-line into the |buffer| array, prompting the user explicitly
//! if |n>=0|.  The value of |n| is set negative so that additional prompts
//! will not be given in the case of multi-line input.
//!
//! @<Input for \.{\\read} from the terminal@>=
//! if interaction>nonstop_mode then
//!   if n<0 then prompt_input("")
//!   else  begin wake_up_terminal;
//!     print_ln; sprint_cs(r); prompt_input("="); n:=-1;
//!     end
//! else fatal_error("*** (cannot \read from terminal in nonstop modes)")
//! @.cannot \\read@>
//!
//! @ The first line of a file must be treated specially, since |input_ln|
//! must be told not to start with |get|.
//! @^system dependencies@>
//!
//! @<Input the first line of |read_file[m]|@>=
//! if input_ln(read_file[m],false) then read_open[m]:=normal
//! else  begin a_close(read_file[m]); read_open[m]:=closed;
//!   end
//!
//! @ An empty line is appended at the end of a |read_file|.
//! @^empty line at end of file@>
//!
//! @<Input the next line of |read_file[m]|@>=
//! begin if not input_ln(read_file[m],true) then
//!   begin a_close(read_file[m]); read_open[m]:=closed;
//!   if align_state<>1000000 then
//!     begin runaway;
//!     print_err("File ended within "); print_esc("read");
//! @.File ended within \\read@>
//!     help1("This \read has unbalanced braces.");
//!     align_state:=1000000; error;
//!     end;
//!   end;
//! end
//!
