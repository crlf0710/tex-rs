//! @* \[53] Extensions.
//! The program above includes a bunch of ``hooks'' that allow further
//! capabilities to be added without upsetting \TeX's basic structure.
//! Most of these hooks are concerned with ``whatsit'' nodes, which are
//! intended to be used for special purposes; whenever a new extension to
//! \TeX\ involves a new kind of whatsit node, a corresponding change needs
//! to be made to the routines below that deal with such nodes,
//! but it will usually be unnecessary to make many changes to the
//! other parts of this program.
//!
//! In order to demonstrate how extensions can be made, we shall treat
//! `\.{\\write}', `\.{\\openout}', `\.{\\closeout}', `\.{\\immediate}',
//! `\.{\\special}', and `\.{\\setlanguage}' as if they were extensions.
//! These commands are actually primitives of \TeX, and they should
//! appear in all implementations of the system; but let's try to imagine
//! that they aren't. Then the program below illustrates how a person
//! could add them.
//!
//! Sometimes, of course, an extension will require changes to \TeX\ itself;
//! no system of hooks could be complete enough for all conceivable extensions.
//! The features associated with `\.{\\write}' are almost all confined to the
//! following paragraphs, but there are small parts of the |print_ln| and
//! |print_char| procedures that were introduced specifically to \.{\\write}
//! characters. Furthermore one of the token lists recognized by the scanner
//! is a |write_text|; and there are a few other miscellaneous places where we
//! have already provided for some aspect of \.{\\write}.  The goal of a \TeX\
//! extender should be to minimize alterations to the standard parts of the
//! program, and to avoid them completely if possible. He or she should also
//! be quite sure that there's no easy way to accomplish the desired goals
//! with the standard features that \TeX\ already has. ``Think thrice before
//! extending,'' because that may save a lot of work, and it will also keep
//! incompatible extensions of \TeX\ from proliferating.
//! @^system dependencies@>
//! @^extensions to \TeX@>
//!
//! @ First let's consider the format of whatsit nodes that are used to represent
//! the data associated with \.{\\write} and its relatives. Recall that a whatsit
//! has |type=whatsit_node|, and the |subtype| is supposed to distinguish
//! different kinds of whatsits. Each node occupies two or more words; the
//! exact number is immaterial, as long as it is readily determined from the
//! |subtype| or other data.
//!
//! We shall introduce five |subtype| values here, corresponding to the
//! control sequences \.{\\openout}, \.{\\write}, \.{\\closeout}, \.{\\special}, and
//! \.{\\setlanguage}. The second word of I/O whatsits has a |write_stream| field
//! that identifies the write-stream number (0 to 15, or 16 for out-of-range and
//! positive, or 17 for out-of-range and negative).
//! In the case of \.{\\write} and \.{\\special}, there is also a field that
//! points to the reference count of a token list that should be sent. In the
//! case of \.{\\openout}, we need three words and three auxiliary subfields
//! to hold the string numbers for name, area, and extension.
//!
//! @d write_node_size=2 {number of words in a write/whatsit node}
//! @d open_node_size=3 {number of words in an open/whatsit node}
//! @d open_node=0 {|subtype| in whatsits that represent files to \.{\\openout}}
//! @d write_node=1 {|subtype| in whatsits that represent things to \.{\\write}}
//! @d close_node=2 {|subtype| in whatsits that represent streams to \.{\\closeout}}
//! @d special_node=3 {|subtype| in whatsits that represent \.{\\special} things}
//! @d language_node=4 {|subtype| in whatsits that change the current language}
//! @d what_lang(#)==link(#+1) {language number, in the range |0..255|}
//! @d what_lhm(#)==type(#+1) {minimum left fragment, in the range |1..63|}
//! @d what_rhm(#)==subtype(#+1) {minimum right fragment, in the range |1..63|}
//! @d write_tokens(#) == link(#+1) {reference count of token list to write}
//! @d write_stream(#) == info(#+1) {stream number (0 to 17)}
//! @d open_name(#) == link(#+1) {string number of file name to open}
//! @d open_area(#) == info(#+2) {string number of file area for |open_name|}
//! @d open_ext(#) == link(#+2) {string number of file extension for |open_name|}
//!
//! @ The sixteen possible \.{\\write} streams are represented by the |write_file|
//! array. The |j|th file is open if and only if |write_open[j]=true|. The last
//! two streams are special; |write_open[16]| represents a stream number
//! greater than 15, while |write_open[17]| represents a negative stream number,
//! and both of these variables are always |false|.
//!
//! @<Glob...@>=
//! @!write_file:array[0..15] of alpha_file;
//! @!write_open:array[0..17] of boolean;
//!
//! @ @<Set init...@>=
//! for k:=0 to 17 do write_open[k]:=false;
//!
//! @ Extensions might introduce new command codes; but it's best to use
//! |extension| with a modifier, whenever possible, so that |main_control|
//! stays the same.
//!
//! @d immediate_code=4 {command modifier for \.{\\immediate}}
//! @d set_language_code=5 {command modifier for \.{\\setlanguage}}
//!
//! @<Put each...@>=
//! primitive("openout",extension,open_node);@/
//! @!@:open_out_}{\.{\\openout} primitive@>
//! primitive("write",extension,write_node); write_loc:=cur_val;@/
//! @!@:write_}{\.{\\write} primitive@>
//! primitive("closeout",extension,close_node);@/
//! @!@:close_out_}{\.{\\closeout} primitive@>
//! primitive("special",extension,special_node);@/
//! @!@:special_}{\.{\\special} primitive@>
//! primitive("immediate",extension,immediate_code);@/
//! @!@:immediate_}{\.{\\immediate} primitive@>
//! primitive("setlanguage",extension,set_language_code);@/
//! @!@:set_language_}{\.{\\setlanguage} primitive@>
//!
//! @ The variable |write_loc| just introduced is used to provide an
//! appropriate error message in case of ``runaway'' write texts.
//!
//! @<Glob...@>=
//! @!write_loc:pointer; {|eqtb| address of \.{\\write}}
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! extension: case chr_code of
//!   open_node:print_esc("openout");
//!   write_node:print_esc("write");
//!   close_node:print_esc("closeout");
//!   special_node:print_esc("special");
//!   immediate_code:print_esc("immediate");
//!   set_language_code:print_esc("setlanguage");
//!   othercases print("[unknown extension!]")
//!   endcases;
//!
//! @ When an |extension| command occurs in |main_control|, in any mode,
//! the |do_extension| routine is called.
//!
//! @<Cases of |main_control| that are for extensions...@>=
//! any_mode(extension):do_extension;
//!
//! @ @<Declare act...@>=
//! @t\4@>@<Declare procedures needed in |do_extension|@>@;
//! procedure do_extension;
//! var i,@!j,@!k:integer; {all-purpose integers}
//! @!p,@!q,@!r:pointer; {all-purpose pointers}
//! begin case cur_chr of
//! open_node:@<Implement \.{\\openout}@>;
//! write_node:@<Implement \.{\\write}@>;
//! close_node:@<Implement \.{\\closeout}@>;
//! special_node:@<Implement \.{\\special}@>;
//! immediate_code:@<Implement \.{\\immediate}@>;
//! set_language_code:@<Implement \.{\\setlanguage}@>;
//! othercases confusion("ext1")
//! @:this can't happen ext1}{\quad ext1@>
//! endcases;
//! end;
//!
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
//! @ The presence of `\.{\\immediate}' causes the |do_extension| procedure
//! to descend to one level of recursion. Nothing happens unless \.{\\immediate}
//! is followed by `\.{\\openout}', `\.{\\write}', or `\.{\\closeout}'.
//! @^recursion@>
//!
//! @<Implement \.{\\immediate}@>=
//! begin get_x_token;
//! if (cur_cmd=extension)and(cur_chr<=close_node) then
//!   begin p:=tail; do_extension; {append a whatsit node}
//!   out_what(tail); {do the action immediately}
//!   flush_node_list(tail); tail:=p; link(p):=null;
//!   end
//! else back_input;
//! end
//!
//! @ The \.{\\language} extension is somewhat different.
//! We need a subroutine that comes into play when a character of
//! a non-|clang| language is being appended to the current paragraph.
//!
//! @<Declare action...@>=
//! procedure fix_language;
//! var @!l:ASCII_code; {the new current language}
//! begin if language<=0 then l:=0
//! else if language>255 then l:=0
//! else l:=language;
//! if l<>clang then
//!   begin new_whatsit(language_node,small_node_size);
//!   what_lang(tail):=l; clang:=l;@/
//!   what_lhm(tail):=norm_min(left_hyphen_min);
//!   what_rhm(tail):=norm_min(right_hyphen_min);
//!   end;
//! end;
//!
//! @ @<Implement \.{\\setlanguage}@>=
//! if abs(mode)<>hmode then report_illegal_case
//! else begin new_whatsit(language_node,small_node_size);
//!   scan_int;
//!   if cur_val<=0 then clang:=0
//!   else if cur_val>255 then clang:=0
//!   else clang:=cur_val;
//!   what_lang(tail):=clang;
//!   what_lhm(tail):=norm_min(left_hyphen_min);
//!   what_rhm(tail):=norm_min(right_hyphen_min);
//!   end
