//! @ Here is a subroutine that creates a whatsit node having a given |subtype|
//! and a given number of words. It initializes only the first word of the whatsit,
//! and appends it to the current list.
//!
//! @<Declare procedures needed in |do_extension|@>=
//! procedure new_whatsit(@!s:small_number;@!w:small_number);
//! var p:pointer; {the new node}
//! begin p:=get_node(w); type(p):=whatsit_node; subtype(p):=s;
//! link(tail):=p; tail:=p;
//! end;
//!
//! @ The next subroutine uses |cur_chr| to decide what sort of whatsit is
//! involved, and also inserts a |write_stream| number.
//!
//! @<Declare procedures needed in |do_ext...@>=
//! procedure new_write_whatsit(@!w:small_number);
//! begin new_whatsit(cur_chr,w);
//! if w<>write_node_size then scan_four_bit_int
//! else  begin scan_int;
//!   if cur_val<0 then cur_val:=17
//!   else if cur_val>15 then cur_val:=16;
//!   end;
//! write_stream(tail):=cur_val;
//! end;
//!
//! @ @<Implement \.{\\openout}@>=
//! begin new_write_whatsit(open_node_size);
//! scan_optional_equals; scan_file_name;@/
//! open_name(tail):=cur_name; open_area(tail):=cur_area; open_ext(tail):=cur_ext;
//! end
//!
//! @ When `\.{\\write 12\{...\}}' appears, we scan the token list `\.{\{...\}}'
//! without expanding its macros; the macros will be expanded later when this
//! token list is rescanned.
//!
//! @<Implement \.{\\write}@>=
//! begin k:=cur_cs; new_write_whatsit(write_node_size);@/
//! cur_cs:=k; p:=scan_toks(false,false); write_tokens(tail):=def_ref;
//! end
//!
//! @ @<Implement \.{\\closeout}@>=
//! begin new_write_whatsit(write_node_size); write_tokens(tail):=null;
//! end
//!
//! @ When `\.{\\special\{...\}}' appears, we expand the macros in the token
//! list as in \.{\\xdef} and \.{\\mark}.
//!
//! @<Implement \.{\\special}@>=
//! begin new_whatsit(special_node,write_node_size); write_stream(tail):=null;
//! p:=scan_toks(false,true); write_tokens(tail):=def_ref;
//! end
//!
//! @ Each new type of node that appears in our data structure must be capable
//! of being displayed, copied, destroyed, and so on. The routines that we
//! need for write-oriented whatsits are somewhat like those for mark nodes;
//! other extensions might, of course, involve more subtlety here.
//!
//! @<Basic printing...@>=
//! procedure print_write_whatsit(@!s:str_number;@!p:pointer);
//! begin print_esc(s);
//! if write_stream(p)<16 then print_int(write_stream(p))
//! else if write_stream(p)=16 then print_char("*")
//! @.*\relax@>
//! else print_char("-");
//! end;
//!
//! @ @<Display the whatsit...@>=
//! case subtype(p) of
//! open_node:begin print_write_whatsit("openout",p);
//!   print_char("="); print_file_name(open_name(p),open_area(p),open_ext(p));
//!   end;
//! write_node:begin print_write_whatsit("write",p);
//!   print_mark(write_tokens(p));
//!   end;
//! close_node:print_write_whatsit("closeout",p);
//! special_node:begin print_esc("special");
//!   print_mark(write_tokens(p));
//!   end;
//! language_node:begin print_esc("setlanguage");
//!   print_int(what_lang(p)); print(" (hyphenmin ");
//!   print_int(what_lhm(p)); print_char(",");
//!   print_int(what_rhm(p)); print_char(")");
//!   end;
//! othercases print("whatsit?")
//! endcases
//!
//! @ @<Make a partial copy of the whatsit...@>=
//! case subtype(p) of
//! open_node: begin r:=get_node(open_node_size); words:=open_node_size;
//!   end;
//! write_node,special_node: begin r:=get_node(write_node_size);
//!   add_token_ref(write_tokens(p)); words:=write_node_size;
//!   end;
//! close_node,language_node: begin r:=get_node(small_node_size);
//!   words:=small_node_size;
//!   end;
//! othercases confusion("ext2")
//! @:this can't happen ext2}{\quad ext2@>
//! endcases
//!
//! @ @<Wipe out the whatsit...@>=
//! begin case subtype(p) of
//! open_node: free_node(p,open_node_size);
//! write_node,special_node: begin delete_token_ref(write_tokens(p));
//!   free_node(p,write_node_size); goto done;
//!   end;
//! close_node,language_node: free_node(p,small_node_size);
//! othercases confusion("ext3")
//! @:this can't happen ext3}{\quad ext3@>
//! endcases;@/
//! goto done;
//! end
//!
//! @ @<Incorporate a whatsit node into a vbox@>=do_nothing
//!
//! @ @<Incorporate a whatsit node into an hbox@>=do_nothing
//!
//! @ @<Let |d| be the width of the whatsit |p|@>=d:=0
//!
//! @ @d adv_past(#)==@+if subtype(#)=language_node then
//!     begin cur_lang:=what_lang(#); l_hyf:=what_lhm(#); r_hyf:=what_rhm(#);@+end
//!
//! @<Advance \(p)past a whatsit node in the \(l)|line_break| loop@>=@+
//! adv_past(cur_p)
//!
//! @ @<Advance \(p)past a whatsit node in the \(p)pre-hyphenation loop@>=@+
//! adv_past(s)
//!
//! @ @<Prepare to move whatsit |p| to the current page, then |goto contribute|@>=
//! goto contribute
//!
//! @ @<Process whatsit |p| in |vert_break| loop, |goto not_found|@>=
//! goto not_found
//!
//! @ @<Output the whatsit node |p| in a vlist@>=
//! out_what(p)
//!
//! @ @<Output the whatsit node |p| in an hlist@>=
//! out_what(p)
//!
//! @ After all this preliminary shuffling, we come finally to the routines
//! that actually send out the requested data. Let's do \.{\\special} first
//! (it's easier).
//!
//! @<Declare procedures needed in |hlist_out|, |vlist_out|@>=
//! procedure special_out(@!p:pointer);
//! var old_setting:0..max_selector; {holds print |selector|}
//! @!k:pool_pointer; {index into |str_pool|}
//! begin synch_h; synch_v;@/
//! old_setting:=selector; selector:=new_string;
//! show_token_list(link(write_tokens(p)),null,pool_size-pool_ptr);
//! selector:=old_setting;
//! str_room(1);
//! if cur_length<256 then
//!   begin dvi_out(xxx1); dvi_out(cur_length);
//!   end
//! else  begin dvi_out(xxx4); dvi_four(cur_length);
//!   end;
//! for k:=str_start[str_ptr] to pool_ptr-1 do dvi_out(so(str_pool[k]));
//! pool_ptr:=str_start[str_ptr]; {erase the string}
//! end;
//!
//! @ To write a token list, we must run it through \TeX's scanner, expanding
//! macros and \.{\\the} and \.{\\number}, etc. This might cause runaways,
//! if a delimited macro parameter isn't matched, and runaways would be
//! extremely confusing since we are calling on \TeX's scanner in the middle
//! of a \.{\\shipout} command. Therefore we will put a dummy control sequence as
//! a ``stopper,'' right after the token list. This control sequence is
//! artificially defined to be \.{\\outer}.
//! @:end_write_}{\.{\\endwrite}@>
//!
//! @<Initialize table...@>=
//! text(end_write):="endwrite"; eq_level(end_write):=level_one;
//! eq_type(end_write):=outer_call; equiv(end_write):=null;
//!
//! @ @<Declare procedures needed in |hlist_out|, |vlist_out|@>=
//! procedure write_out(@!p:pointer);
//! var old_setting:0..max_selector; {holds print |selector|}
//! @!old_mode:integer; {saved |mode|}
//! @!j:small_number; {write stream number}
//! @!q,@!r:pointer; {temporary variables for list manipulation}
//! begin @<Expand macros in the token list
//!   and make |link(def_ref)| point to the result@>;
//! old_setting:=selector; j:=write_stream(p);
//! if write_open[j] then selector:=j
//! else  begin {write to the terminal if file isn't open}
//!   if (j=17)and(selector=term_and_log) then selector:=log_only;
//!   print_nl("");
//!   end;
//! token_show(def_ref); print_ln;
//! flush_list(def_ref); selector:=old_setting;
//! end;
//!
//! @ The final line of this routine is slightly subtle; at least, the author
//! didn't think about it until getting burnt! There is a used-up token list
//! @^Knuth, Donald Ervin@>
//! on the stack, namely the one that contained |end_write_token|. (We
//! insert this artificial `\.{\\endwrite}' to prevent runaways, as explained
//! above.) If it were not removed, and if there were numerous writes on a
//! single page, the stack would overflow.
//!
//! @d end_write_token==cs_token_flag+end_write
//!
//! @<Expand macros in the token list and...@>=
//! q:=get_avail; info(q):=right_brace_token+"}";@/
//! r:=get_avail; link(q):=r; info(r):=end_write_token; ins_list(q);@/
//! begin_token_list(write_tokens(p),write_text);@/
//! q:=get_avail; info(q):=left_brace_token+"{"; ins_list(q);
//! {now we're ready to scan
//!   `\.\{$\langle\,$token list$\,\rangle$\.{\} \\endwrite}'}
//! old_mode:=mode; mode:=0;
//!   {disable \.{\\prevdepth}, \.{\\spacefactor}, \.{\\lastskip}, \.{\\prevgraf}}
//! cur_cs:=write_loc; q:=scan_toks(false,true); {expand macros, etc.}
//! get_token;@+if cur_tok<>end_write_token then
//!   @<Recover from an unbalanced write command@>;
//! mode:=old_mode;
//! end_token_list {conserve stack space}
//!
//! @ @<Recover from an unbalanced write command@>=
//! begin print_err("Unbalanced write command");
//! @.Unbalanced write...@>
//! help2("On this page there's a \write with fewer real {'s than }'s.")@/
//! ("I can't handle that very well; good luck."); error;
//! repeat get_token;
//! until cur_tok=end_write_token;
//! end
//!
//! @ The |out_what| procedure takes care of outputting whatsit nodes for
//! |vlist_out| and |hlist_out|\kern-.3pt.
//!
//! @<Declare procedures needed in |hlist_out|, |vlist_out|@>=
//! procedure out_what(@!p:pointer);
//! var j:small_number; {write stream number}
//! begin case subtype(p) of
//! open_node,write_node,close_node:@<Do some work that has been queued up
//!   for \.{\\write}@>;
//! special_node:special_out(p);
//! language_node:do_nothing;
//! othercases confusion("ext4")
//! @:this can't happen ext4}{\quad ext4@>
//! endcases;
//! end;
//!
//! @ We don't implement \.{\\write} inside of leaders. (The reason is that
//! the number of times a leader box appears might be different in different
//! implementations, due to machine-dependent rounding in the glue calculations.)
//! @^leaders@>
//!
//! @<Do some work that has been queued up...@>=
//! if not doing_leaders then
//!   begin j:=write_stream(p);
//!   if subtype(p)=write_node then write_out(p)
//!   else  begin if write_open[j] then a_close(write_file[j]);
//!     if subtype(p)=close_node then write_open[j]:=false
//!     else if j<16 then
//!       begin cur_name:=open_name(p); cur_area:=open_area(p);
//!       cur_ext:=open_ext(p);
//!       if cur_ext="" then cur_ext:=".tex";
//!       pack_cur_name;
//!       while not a_open_out(write_file[j]) do
//!         prompt_file_name("output file name",".tex");
//!       write_open[j]:=true;
//!       end;
//!     end;
//!   end
//!
