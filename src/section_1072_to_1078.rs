//! @ @<Cases of |print_cmd_chr|...@>=
//! hmove: if chr_code=1 then print_esc("moveleft")@+else print_esc("moveright");
//! vmove: if chr_code=1 then print_esc("raise")@+else print_esc("lower");
//! make_box: case chr_code of
//!   box_code: print_esc("box");
//!   copy_code: print_esc("copy");
//!   last_box_code: print_esc("lastbox");
//!   vsplit_code: print_esc("vsplit");
//!   vtop_code: print_esc("vtop");
//!   vtop_code+vmode: print_esc("vbox");
//!   othercases print_esc("hbox")
//!   endcases;
//! leader_ship: if chr_code=a_leaders then print_esc("leaders")
//!   else if chr_code=c_leaders then print_esc("cleaders")
//!   else if chr_code=x_leaders then print_esc("xleaders")
//!   else print_esc("shipout");
//!
//! @ Constructions that require a box are started by calling |scan_box| with
//! a specified context code. The |scan_box| routine verifies
//! that a |make_box| command comes next and then it calls |begin_box|.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+hmove,hmode+vmove,mmode+vmove: begin t:=cur_chr;
//!   scan_normal_dimen;
//!   if t=0 then scan_box(cur_val)@+else scan_box(-cur_val);
//!   end;
//! any_mode(leader_ship): scan_box(leader_flag-a_leaders+cur_chr);
//! any_mode(make_box): begin_box(0);
//!
//! @ The global variable |cur_box| will point to a newly made box. If the box
//! is void, we will have |cur_box=null|. Otherwise we will have
//! |type(cur_box)=hlist_node| or |vlist_node| or |rule_node|; the |rule_node|
//! case can occur only with leaders.
//!
//! @<Glob...@>=
//! @!cur_box:pointer; {box to be placed into its context}
//!
//! @ The |box_end| procedure does the right thing with |cur_box|, if
//! |box_context| represents the context as explained above.
//!
//! @<Declare act...@>=
//! procedure box_end(@!box_context:integer);
//! var p:pointer; {|ord_noad| for new box in math mode}
//! begin if box_context<box_flag then @<Append box |cur_box| to the current list,
//!     shifted by |box_context|@>
//! else if box_context<ship_out_flag then @<Store \(c)|cur_box| in a box register@>
//! else if cur_box<>null then
//!   if box_context>ship_out_flag then @<Append a new leader node that
//!       uses |cur_box|@>
//!   else ship_out(cur_box);
//! end;
//!
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
