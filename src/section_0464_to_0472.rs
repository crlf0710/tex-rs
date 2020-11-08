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
