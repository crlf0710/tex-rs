//! @ The global variable |adjust_tail| will be non-null if and only if the
//! current box might include adjustments that should be appended to the
//! current vertical list.
//!
//! @<Append box |cur_box| to the current...@>=
//! begin if cur_box<>null then
//!   begin shift_amount(cur_box):=box_context;
//!   if abs(mode)=vmode then
//!     begin append_to_vlist(cur_box);
//!     if adjust_tail<>null then
//!       begin if adjust_head<>adjust_tail then
//!         begin link(tail):=link(adjust_head); tail:=adjust_tail;
//!         end;
//!       adjust_tail:=null;
//!       end;
//!     if mode>0 then build_page;
//!     end
//!   else  begin if abs(mode)=hmode then space_factor:=1000
//!     else  begin p:=new_noad;
//!       math_type(nucleus(p)):=sub_box;
//!       info(nucleus(p)):=cur_box; cur_box:=p;
//!       end;
//!     link(tail):=cur_box; tail:=cur_box;
//!     end;
//!   end;
//! end
//!
//! @ @<Store \(c)|cur_box| in a box register@>=
//! if box_context<box_flag+256 then
//!   eq_define(box_base-box_flag+box_context,box_ref,cur_box)
//! else geq_define(box_base-box_flag-256+box_context,box_ref,cur_box)
//!
//! @ @<Append a new leader node ...@>=
//! begin @<Get the next non-blank non-relax...@>;
//! if ((cur_cmd=hskip)and(abs(mode)<>vmode))or@|
//!    ((cur_cmd=vskip)and(abs(mode)=vmode)) then
//!   begin append_glue; subtype(tail):=box_context-(leader_flag-a_leaders);
//!   leader_ptr(tail):=cur_box;
//!   end
//! else  begin print_err("Leaders not followed by proper glue");
//! @.Leaders not followed by...@>
//!   help3("You should say `\leaders <box or rule><hskip or vskip>'.")@/
//!   ("I found the <box or rule>, but there's no suitable")@/
//!   ("<hskip or vskip>, so I'm ignoring these leaders."); back_error;
//!   flush_node_list(cur_box);
//!   end;
//! end
//!
