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