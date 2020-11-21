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
