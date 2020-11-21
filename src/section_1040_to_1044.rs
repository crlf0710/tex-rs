//! @ When a ligature or kern instruction matches a character, we know from
//! |read_font_info| that the character exists in the font, even though we
//! haven't verified its existence in the normal way.
//!
//! This section could be made into a subroutine, if the code inside
//! |main_control| needs to be shortened.
//!
//! \chardef\?='174 % vertical line to indicate character retention
//!
//! @<Do ligature or kern command...@>=
//! begin if op_byte(main_j)>=kern_flag then
//!   begin wrapup(rt_hit);
//!   tail_append(new_kern(char_kern(main_f)(main_j))); goto main_loop_move;
//!   end;
//! if cur_l=non_char then lft_hit:=true
//! else if lig_stack=null then rt_hit:=true;
//! check_interrupt; {allow a way out in case there's an infinite ligature loop}
//! case op_byte(main_j) of
//! qi(1),qi(5):begin cur_l:=rem_byte(main_j); {\.{=:\?}, \.{=:\?>}}
//!   main_i:=char_info(main_f)(cur_l); ligature_present:=true;
//!   end;
//! qi(2),qi(6):begin cur_r:=rem_byte(main_j); {\.{\?=:}, \.{\?=:>}}
//!   if lig_stack=null then {right boundary character is being consumed}
//!     begin lig_stack:=new_lig_item(cur_r); bchar:=non_char;
//!     end
//!   else if is_char_node(lig_stack) then {|link(lig_stack)=null|}
//!     begin main_p:=lig_stack; lig_stack:=new_lig_item(cur_r);
//!     lig_ptr(lig_stack):=main_p;
//!     end
//!   else character(lig_stack):=cur_r;
//!   end;
//! qi(3):begin cur_r:=rem_byte(main_j); {\.{\?=:\?}}
//!   main_p:=lig_stack; lig_stack:=new_lig_item(cur_r);
//!   link(lig_stack):=main_p;
//!   end;
//! qi(7),qi(11):begin wrapup(false); {\.{\?=:\?>}, \.{\?=:\?>>}}
//!   cur_q:=tail; cur_l:=rem_byte(main_j);
//!   main_i:=char_info(main_f)(cur_l); ligature_present:=true;
//!   end;
//! othercases begin cur_l:=rem_byte(main_j); ligature_present:=true; {\.{=:}}
//!   if lig_stack=null then goto main_loop_wrapup
//!   else goto main_loop_move+1;
//!   end
//! endcases;
//! if op_byte(main_j)>qi(4) then
//!   if op_byte(main_j)<>qi(7) then goto main_loop_wrapup;
//! if cur_l<non_char then goto main_lig_loop;
//! main_k:=bchar_label[main_f]; goto main_lig_loop+1;
//! end
//!
//! @ The occurrence of blank spaces is almost part of \TeX's inner loop,
//! @^inner loop@>
//! since we usually encounter about one space for every five non-blank characters.
//! Therefore |main_control| gives second-highest priority to ordinary spaces.
//!
//! When a glue parameter like \.{\\spaceskip} is set to `\.{0pt}', we will
//! see to it later that the corresponding glue specification is precisely
//! |zero_glue|, not merely a pointer to some specification that happens
//! to be full of zeroes. Therefore it is simple to test whether a glue parameter
//! is zero or~not.
//!
//! @<Append a normal inter-word space...@>=
//! if space_skip=zero_glue then
//!   begin @<Find the glue specification, |main_p|, for
//!     text spaces in the current font@>;
//!   temp_ptr:=new_glue(main_p);
//!   end
//! else temp_ptr:=new_param_glue(space_skip_code);
//! link(tail):=temp_ptr; tail:=temp_ptr;
//! goto big_switch
//!
//! @ Having |font_glue| allocated for each text font saves both time and memory.
//! If any of the three spacing parameters are subsequently changed by the
//! use of \.{\\fontdimen}, the |find_font_dimen| procedure deallocates the
//! |font_glue| specification allocated here.
//!
//! @<Find the glue specification...@>=
//! begin main_p:=font_glue[cur_font];
//! if main_p=null then
//!   begin main_p:=new_spec(zero_glue); main_k:=param_base[cur_font]+space_code;
//!   width(main_p):=font_info[main_k].sc; {that's |space(cur_font)|}
//!   stretch(main_p):=font_info[main_k+1].sc; {and |space_stretch(cur_font)|}
//!   shrink(main_p):=font_info[main_k+2].sc; {and |space_shrink(cur_font)|}
//!   font_glue[cur_font]:=main_p;
//!   end;
//! end
//!
//! @ @<Declare act...@>=
//! procedure app_space; {handle spaces when |space_factor<>1000|}
//! var@!q:pointer; {glue node}
//! begin if (space_factor>=2000)and(xspace_skip<>zero_glue) then
//!   q:=new_param_glue(xspace_skip_code)
//! else  begin if space_skip<>zero_glue then main_p:=space_skip
//!   else @<Find the glue specification...@>;
//!   main_p:=new_spec(main_p);
//!   @<Modify the glue specification in |main_p| according to the space factor@>;
//!   q:=new_glue(main_p); glue_ref_count(main_p):=null;
//!   end;
//! link(tail):=q; tail:=q;
//! end;
//!
//! @ @<Modify the glue specification in |main_p| according to the space factor@>=
//! if space_factor>=2000 then width(main_p):=width(main_p)+extra_space(cur_font);
//! stretch(main_p):=xn_over_d(stretch(main_p),space_factor,1000);
//! shrink(main_p):=xn_over_d(shrink(main_p),1000,space_factor)
//!
