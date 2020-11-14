//! @ @<Fetch a token list...@>=
//! if level<>tok_val then
//!   begin print_err("Missing number, treated as zero");
//! @.Missing number...@>
//!   help3("A number should have been here; I inserted `0'.")@/
//!     ("(If you can't figure out why I needed to see a number,")@/
//!     ("look up `weird error' in the index to The TeXbook.)");
//! @:TeXbook}{\sl The \TeX book@>
//!   back_error; scanned_result(0)(dimen_val);
//!   end
//! else if cur_cmd<=assign_toks then
//!   begin if cur_cmd<assign_toks then {|cur_cmd=toks_register|}
//!     begin scan_eight_bit_int; m:=toks_base+cur_val;
//!     end;
//!   scanned_result(equiv(m))(tok_val);
//!   end
//! else  begin back_input; scan_font_ident;
//!   scanned_result(font_id_base+cur_val)(ident_val);
//!   end
//!
//! @ Users refer to `\.{\\the\\spacefactor}' only in horizontal
//! mode, and to `\.{\\the\\prevdepth}' only in vertical mode; so we put the
//! associated mode in the modifier part of the |set_aux| command.
//! The |set_page_int| command has modifier 0 or 1, for `\.{\\deadcycles}' and
//! `\.{\\insertpenalties}', respectively. The |set_box_dimen| command is
//! modified by either |width_offset|, |height_offset|, or |depth_offset|.
//! And the |last_item| command is modified by either |int_val|, |dimen_val|,
//! |glue_val|, |input_line_no_code|, or |badness_code|.
//!
//! @d input_line_no_code=glue_val+1 {code for \.{\\inputlineno}}
//! @d badness_code=glue_val+2 {code for \.{\\badness}}
//!
//! @<Put each...@>=
//! primitive("spacefactor",set_aux,hmode);
//! @!@:space_factor_}{\.{\\spacefactor} primitive@>
//! primitive("prevdepth",set_aux,vmode);@/
//! @!@:prev_depth_}{\.{\\prevdepth} primitive@>
//! primitive("deadcycles",set_page_int,0);
//! @!@:dead_cycles_}{\.{\\deadcycles} primitive@>
//! primitive("insertpenalties",set_page_int,1);
//! @!@:insert_penalties_}{\.{\\insertpenalties} primitive@>
//! primitive("wd",set_box_dimen,width_offset);
//! @!@:wd_}{\.{\\wd} primitive@>
//! primitive("ht",set_box_dimen,height_offset);
//! @!@:ht_}{\.{\\ht} primitive@>
//! primitive("dp",set_box_dimen,depth_offset);
//! @!@:dp_}{\.{\\dp} primitive@>
//! primitive("lastpenalty",last_item,int_val);
//! @!@:last_penalty_}{\.{\\lastpenalty} primitive@>
//! primitive("lastkern",last_item,dimen_val);
//! @!@:last_kern_}{\.{\\lastkern} primitive@>
//! primitive("lastskip",last_item,glue_val);
//! @!@:last_skip_}{\.{\\lastskip} primitive@>
//! primitive("inputlineno",last_item,input_line_no_code);
//! @!@:input_line_no_}{\.{\\inputlineno} primitive@>
//! primitive("badness",last_item,badness_code);
//! @!@:badness_}{\.{\\badness} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_aux: if chr_code=vmode then print_esc("prevdepth")
//! @+else print_esc("spacefactor");
//! set_page_int: if chr_code=0 then print_esc("deadcycles")
//! @+else print_esc("insertpenalties");
//! set_box_dimen: if chr_code=width_offset then print_esc("wd")
//! else if chr_code=height_offset then print_esc("ht")
//! else print_esc("dp");
//! last_item: case chr_code of
//!   int_val: print_esc("lastpenalty");
//!   dimen_val: print_esc("lastkern");
//!   glue_val: print_esc("lastskip");
//!   input_line_no_code: print_esc("inputlineno");
//!   othercases print_esc("badness")
//!   endcases;
//!
//! @ @<Fetch the |space_factor| or the |prev_depth|@>=
//! if abs(mode)<>m then
//!   begin print_err("Improper "); print_cmd_chr(set_aux,m);
//! @.Improper \\spacefactor@>
//! @.Improper \\prevdepth@>
//!   help4("You can refer to \spacefactor only in horizontal mode;")@/
//!     ("you can refer to \prevdepth only in vertical mode; and")@/
//!     ("neither of these is meaningful inside \write. So")@/
//!     ("I'm forgetting what you said and using zero instead.");
//!   error;
//!   if level<>tok_val then scanned_result(0)(dimen_val)
//!   else scanned_result(0)(int_val);
//!   end
//! else if m=vmode then scanned_result(prev_depth)(dimen_val)
//! else scanned_result(space_factor)(int_val)
//!
//! @ @<Fetch the |dead_cycles| or the |insert_penalties|@>=
//! begin if m=0 then cur_val:=dead_cycles@+else cur_val:=insert_penalties;
//! cur_val_level:=int_val;
//! end
//!
//! @ @<Fetch a box dimension@>=
//! begin scan_eight_bit_int;
//! if box(cur_val)=null then cur_val:=0 @+else cur_val:=mem[box(cur_val)+m].sc;
//! cur_val_level:=dimen_val;
//! end
//!
//! @ Inside an \.{\\output} routine, a user may wish to look at the page totals
//! that were present at the moment when output was triggered.
//!
//! @d max_dimen==@'7777777777 {$2^{30}-1$}
//!
//! @<Fetch something on the |page_so_far|@>=
//! begin if (page_contents=empty) and (not output_active) then
//!   if m=0 then cur_val:=max_dimen@+else cur_val:=0
//! else cur_val:=page_so_far[m];
//! cur_val_level:=dimen_val;
//! end
//!
//! @ @<Fetch the |prev_graf|@>=
//! if mode=0 then scanned_result(0)(int_val) {|prev_graf=0| within \.{\\write}}
//! else begin nest[nest_ptr]:=cur_list; p:=nest_ptr;
//!   while abs(nest[p].mode_field)<>vmode do decr(p);
//!   scanned_result(nest[p].pg_field)(int_val);
//!   end
//!
//! @ @<Fetch the |par_shape| size@>=
//! begin if par_shape_ptr=null then cur_val:=0
//! else cur_val:=info(par_shape_ptr);
//! cur_val_level:=int_val;
//! end
//!
//! @ Here is where \.{\\lastpenalty}, \.{\\lastkern}, and \.{\\lastskip} are
//! implemented. The reference count for \.{\\lastskip} will be updated later.
//!
//! We also handle \.{\\inputlineno} and \.{\\badness} here, because they are
//! legal in similar contexts.
//!
//! @<Fetch an item in the current node...@>=
//! if cur_chr>glue_val then
//!   begin if cur_chr=input_line_no_code then cur_val:=line
//!   else cur_val:=last_badness; {|cur_chr=badness_code|}
//!   cur_val_level:=int_val;
//!   end
//! else begin if cur_chr=glue_val then cur_val:=zero_glue@+else cur_val:=0;
//!   cur_val_level:=cur_chr;
//!   if not is_char_node(tail)and(mode<>0) then
//!     case cur_chr of
//!     int_val: if type(tail)=penalty_node then cur_val:=penalty(tail);
//!     dimen_val: if type(tail)=kern_node then cur_val:=width(tail);
//!     glue_val: if type(tail)=glue_node then
//!       begin cur_val:=glue_ptr(tail);
//!       if subtype(tail)=mu_glue then cur_val_level:=mu_val;
//!       end;
//!     end {there are no other cases}
//!   else if (mode=vmode)and(tail=head) then
//!     case cur_chr of
//!     int_val: cur_val:=last_penalty;
//!     dimen_val: cur_val:=last_kern;
//!     glue_val: if last_glue<>max_halfword then cur_val:=last_glue;
//!     end; {there are no other cases}
//!   end
//!
//! @ @<Fetch a font dimension@>=
//! begin find_font_dimen(false); font_info[fmem_ptr].sc:=0;
//! scanned_result(font_info[cur_val].sc)(dimen_val);
//! end
//!
//! @ @<Fetch a font integer@>=
//! begin scan_font_ident;
//! if m=0 then scanned_result(hyphen_char[cur_val])(int_val)
//! else scanned_result(skew_char[cur_val])(int_val);
//! end
//!
//! @ @<Fetch a register@>=
//! begin scan_eight_bit_int;
//! case m of
//! int_val:cur_val:=count(cur_val);
//! dimen_val:cur_val:=dimen(cur_val);
//! glue_val: cur_val:=skip(cur_val);
//! mu_val: cur_val:=mu_skip(cur_val);
//! end; {there are no other cases}
//! cur_val_level:=m;
//! end
//!
//! @ @<Complain that \.{\\the} can't do this; give zero result@>=
//! begin print_err("You can't use `"); print_cmd_chr(cur_cmd,cur_chr);
//! @.You can't use x after ...@>
//! print("' after "); print_esc("the");
//! help1("I'm forgetting what you said and using zero instead.");
//! error;
//! if level<>tok_val then scanned_result(0)(dimen_val)
//! else scanned_result(0)(int_val);
//! end
//!
//! @ When a |glue_val| changes to a |dimen_val|, we use the width component
//! of the glue; there is no need to decrease the reference count, since it
//! has not yet been increased.  When a |dimen_val| changes to an |int_val|,
//! we use scaled points so that the value doesn't actually change. And when a
//! |mu_val| changes to a |glue_val|, the value doesn't change either.
//!
//! @<Convert \(c)|cur_val| to a lower level@>=
//! begin if cur_val_level=glue_val then cur_val:=width(cur_val)
//! else if cur_val_level=mu_val then mu_error;
//! decr(cur_val_level);
//! end
//!
//! @ If |cur_val| points to a glue specification at this point, the reference
//! count for the glue does not yet include the reference by |cur_val|.
//! If |negative| is |true|, |cur_val_level| is known to be |<=mu_val|.
//!
//! @<Fix the reference count, if any, ...@>=
//! if negative then
//!   if cur_val_level>=glue_val then
//!     begin cur_val:=new_spec(cur_val);
//!     @<Negate all three glue components of |cur_val|@>;
//!     end
//!   else negate(cur_val)
//! else if (cur_val_level>=glue_val)and(cur_val_level<=mu_val) then
//!   add_glue_ref(cur_val)
//!
//! @ @<Negate all three...@>=
//! begin negate(width(cur_val));
//! negate(stretch(cur_val));
//! negate(shrink(cur_val));
//! end
//!
//! @ Our next goal is to write the |scan_int| procedure, which scans anything that
//! \TeX\ treats as an integer. But first we might as well look at some simple
//! applications of |scan_int| that have already been made inside of
//! |scan_something_internal|.
//!
