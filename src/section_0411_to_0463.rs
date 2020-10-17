//! @ The hash table is initialized with `\.{\\count}', `\.{\\dimen}', `\.{\\skip}',
//! and `\.{\\muskip}' all having |register| as their command code; they are
//! distinguished by the |chr_code|, which is either |int_val|, |dimen_val|,
//! |glue_val|, or |mu_val|.
//!
//! @<Put each...@>=
//! primitive("count",register,int_val);
//! @!@:count_}{\.{\\count} primitive@>
//! primitive("dimen",register,dimen_val);
//! @!@:dimen_}{\.{\\dimen} primitive@>
//! primitive("skip",register,glue_val);
//! @!@:skip_}{\.{\\skip} primitive@>
//! primitive("muskip",register,mu_val);
//! @!@:mu_skip_}{\.{\\muskip} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! register: if chr_code=int_val then print_esc("count")
//!   else if chr_code=dimen_val then print_esc("dimen")
//!   else if chr_code=glue_val then print_esc("skip")
//!   else print_esc("muskip");
//!
//! @ OK, we're ready for |scan_something_internal| itself. A second parameter,
//! |negative|, is set |true| if the value that is found should be negated.
//! It is assumed that |cur_cmd| and |cur_chr| represent the first token of
//! the internal quantity to be scanned; an error will be signalled if
//! |cur_cmd<min_internal| or |cur_cmd>max_internal|.
//!
//! @d scanned_result_end(#)==cur_val_level:=#;@+end
//! @d scanned_result(#)==@+begin cur_val:=#;scanned_result_end
//!
//! @p procedure scan_something_internal(@!level:small_number;@!negative:boolean);
//!   {fetch an internal parameter}
//! var m:halfword; {|chr_code| part of the operand token}
//! @!p:0..nest_size; {index into |nest|}
//! begin m:=cur_chr;
//! case cur_cmd of
//! def_code: @<Fetch a character code from some table@>;
//! toks_register,assign_toks,def_family,set_font,def_font: @<Fetch a token list or
//!   font identifier, provided that |level=tok_val|@>;
//! assign_int: scanned_result(eqtb[m].int)(int_val);
//! assign_dimen: scanned_result(eqtb[m].sc)(dimen_val);
//! assign_glue: scanned_result(equiv(m))(glue_val);
//! assign_mu_glue: scanned_result(equiv(m))(mu_val);
//! set_aux: @<Fetch the |space_factor| or the |prev_depth|@>;
//! set_prev_graf: @<Fetch the |prev_graf|@>;
//! set_page_int:@<Fetch the |dead_cycles| or the |insert_penalties|@>;
//! set_page_dimen: @<Fetch something on the |page_so_far|@>;
//! set_shape: @<Fetch the |par_shape| size@>;
//! set_box_dimen: @<Fetch a box dimension@>;
//! char_given,math_given: scanned_result(cur_chr)(int_val);
//! assign_font_dimen: @<Fetch a font dimension@>;
//! assign_font_int: @<Fetch a font integer@>;
//! register: @<Fetch a register@>;
//! last_item: @<Fetch an item in the current node, if appropriate@>;
//! othercases @<Complain that \.{\\the} can't do this; give zero result@>
//! endcases;@/
//! while cur_val_level>level do @<Convert \(c)|cur_val| to a lower level@>;
//! @<Fix the reference count, if any, and negate |cur_val| if |negative|@>;
//! end;
//!
//! @ @<Fetch a character code from some table@>=
//! begin scan_char_num;
//! if m=math_code_base then scanned_result(ho(math_code(cur_val)))(int_val)
//! else if m<math_code_base then scanned_result(equiv(m+cur_val))(int_val)
//! else scanned_result(eqtb[m+cur_val].int)(int_val);
//! end
//!
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
//! @ @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_eight_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>255) then
//!   begin print_err("Bad register code");
//! @.Bad register code@>
//!   help2("A register number must be between 0 and 255.")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
//! @ @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_char_num;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>255) then
//!   begin print_err("Bad character code");
//! @.Bad character code@>
//!   help2("A character number must be between 0 and 255.")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
//! @ While we're at it, we might as well deal with similar routines that
//! will be needed later.
//!
//! @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_four_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>15) then
//!   begin print_err("Bad number");
//! @.Bad number@>
//!   help2("Since I expected to read a number between 0 and 15,")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
//! @ @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_fifteen_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>@'77777) then
//!   begin print_err("Bad mathchar");
//! @.Bad mathchar@>
//!   help2("A mathchar number must be between 0 and 32767.")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
//! @ @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_twenty_seven_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>@'777777777) then
//!   begin print_err("Bad delimiter code");
//! @.Bad delimiter code@>
//!   help2("A numeric delimiter code must be between 0 and 2^{27}-1.")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!
//! @ An integer number can be preceded by any number of spaces and `\.+' or
//! `\.-' signs. Then comes either a decimal constant (i.e., radix 10), an
//! octal constant (i.e., radix 8, preceded by~\.\'), a hexadecimal constant
//! (radix 16, preceded by~\."), an alphabetic constant (preceded by~\.\`), or
//! an internal variable. After scanning is complete,
//! |cur_val| will contain the answer, which must be at most
//! $2^{31}-1=2147483647$ in absolute value. The value of |radix| is set to
//! 10, 8, or 16 in the cases of decimal, octal, or hexadecimal constants,
//! otherwise |radix| is set to zero. An optional space follows a constant.
//!
//! @d octal_token=other_token+"'" {apostrophe, indicates an octal constant}
//! @d hex_token=other_token+"""" {double quote, indicates a hex constant}
//! @d alpha_token=other_token+"`" {reverse apostrophe, precedes alpha constants}
//! @d point_token=other_token+"." {decimal point}
//! @d continental_point_token=other_token+"," {decimal point, Eurostyle}
//!
//! @<Glob...@>=
//! @!radix:small_number; {|scan_int| sets this to 8, 10, 16, or zero}
//!
//! @ We initialize the following global variables just in case |expand|
//! comes into action before any of the basic scanning routines has assigned
//! them a value.
//!
//! @<Set init...@>=
//! cur_val:=0; cur_val_level:=int_val; radix:=0; cur_order:=normal;
//!
//! @ The |scan_int| routine is used also to scan the integer part of a
//! fraction; for example, the `\.3' in `\.{3.14159}' will be found by
//! |scan_int|. The |scan_dimen| routine assumes that |cur_tok=point_token|
//! after the integer part of such a fraction has been scanned by |scan_int|,
//! and that the decimal point has been backed up to be scanned again.
//!
//! @p procedure scan_int; {sets |cur_val| to an integer}
//! label done;
//! var negative:boolean; {should the answer be negated?}
//! @!m:integer; {|@t$2^{31}$@> div radix|, the threshold of danger}
//! @!d:small_number; {the digit just scanned}
//! @!vacuous:boolean; {have no digits appeared?}
//! @!OK_so_far:boolean; {has an error message been issued?}
//! begin radix:=0; OK_so_far:=true;@/
//! @<Get the next non-blank non-sign token; set |negative| appropriately@>;
//! if cur_tok=alpha_token then @<Scan an alphabetic character code into |cur_val|@>
//! else if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
//!   scan_something_internal(int_val,false)
//! else @<Scan a numeric constant@>;
//! if negative then negate(cur_val);
//! end;
//!
//! @ @<Get the next non-blank non-sign token...@>=
//! negative:=false;
//! repeat @<Get the next non-blank non-call token@>;
//! if cur_tok=other_token+"-" then
//!   begin negative := not negative; cur_tok:=other_token+"+";
//!   end;
//! until cur_tok<>other_token+"+"
//!
//! @ A space is ignored after an alphabetic character constant, so that
//! such constants behave like numeric ones.
//!
//! @<Scan an alphabetic character code into |cur_val|@>=
//! begin get_token; {suppress macro expansion}
//! if cur_tok<cs_token_flag then
//!   begin cur_val:=cur_chr;
//!   if cur_cmd<=right_brace then
//!     if cur_cmd=right_brace then incr(align_state)
//!     else decr(align_state);
//!   end
//! else if cur_tok<cs_token_flag+single_base then
//!   cur_val:=cur_tok-cs_token_flag-active_base
//! else cur_val:=cur_tok-cs_token_flag-single_base;
//! if cur_val>255 then
//!   begin print_err("Improper alphabetic constant");
//! @.Improper alphabetic constant@>
//!   help2("A one-character control sequence belongs after a ` mark.")@/
//!     ("So I'm essentially inserting \0 here.");
//!   cur_val:="0"; back_error;
//!   end
//! else @<Scan an optional space@>;
//! end
//!
//! @ @<Scan an optional space@>=
//! begin get_x_token; if cur_cmd<>spacer then back_input;
//! end
//!
//! @ @<Scan a numeric constant@>=
//! begin radix:=10; m:=214748364;
//! if cur_tok=octal_token then
//!   begin radix:=8; m:=@'2000000000; get_x_token;
//!   end
//! else if cur_tok=hex_token then
//!   begin radix:=16; m:=@'1000000000; get_x_token;
//!   end;
//! vacuous:=true; cur_val:=0;@/
//! @<Accumulate the constant until |cur_tok| is not a suitable digit@>;
//! if vacuous then @<Express astonishment that no number was here@>
//! else if cur_cmd<>spacer then back_input;
//! end
//!
//! @ @d infinity==@'17777777777 {the largest positive value that \TeX\ knows}
//! @d zero_token=other_token+"0" {zero, the smallest digit}
//! @d A_token=letter_token+"A" {the smallest special hex digit}
//! @d other_A_token=other_token+"A" {special hex digit of type |other_char|}
//!
//! @<Accumulate the constant...@>=
//! loop@+  begin if (cur_tok<zero_token+radix)and(cur_tok>=zero_token)and
//!     (cur_tok<=zero_token+9) then d:=cur_tok-zero_token
//!   else if radix=16 then
//!     if (cur_tok<=A_token+5)and(cur_tok>=A_token) then d:=cur_tok-A_token+10
//!     else if (cur_tok<=other_A_token+5)and(cur_tok>=other_A_token) then
//!       d:=cur_tok-other_A_token+10
//!     else goto done
//!   else goto done;
//!   vacuous:=false;
//!   if (cur_val>=m)and((cur_val>m)or(d>7)or(radix<>10)) then
//!     begin if OK_so_far then
//!       begin print_err("Number too big");
//! @.Number too big@>
//!       help2("I can only go up to 2147483647='17777777777=""7FFFFFFF,")@/
//!         ("so I'm using that number instead of yours.");
//!       error; cur_val:=infinity; OK_so_far:=false;
//!       end;
//!     end
//!   else cur_val:=cur_val*radix+d;
//!   get_x_token;
//!   end;
//! done:
//!
//! @ @<Express astonishment...@>=
//! begin print_err("Missing number, treated as zero");
//! @.Missing number...@>
//! help3("A number should have been here; I inserted `0'.")@/
//!   ("(If you can't figure out why I needed to see a number,")@/
//!   ("look up `weird error' in the index to The TeXbook.)");
//! @:TeXbook}{\sl The \TeX book@>
//! back_error;
//! end
//!
//! @ The |scan_dimen| routine is similar to |scan_int|, but it sets |cur_val| to
//! a |scaled| value, i.e., an integral number of sp. One of its main tasks
//! is therefore to interpret the abbreviations for various kinds of units and
//! to convert measurements to scaled points.
//!
//! There are three parameters: |mu| is |true| if the finite units must be
//! `\.{mu}', while |mu| is |false| if `\.{mu}' units are disallowed;
//! |inf| is |true| if the infinite units `\.{fil}', `\.{fill}', `\.{filll}'
//! are permitted; and |shortcut| is |true| if |cur_val| already contains
//! an integer and only the units need to be considered.
//!
//! The order of infinity that was found in the case of infinite glue is returned
//! in the global variable |cur_order|.
//!
//! @<Glob...@>=
//! @!cur_order:glue_ord; {order of infinity found by |scan_dimen|}
//!
//! @ Constructions like `\.{-\'77 pt}' are legal dimensions, so |scan_dimen|
//! may begin with |scan_int|. This explains why it is convenient to use
//! |scan_int| also for the integer part of a decimal fraction.
//!
//! Several branches of |scan_dimen| work with |cur_val| as an integer and
//! with an auxiliary fraction |f|, so that the actual quantity of interest is
//! $|cur_val|+|f|/2^{16}$. At the end of the routine, this ``unpacked''
//! representation is put into the single word |cur_val|, which suddenly
//! switches significance from |integer| to |scaled|.
//!
//! @d attach_fraction=88 {go here to pack |cur_val| and |f| into |cur_val|}
//! @d attach_sign=89 {go here when |cur_val| is correct except perhaps for sign}
//! @d scan_normal_dimen==scan_dimen(false,false,false)
//!
//! @p procedure scan_dimen(@!mu,@!inf,@!shortcut:boolean);
//!   {sets |cur_val| to a dimension}
//! label done, done1, done2, found, not_found, attach_fraction, attach_sign;
//! var negative:boolean; {should the answer be negated?}
//! @!f:integer; {numerator of a fraction whose denominator is $2^{16}$}
//! @<Local variables for dimension calculations@>@;
//! begin f:=0; arith_error:=false; cur_order:=normal; negative:=false;
//! if not shortcut then
//!   begin @<Get the next non-blank non-sign...@>;
//!   if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
//!     @<Fetch an internal dimension and |goto attach_sign|,
//!       or fetch an internal integer@>
//!   else  begin back_input;
//!     if cur_tok=continental_point_token then cur_tok:=point_token;
//!     if cur_tok<>point_token then scan_int
//!     else  begin radix:=10; cur_val:=0;
//!       end;
//!     if cur_tok=continental_point_token then cur_tok:=point_token;
//!     if (radix=10)and(cur_tok=point_token) then @<Scan decimal fraction@>;
//!     end;
//!   end;
//! if cur_val<0 then {in this case |f=0|}
//!   begin negative := not negative; negate(cur_val);
//!   end;
//! @<Scan units and set |cur_val| to $x\cdot(|cur_val|+f/2^{16})$, where there
//!   are |x| sp per unit; |goto attach_sign| if the units are internal@>;
//! @<Scan an optional space@>;
//! attach_sign: if arith_error or(abs(cur_val)>=@'10000000000) then
//!   @<Report that this dimension is out of range@>;
//! if negative then negate(cur_val);
//! end;
//!
//! @ @<Fetch an internal dimension and |goto attach_sign|...@>=
//! if mu then
//!   begin scan_something_internal(mu_val,false);
//!   @<Coerce glue to a dimension@>;
//!   if cur_val_level=mu_val then goto attach_sign;
//!   if cur_val_level<>int_val then mu_error;
//!   end
//! else  begin scan_something_internal(dimen_val,false);
//!   if cur_val_level=dimen_val then goto attach_sign;
//!   end
//!
//! @ @<Local variables for dimension calculations@>=
//! @!num,@!denom:1..65536; {conversion ratio for the scanned units}
//! @!k,@!kk:small_number; {number of digits in a decimal fraction}
//! @!p,@!q:pointer; {top of decimal digit stack}
//! @!v:scaled; {an internal dimension}
//! @!save_cur_val:integer; {temporary storage of |cur_val|}
//!
//! @ The following code is executed when |scan_something_internal| was
//! called asking for |mu_val|, when we really wanted a ``mudimen'' instead
//! of ``muglue.''
//!
//! @<Coerce glue to a dimension@>=
//! if cur_val_level>=glue_val then
//!   begin v:=width(cur_val); delete_glue_ref(cur_val); cur_val:=v;
//!   end
//!
//! @ When the following code is executed, we have |cur_tok=point_token|, but this
//! token has been backed up using |back_input|; we must first discard it.
//!
//! It turns out that a decimal point all by itself is equivalent to `\.{0.0}'.
//! Let's hope people don't use that fact.
//!
//! @<Scan decimal fraction@>=
//! begin k:=0; p:=null; get_token; {|point_token| is being re-scanned}
//! loop@+  begin get_x_token;
//!   if (cur_tok>zero_token+9)or(cur_tok<zero_token) then goto done1;
//!   if k<17 then {digits for |k>=17| cannot affect the result}
//!     begin q:=get_avail; link(q):=p; info(q):=cur_tok-zero_token;
//!     p:=q; incr(k);
//!     end;
//!   end;
//! done1: for kk:=k downto 1 do
//!   begin dig[kk-1]:=info(p); q:=p; p:=link(p); free_avail(q);
//!   end;
//! f:=round_decimals(k);
//! if cur_cmd<>spacer then back_input;
//! end
//!
//! @ Now comes the harder part: At this point in the program, |cur_val| is a
//! nonnegative integer and $f/2^{16}$ is a nonnegative fraction less than 1;
//! we want to multiply the sum of these two quantities by the appropriate
//! factor, based on the specified units, in order to produce a |scaled|
//! result, and we want to do the calculation with fixed point arithmetic that
//! does not overflow.
//!
//! @<Scan units and set |cur_val| to $x\cdot(|cur_val|+f/2^{16})$...@>=
//! if inf then @<Scan for \(f)\.{fil} units; |goto attach_fraction| if found@>;
//! @<Scan for \(u)units that are internal dimensions;
//!   |goto attach_sign| with |cur_val| set if found@>;
//! if mu then @<Scan for \(m)\.{mu} units and |goto attach_fraction|@>;
//! if scan_keyword("true") then @<Adjust \(f)for the magnification ratio@>;
//! @.true@>
//! if scan_keyword("pt") then goto attach_fraction; {the easy case}
//! @.pt@>
//! @<Scan for \(a)all other units and adjust |cur_val| and |f| accordingly;
//!   |goto done| in the case of scaled points@>;
//! attach_fraction: if cur_val>=@'40000 then arith_error:=true
//! else cur_val:=cur_val*unity+f;
//! done:
//!
//! @ A specification like `\.{filllll}' or `\.{fill L L L}' will lead to two
//! error messages (one for each additional keyword \.{"l"}).
//!
//! @<Scan for \(f)\.{fil} units...@>=
//! if scan_keyword("fil") then
//! @.fil@>
//!   begin cur_order:=fil;
//!   while scan_keyword("l") do
//!     begin if cur_order=filll then
//!       begin print_err("Illegal unit of measure (");
//! @.Illegal unit of measure@>
//!       print("replaced by filll)");
//!       help1("I dddon't go any higher than filll."); error;
//!       end
//!     else incr(cur_order);
//!     end;
//!   goto attach_fraction;
//!   end
//!
//! @ @<Scan for \(u)units that are internal dimensions...@>=
//! save_cur_val:=cur_val;
//! @<Get the next non-blank non-call...@>;
//! if (cur_cmd<min_internal)or(cur_cmd>max_internal) then back_input
//! else  begin if mu then
//!     begin scan_something_internal(mu_val,false); @<Coerce glue...@>;
//!     if cur_val_level<>mu_val then mu_error;
//!     end
//!   else scan_something_internal(dimen_val,false);
//!   v:=cur_val; goto found;
//!   end;
//! if mu then goto not_found;
//! if scan_keyword("em") then v:=(@<The em width for |cur_font|@>)
//! @.em@>
//! else if scan_keyword("ex") then v:=(@<The x-height for |cur_font|@>)
//! @.ex@>
//! else goto not_found;
//! @<Scan an optional space@>;
//! found:cur_val:=nx_plus_y(save_cur_val,v,xn_over_d(v,f,@'200000));
//! goto attach_sign;
//! not_found:
//!
//! @ @<Scan for \(m)\.{mu} units and |goto attach_fraction|@>=
//! if scan_keyword("mu") then goto attach_fraction
//! @.mu@>
//! else  begin print_err("Illegal unit of measure ("); print("mu inserted)");
//! @.Illegal unit of measure@>
//!   help4("The unit of measurement in math glue must be mu.")@/
//!     ("To recover gracefully from this error, it's best to")@/
//!     ("delete the erroneous units; e.g., type `2' to delete")@/
//!     ("two letters. (See Chapter 27 of The TeXbook.)");
//! @:TeXbook}{\sl The \TeX book@>
//!   error; goto attach_fraction;
//!   end
//!
//! @ @<Adjust \(f)for the magnification ratio@>=
//! begin prepare_mag;
//! if mag<>1000 then
//!   begin cur_val:=xn_over_d(cur_val,1000,mag);
//!   f:=(1000*f+@'200000*remainder) div mag;
//!   cur_val:=cur_val+(f div @'200000); f:=f mod @'200000;
//!   end;
//! end
//!
//! @ The necessary conversion factors can all be specified exactly as
//! fractions whose numerator and denominator sum to 32768 or less.
//! According to the definitions here, $\rm2660\,dd\approx1000.33297\,mm$;
//! this agrees well with the value $\rm1000.333\,mm$ cited by Bosshard
//! @^Bosshard, Hans Rudolf@>
//! in {\sl Technische Grundlagen zur Satzherstellung\/} (Bern, 1980).
//!
//! @d set_conversion_end(#)== denom:=#; end
//! @d set_conversion(#)==@+begin num:=#; set_conversion_end
//!
//! @<Scan for \(a)all other units and adjust |cur_val| and |f|...@>=
//! if scan_keyword("in") then set_conversion(7227)(100)
//! @.in@>
//! else if scan_keyword("pc") then set_conversion(12)(1)
//! @.pc@>
//! else if scan_keyword("cm") then set_conversion(7227)(254)
//! @.cm@>
//! else if scan_keyword("mm") then set_conversion(7227)(2540)
//! @.mm@>
//! else if scan_keyword("bp") then set_conversion(7227)(7200)
//! @.bp@>
//! else if scan_keyword("dd") then set_conversion(1238)(1157)
//! @.dd@>
//! else if scan_keyword("cc") then set_conversion(14856)(1157)
//! @.cc@>
//! else if scan_keyword("sp") then goto done
//! @.sp@>
//! else @<Complain about unknown unit and |goto done2|@>;
//! cur_val:=xn_over_d(cur_val,num,denom);
//! f:=(num*f+@'200000*remainder) div denom;@/
//! cur_val:=cur_val+(f div @'200000); f:=f mod @'200000;
//! done2:
//!
//! @ @<Complain about unknown unit...@>=
//! begin print_err("Illegal unit of measure ("); print("pt inserted)");
//! @.Illegal unit of measure@>
//! help6("Dimensions can be in units of em, ex, in, pt, pc,")@/
//!   ("cm, mm, dd, cc, bp, or sp; but yours is a new one!")@/
//!   ("I'll assume that you meant to say pt, for printer's points.")@/
//!   ("To recover gracefully from this error, it's best to")@/
//!   ("delete the erroneous units; e.g., type `2' to delete")@/
//!   ("two letters. (See Chapter 27 of The TeXbook.)");
//! @:TeXbook}{\sl The \TeX book@>
//! error; goto done2;
//! end
//!
//!
//! @ @<Report that this dimension is out of range@>=
//! begin print_err("Dimension too large");
//! @.Dimension too large@>
//! help2("I can't work with sizes bigger than about 19 feet.")@/
//!   ("Continue and I'll use the largest value I can.");@/
//! error; cur_val:=max_dimen; arith_error:=false;
//! end
//!
//! @ The final member of \TeX's value-scanning trio is |scan_glue|, which
//! makes |cur_val| point to a glue specification. The reference count of that
//! glue spec will take account of the fact that |cur_val| is pointing to~it.
//!
//! The |level| parameter should be either |glue_val| or |mu_val|.
//!
//! Since |scan_dimen| was so much more complex than |scan_int|, we might expect
//! |scan_glue| to be even worse. But fortunately, it is very simple, since
//! most of the work has already been done.
//!
//! @p procedure scan_glue(@!level:small_number);
//!   {sets |cur_val| to a glue spec pointer}
//! label exit;
//! var negative:boolean; {should the answer be negated?}
//! @!q:pointer; {new glue specification}
//! @!mu:boolean; {does |level=mu_val|?}
//! begin mu:=(level=mu_val); @<Get the next non-blank non-sign...@>;
//! if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
//!   begin scan_something_internal(level,negative);
//!   if cur_val_level>=glue_val then
//!     begin if cur_val_level<>level then mu_error;
//!     return;
//!     end;
//!   if cur_val_level=int_val then scan_dimen(mu,false,true)
//!   else if level=mu_val then mu_error;
//!   end
//! else  begin back_input; scan_dimen(mu,false,false);
//!   if negative then negate(cur_val);
//!   end;
//! @<Create a new glue specification whose width is |cur_val|; scan for its
//!   stretch and shrink components@>;
//! exit:end;
//!
//! @ @<Create a new glue specification whose width is |cur_val|...@>=
//! q:=new_spec(zero_glue); width(q):=cur_val;
//! if scan_keyword("plus") then
//! @.plus@>
//!   begin scan_dimen(mu,true,false);
//!   stretch(q):=cur_val; stretch_order(q):=cur_order;
//!   end;
//! if scan_keyword("minus") then
//! @.minus@>
//!   begin scan_dimen(mu,true,false);
//!   shrink(q):=cur_val; shrink_order(q):=cur_order;
//!   end;
//! cur_val:=q
//!
//! @ Here's a similar procedure that returns a pointer to a rule node. This
//! routine is called just after \TeX\ has seen \.{\\hrule} or \.{\\vrule};
//! therefore |cur_cmd| will be either |hrule| or |vrule|. The idea is to store
//! the default rule dimensions in the node, then to override them if
//! `\.{height}' or `\.{width}' or `\.{depth}' specifications are
//! found (in any order).
//!
//! @d default_rule=26214 {0.4\thinspace pt}
//!
//! @p function scan_rule_spec:pointer;
//! label reswitch;
//! var q:pointer; {the rule node being created}
//! begin q:=new_rule; {|width|, |depth|, and |height| all equal |null_flag| now}
//! if cur_cmd=vrule then width(q):=default_rule
//! else  begin height(q):=default_rule; depth(q):=0;
//!   end;
//! reswitch: if scan_keyword("width") then
//! @.width@>
//!   begin scan_normal_dimen; width(q):=cur_val; goto reswitch;
//!   end;
//! if scan_keyword("height") then
//! @.height@>
//!   begin scan_normal_dimen; height(q):=cur_val; goto reswitch;
//!   end;
//! if scan_keyword("depth") then
//! @.depth@>
//!   begin scan_normal_dimen; depth(q):=cur_val; goto reswitch;
//!   end;
//! scan_rule_spec:=q;
//! end;
//!
