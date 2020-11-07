//! @ @<Assignments@>=
//! def_family: begin p:=cur_chr; scan_four_bit_int; p:=p+cur_val;
//!   scan_optional_equals; scan_font_ident; define(p,data,cur_val);
//!   end;
//!
//! @ Next we consider changes to \TeX's numeric registers.
//!
//! @<Assignments@>=
//! register,advance,multiply,divide: do_register_command(a);
//!
//! @ We use the fact that |register<advance<multiply<divide|.
//!
//! @<Declare subprocedures for |prefixed_command|@>=
//! procedure do_register_command(@!a:small_number);
//! label found,exit;
//! var l,@!q,@!r,@!s:pointer; {for list manipulation}
//! @!p:int_val..mu_val; {type of register involved}
//! begin q:=cur_cmd;
//! @<Compute the register location |l| and its type |p|; but |return| if invalid@>;
//! if q=register then scan_optional_equals
//! else if scan_keyword("by") then do_nothing; {optional `\.{by}'}
//! @.by@>
//! arith_error:=false;
//! if q<multiply then @<Compute result of |register| or
//!     |advance|, put it in |cur_val|@>
//! else @<Compute result of |multiply| or |divide|, put it in |cur_val|@>;
//! if arith_error then
//!   begin print_err("Arithmetic overflow");
//! @.Arithmetic overflow@>
//!   help2("I can't carry out that multiplication or division,")@/
//!     ("since the result is out of range.");
//!   if p>=glue_val then delete_glue_ref(cur_val);
//!   error; return;
//!   end;
//! if p<glue_val then word_define(l,cur_val)
//! else  begin trap_zero_glue; define(l,glue_ref,cur_val);
//!   end;
//! exit: end;
//!
//! @ Here we use the fact that the consecutive codes |int_val..mu_val| and
//! |assign_int..assign_mu_glue| correspond to each other nicely.
//!
//! @<Compute the register location |l| and its type |p|...@>=
//! begin if q<>register then
//!   begin get_x_token;
//!   if (cur_cmd>=assign_int)and(cur_cmd<=assign_mu_glue) then
//!     begin l:=cur_chr; p:=cur_cmd-assign_int; goto found;
//!     end;
//!   if cur_cmd<>register then
//!     begin print_err("You can't use `"); print_cmd_chr(cur_cmd,cur_chr);
//! @.You can't use x after ...@>
//!     print("' after "); print_cmd_chr(q,0);
//!     help1("I'm forgetting what you said and not changing anything.");
//!     error; return;
//!     end;
//!   end;
//! p:=cur_chr; scan_eight_bit_int;
//! case p of
//! int_val: l:=cur_val+count_base;
//! dimen_val: l:=cur_val+scaled_base;
//! glue_val: l:=cur_val+skip_base;
//! mu_val: l:=cur_val+mu_skip_base;
//! end; {there are no other cases}
//! end;
//! found:
//!
//! @ @<Compute result of |register| or |advance|...@>=
//! if p<glue_val then
//!   begin if p=int_val then scan_int@+else scan_normal_dimen;
//!   if q=advance then cur_val:=cur_val+eqtb[l].int;
//!   end
//! else  begin scan_glue(p);
//!   if q=advance then @<Compute the sum of two glue specs@>;
//!   end
//!
//! @ @<Compute the sum of two glue specs@>=
//! begin q:=new_spec(cur_val); r:=equiv(l);
//! delete_glue_ref(cur_val);
//! width(q):=width(q)+width(r);
//! if stretch(q)=0 then stretch_order(q):=normal;
//! if stretch_order(q)=stretch_order(r) then stretch(q):=stretch(q)+stretch(r)
//! else if (stretch_order(q)<stretch_order(r))and(stretch(r)<>0) then
//!   begin stretch(q):=stretch(r); stretch_order(q):=stretch_order(r);
//!   end;
//! if shrink(q)=0 then shrink_order(q):=normal;
//! if shrink_order(q)=shrink_order(r) then shrink(q):=shrink(q)+shrink(r)
//! else if (shrink_order(q)<shrink_order(r))and(shrink(r)<>0) then
//!   begin shrink(q):=shrink(r); shrink_order(q):=shrink_order(r);
//!   end;
//! cur_val:=q;
//! end
//!
//! @ @<Compute result of |multiply| or |divide|...@>=
//! begin scan_int;
//! if p<glue_val then
//!   if q=multiply then
//!     if p=int_val then cur_val:=mult_integers(eqtb[l].int,cur_val)
//!     else cur_val:=nx_plus_y(eqtb[l].int,cur_val,0)
//!   else cur_val:=x_over_n(eqtb[l].int,cur_val)
//! else  begin s:=equiv(l); r:=new_spec(s);
//!   if q=multiply then
//!     begin width(r):=nx_plus_y(width(s),cur_val,0);
//!     stretch(r):=nx_plus_y(stretch(s),cur_val,0);
//!     shrink(r):=nx_plus_y(shrink(s),cur_val,0);
//!     end
//!   else  begin width(r):=x_over_n(width(s),cur_val);
//!     stretch(r):=x_over_n(stretch(s),cur_val);
//!     shrink(r):=x_over_n(shrink(s),cur_val);
//!     end;
//!   cur_val:=r;
//!   end;
//! end
//!
//! @ The processing of boxes is somewhat different, because we may need
//! to scan and create an entire box before we actually change the value of the old
//! one.
//!
//! @<Assignments@>=
//! set_box: begin scan_eight_bit_int;
//!   if global then n:=256+cur_val@+else n:=cur_val;
//!   scan_optional_equals;
//!   if set_box_allowed then scan_box(box_flag+n)
//!   else begin print_err("Improper "); print_esc("setbox");
//! @.Improper \\setbox@>
//!     help2("Sorry, \setbox is not allowed after \halign in a display,")@/
//!     ("or between \accent and an accented character."); error;
//!     end;
//!   end;
//!
//! @ The |space_factor| or |prev_depth| settings are changed when a |set_aux|
//! command is sensed. Similarly, |prev_graf| is changed in the presence of
//! |set_prev_graf|, and |dead_cycles| or |insert_penalties| in the presence of
//! |set_page_int|. These definitions are always global.
//!
//! When some dimension of a box register is changed, the change isn't exactly
//! global; but \TeX\ does not look at the \.{\\global} switch.
//!
//! @<Assignments@>=
//! set_aux:alter_aux;
//! set_prev_graf:alter_prev_graf;
//! set_page_dimen:alter_page_so_far;
//! set_page_int:alter_integer;
//! set_box_dimen:alter_box_dimen;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_aux;
//! var c:halfword; {|hmode| or |vmode|}
//! begin if cur_chr<>abs(mode) then report_illegal_case
//! else  begin c:=cur_chr; scan_optional_equals;
//!   if c=vmode then
//!     begin scan_normal_dimen; prev_depth:=cur_val;
//!     end
//!   else  begin scan_int;
//!     if (cur_val<=0)or(cur_val>32767) then
//!       begin print_err("Bad space factor");
//! @.Bad space factor@>
//!       help1("I allow only values in the range 1..32767 here.");
//!       int_error(cur_val);
//!       end
//!     else space_factor:=cur_val;
//!     end;
//!   end;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_prev_graf;
//! var p:0..nest_size; {index into |nest|}
//! begin nest[nest_ptr]:=cur_list; p:=nest_ptr;
//! while abs(nest[p].mode_field)<>vmode do decr(p);
//! scan_optional_equals; scan_int;
//! if cur_val<0 then
//!   begin print_err("Bad "); print_esc("prevgraf");
//! @.Bad \\prevgraf@>
//!   help1("I allow only nonnegative values here.");
//!   int_error(cur_val);
//!   end
//! else  begin nest[p].pg_field:=cur_val; cur_list:=nest[nest_ptr];
//!   end;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_page_so_far;
//! var c:0..7; {index into |page_so_far|}
//! begin c:=cur_chr; scan_optional_equals; scan_normal_dimen;
//! page_so_far[c]:=cur_val;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_integer;
//! var c:0..1; {0 for \.{\\deadcycles}, 1 for \.{\\insertpenalties}}
//! begin c:=cur_chr; scan_optional_equals; scan_int;
//! if c=0 then dead_cycles:=cur_val
//! else insert_penalties:=cur_val;
//! end;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure alter_box_dimen;
//! var c:small_number; {|width_offset| or |height_offset| or |depth_offset|}
//! @!b:eight_bits; {box number}
//! begin c:=cur_chr; scan_eight_bit_int; b:=cur_val; scan_optional_equals;
//! scan_normal_dimen;
//! if box(b)<>null then mem[box(b)+c].sc:=cur_val;
//! end;
//!
//! @ Paragraph shapes are set up in the obvious way.
//!
//! @<Assignments@>=
//! set_shape: begin scan_optional_equals; scan_int; n:=cur_val;
//!   if n<=0 then p:=null
//!   else  begin p:=get_node(2*n+1); info(p):=n;
//!     for j:=1 to n do
//!       begin scan_normal_dimen;
//!       mem[p+2*j-1].sc:=cur_val; {indentation}
//!       scan_normal_dimen;
//!       mem[p+2*j].sc:=cur_val; {width}
//!       end;
//!     end;
//!   define(par_shape_loc,shape_ref,p);
//!   end;
//!
//! @ Here's something that isn't quite so obvious. It guarantees that
//! |info(par_shape_ptr)| can hold any positive~|n| for which |get_node(2*n+1)|
//! doesn't overflow the memory capacity.
//!
//! @<Check the ``constant''...@>=
//! if 2*max_halfword<mem_top-mem_min then bad:=41;
//!
//! @ New hyphenation data is loaded by the |hyph_data| command.
//!
//! @<Put each...@>=
//! primitive("hyphenation",hyph_data,0);
//! @!@:hyphenation_}{\.{\\hyphenation} primitive@>
//! primitive("patterns",hyph_data,1);
//! @!@:patterns_}{\.{\\patterns} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! hyph_data: if chr_code=1 then print_esc("patterns")
//!   else print_esc("hyphenation");
//!
//! @ @<Assignments@>=
//! hyph_data: if cur_chr=1 then
//!     begin @!init new_patterns; goto done;@;@+tini@/
//!     print_err("Patterns can be loaded only by INITEX");
//! @.Patterns can be...@>
//!     help0; error;
//!     repeat get_token; until cur_cmd=right_brace; {flush the patterns}
//!     return;
//!     end
//!   else  begin new_hyph_exceptions; goto done;
//!     end;
//!
//! @ All of \TeX's parameters are kept in |eqtb| except the font information,
//! the interaction mode, and the hyphenation tables; these are strictly global.
//!
//! @<Assignments@>=
//! assign_font_dimen: begin find_font_dimen(true); k:=cur_val;
//!   scan_optional_equals; scan_normal_dimen; font_info[k].sc:=cur_val;
//!   end;
//! assign_font_int: begin n:=cur_chr; scan_font_ident; f:=cur_val;
//!   scan_optional_equals; scan_int;
//!   if n=0 then hyphen_char[f]:=cur_val@+else skew_char[f]:=cur_val;
//!   end;
//!
//! @ @<Put each...@>=
//! primitive("hyphenchar",assign_font_int,0);
//! @!@:hyphen_char_}{\.{\\hyphenchar} primitive@>
//! primitive("skewchar",assign_font_int,1);
//! @!@:skew_char_}{\.{\\skewchar} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_font_int: if chr_code=0 then print_esc("hyphenchar")
//!   else print_esc("skewchar");
//!
//! @ Here is where the information for a new font gets loaded.
//!
//! @<Assignments@>=
//! def_font: new_font(a);
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure new_font(@!a:small_number);
//! label common_ending;
//! var u:pointer; {user's font identifier}
//! @!s:scaled; {stated ``at'' size, or negative of scaled magnification}
//! @!f:internal_font_number; {runs through existing fonts}
//! @!t:str_number; {name for the frozen font identifier}
//! @!old_setting:0..max_selector; {holds |selector| setting}
//! @!flushable_string:str_number; {string not yet referenced}
//! begin if job_name=0 then open_log_file;
//!   {avoid confusing \.{texput} with the font name}
//! @.texput@>
//! get_r_token; u:=cur_cs;
//! if u>=hash_base then t:=text(u)
//! else if u>=single_base then
//!   if u=null_cs then t:="FONT"@+else t:=u-single_base
//! else  begin old_setting:=selector; selector:=new_string;
//!   print("FONT"); print(u-active_base); selector:=old_setting;
//! @.FONTx@>
//!   str_room(1); t:=make_string;
//!   end;
//! define(u,set_font,null_font); scan_optional_equals; scan_file_name;
//! @<Scan the font size specification@>;
//! @<If this font has already been loaded, set |f| to the internal
//!   font number and |goto common_ending|@>;
//! f:=read_font_info(u,cur_name,cur_area,s);
//! common_ending: equiv(u):=f; eqtb[font_id_base+f]:=eqtb[u]; font_id_text(f):=t;
//! end;
//!
//! @ @<Scan the font size specification@>=
//! name_in_progress:=true; {this keeps |cur_name| from being changed}
//! if scan_keyword("at") then @<Put the \(p)(positive) `at' size into |s|@>
//! @.at@>
//! else if scan_keyword("scaled") then
//! @.scaled@>
//!   begin scan_int; s:=-cur_val;
//!   if (cur_val<=0)or(cur_val>32768) then
//!     begin print_err("Illegal magnification has been changed to 1000");@/
//! @.Illegal magnification...@>
//!     help1("The magnification ratio must be between 1 and 32768.");
//!     int_error(cur_val); s:=-1000;
//!     end;
//!   end
//! else s:=-1000;
//! name_in_progress:=false
//!
//! @ @<Put the \(p)(positive) `at' size into |s|@>=
//! begin scan_normal_dimen; s:=cur_val;
//! if (s<=0)or(s>=@'1000000000) then
//!   begin print_err("Improper `at' size (");
//!   print_scaled(s); print("pt), replaced by 10pt");
//! @.Improper `at' size...@>
//!   help2("I can only handle fonts at positive sizes that are")@/
//!   ("less than 2048pt, so I've changed what you said to 10pt.");
//!   error; s:=10*unity;
//!   end;
//! end
//!
//! @ When the user gives a new identifier to a font that was previously loaded,
//! the new name becomes the font identifier of record. Font names `\.{xyz}' and
//! `\.{XYZ}' are considered to be different.
//!
//! @<If this font has already been loaded...@>=
//! flushable_string:=str_ptr-1;
//! for f:=font_base+1 to font_ptr do
//!   if str_eq_str(font_name[f],cur_name)and str_eq_str(font_area[f],cur_area) then
//!     begin if cur_name=flushable_string then
//!       begin flush_string; cur_name:=font_name[f];
//!       end;
//!     if s>0 then
//!       begin if s=font_size[f] then goto common_ending;
//!       end
//!     else if font_size[f]=xn_over_d(font_dsize[f],-s,1000) then
//!       goto common_ending;
//!     end
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_font:begin print("select font "); slow_print(font_name[chr_code]);
//!   if font_size[chr_code]<>font_dsize[chr_code] then
//!     begin print(" at "); print_scaled(font_size[chr_code]);
//!     print("pt");
//!     end;
//!   end;
//!
//! @ @<Put each...@>=
//! primitive("batchmode",set_interaction,batch_mode);
//! @!@:batch_mode_}{\.{\\batchmode} primitive@>
//! primitive("nonstopmode",set_interaction,nonstop_mode);
//! @!@:nonstop_mode_}{\.{\\nonstopmode} primitive@>
//! primitive("scrollmode",set_interaction,scroll_mode);
//! @!@:scroll_mode_}{\.{\\scrollmode} primitive@>
//! primitive("errorstopmode",set_interaction,error_stop_mode);
//! @!@:error_stop_mode_}{\.{\\errorstopmode} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_interaction: case chr_code of
//!   batch_mode: print_esc("batchmode");
//!   nonstop_mode: print_esc("nonstopmode");
//!   scroll_mode: print_esc("scrollmode");
//!   othercases print_esc("errorstopmode")
//!   endcases;
//!
//! @ @<Assignments@>=
//! set_interaction: new_interaction;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure new_interaction;
//! begin print_ln;
//! interaction:=cur_chr;
//! @<Initialize the print |selector| based on |interaction|@>;
//! if log_opened then selector:=selector+2;
//! end;
//!
//! @ The \.{\\afterassignment} command puts a token into the global
//! variable |after_token|. This global variable is examined just after
//! every assignment has been performed.
//!
//! @<Glob...@>=
//! @!after_token:halfword; {zero, or a saved token}
//!
//! @ @<Set init...@>=
//! after_token:=0;
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(after_assignment):begin get_token; after_token:=cur_tok;
//!   end;
//!
//! @ @<Insert a token saved by \.{\\afterassignment}, if any@>=
//! if after_token<>0 then
//!   begin cur_tok:=after_token; back_input; after_token:=0;
//!   end
//!
//! @ Here is a procedure that might be called `Get the next non-blank non-relax
//! non-call non-assignment token'.
//!
//! @<Declare act...@>=
//! procedure do_assignments;
//! label exit;
//! begin loop begin @<Get the next non-blank non-relax...@>;
//!   if cur_cmd<=max_non_prefixed_command then return;
//!   set_box_allowed:=false; prefixed_command; set_box_allowed:=true;
//!   end;
//! exit:end;
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(after_group):begin get_token; save_for_after(cur_tok);
//!   end;
//!
//! @ Files for \.{\\read} are opened and closed by the |in_stream| command.
//!
//! @<Put each...@>=
//! primitive("openin",in_stream,1);
//! @!@:open_in_}{\.{\\openin} primitive@>
//! primitive("closein",in_stream,0);
//! @!@:close_in_}{\.{\\closein} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! in_stream: if chr_code=0 then print_esc("closein")
//!   else print_esc("openin");
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(in_stream): open_or_close_in;
//!
//! @ @<Declare act...@>=
//! procedure open_or_close_in;
//! var c:0..1; {1 for \.{\\openin}, 0 for \.{\\closein}}
//! @!n:0..15; {stream number}
//! begin c:=cur_chr; scan_four_bit_int; n:=cur_val;
//! if read_open[n]<>closed then
//!   begin a_close(read_file[n]); read_open[n]:=closed;
//!   end;
//! if c<>0 then
//!   begin scan_optional_equals; scan_file_name;
//!   if cur_ext="" then cur_ext:=".tex";
//!   pack_cur_name;
//!   if a_open_in(read_file[n]) then read_open[n]:=just_open;
//!   end;
//! end;
//!
