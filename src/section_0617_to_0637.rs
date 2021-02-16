//! @ @<Initialize variables as |ship_out| begins@>=
//! dvi_h:=0; dvi_v:=0; cur_h:=h_offset; dvi_f:=null_font;
//! ensure_dvi_open;
//! if total_pages=0 then
//!   begin dvi_out(pre); dvi_out(id_byte); {output the preamble}
//! @^preamble of \.{DVI} file@>
//!   dvi_four(25400000); dvi_four(473628672); {conversion ratio for sp}
//!   prepare_mag; dvi_four(mag); {magnification factor is frozen}
//!   old_setting:=selector; selector:=new_string;
//!   print(" TeX output "); print_int(year); print_char(".");
//!   print_two(month); print_char("."); print_two(day);
//!   print_char(":"); print_two(time div 60);
//!   print_two(time mod 60);
//!   selector:=old_setting; dvi_out(cur_length);
//!   for s:=str_start[str_ptr] to pool_ptr-1 do dvi_out(so(str_pool[s]));
//!   pool_ptr:=str_start[str_ptr]; {flush the current string}
//!   end
//!
//! @ When |hlist_out| is called, its duty is to output the box represented
//! by the |hlist_node| pointed to by |temp_ptr|. The reference point of that
//! box has coordinates |(cur_h,cur_v)|.
//!
//! Similarly, when |vlist_out| is called, its duty is to output the box represented
//! by the |vlist_node| pointed to by |temp_ptr|. The reference point of that
//! box has coordinates |(cur_h,cur_v)|.
//! @^recursion@>
//!
//! @p procedure@?vlist_out; forward; {|hlist_out| and |vlist_out| are mutually
//!   recursive}
//!
//! @ The recursive procedures |hlist_out| and |vlist_out| each have local variables
//! |save_h| and |save_v| to hold the values of |dvi_h| and |dvi_v| just before
//! entering a new level of recursion.  In effect, the values of |save_h| and
//! |save_v| on \TeX's run-time stack correspond to the values of |h| and |v|
//! that a \.{DVI}-reading program will push onto its coordinate stack.
//!
//! @d move_past=13 {go to this label when advancing past glue or a rule}
//! @d fin_rule=14 {go to this label to finish processing a rule}
//! @d next_p=15 {go to this label when finished with node |p|}
//!
//! @p @t\4@>@<Declare procedures needed in |hlist_out|, |vlist_out|@>@t@>@/
//! procedure hlist_out; {output an |hlist_node| box}
//! label reswitch, move_past, fin_rule, next_p;
//! var base_line: scaled; {the baseline coordinate for this box}
//! @!left_edge: scaled; {the left coordinate for this box}
//! @!save_h,@!save_v: scaled; {what |dvi_h| and |dvi_v| should pop to}
//! @!this_box: pointer; {pointer to containing box}
//! @!g_order: glue_ord; {applicable order of infinity for glue}
//! @!g_sign: normal..shrinking; {selects type of glue}
//! @!p:pointer; {current position in the hlist}
//! @!save_loc:integer; {\.{DVI} byte location upon entry}
//! @!leader_box:pointer; {the leader box being replicated}
//! @!leader_wd:scaled; {width of leader box being replicated}
//! @!lx:scaled; {extra space between leader boxes}
//! @!outer_doing_leaders:boolean; {were we doing leaders?}
//! @!edge:scaled; {left edge of sub-box, or right edge of leader space}
//! @!glue_temp:real; {glue value before rounding}
//! @!cur_glue:real; {glue seen so far}
//! @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
//! begin cur_g:=0; cur_glue:=float_constant(0);
//! this_box:=temp_ptr; g_order:=glue_order(this_box);
//! g_sign:=glue_sign(this_box); p:=list_ptr(this_box);
//! incr(cur_s);
//! if cur_s>0 then dvi_out(push);
//! if cur_s>max_push then max_push:=cur_s;
//! save_loc:=dvi_offset+dvi_ptr; base_line:=cur_v; left_edge:=cur_h;
//! while p<>null do @<Output node |p| for |hlist_out| and move to the next node,
//!   maintaining the condition |cur_v=base_line|@>;
//! prune_movements(save_loc);
//! if cur_s>0 then dvi_pop(save_loc);
//! decr(cur_s);
//! end;
//!
//! @ We ought to give special care to the efficiency of one part of |hlist_out|,
//! since it belongs to \TeX's inner loop. When a |char_node| is encountered,
//! we save a little time by processing several nodes in succession until
//! reaching a non-|char_node|. The program uses the fact that |set_char_0=0|.
//! @^inner loop@>
//!
//! @<Output node |p| for |hlist_out|...@>=
//! reswitch: if is_char_node(p) then
//!   begin synch_h; synch_v;
//!   repeat f:=font(p); c:=character(p);
//!   if f<>dvi_f then @<Change font |dvi_f| to |f|@>;
//!   if c>=qi(128) then dvi_out(set1);
//!   dvi_out(qo(c));@/
//!   cur_h:=cur_h+char_width(f)(char_info(f)(c));
//!   p:=link(p);
//!   until not is_char_node(p);
//!   dvi_h:=cur_h;
//!   end
//! else @<Output the non-|char_node| |p| for |hlist_out|
//!     and move to the next node@>
//!
//! @ @<Change font |dvi_f| to |f|@>=
//! begin if not font_used[f] then
//!   begin dvi_font_def(f); font_used[f]:=true;
//!   end;
//! if f<=64+font_base then dvi_out(f-font_base-1+fnt_num_0)
//! else  begin dvi_out(fnt1); dvi_out(f-font_base-1);
//!   end;
//! dvi_f:=f;
//! end
//!
//! @ @<Output the non-|char_node| |p| for |hlist_out|...@>=
//! begin case type(p) of
//! hlist_node,vlist_node:@<Output a box in an hlist@>;
//! rule_node: begin rule_ht:=height(p); rule_dp:=depth(p); rule_wd:=width(p);
//!   goto fin_rule;
//!   end;
//! whatsit_node: @<Output the whatsit node |p| in an hlist@>;
//! glue_node: @<Move right or output leaders@>;
//! kern_node,math_node:cur_h:=cur_h+width(p);
//! ligature_node: @<Make node |p| look like a |char_node| and |goto reswitch|@>;
//! othercases do_nothing
//! endcases;@/
//! goto next_p;
//! fin_rule: @<Output a rule in an hlist@>;
//! move_past: cur_h:=cur_h+rule_wd;
//! next_p:p:=link(p);
//! end
//!
//! @ @<Output a box in an hlist@>=
//! if list_ptr(p)=null then cur_h:=cur_h+width(p)
//! else  begin save_h:=dvi_h; save_v:=dvi_v;
//!   cur_v:=base_line+shift_amount(p); {shift the box down}
//!   temp_ptr:=p; edge:=cur_h;
//!   if type(p)=vlist_node then vlist_out@+else hlist_out;
//!   dvi_h:=save_h; dvi_v:=save_v;
//!   cur_h:=edge+width(p); cur_v:=base_line;
//!   end
//!
//! @ @<Output a rule in an hlist@>=
//! if is_running(rule_ht) then rule_ht:=height(this_box);
//! if is_running(rule_dp) then rule_dp:=depth(this_box);
//! rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
//! if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
//!   begin synch_h; cur_v:=base_line+rule_dp; synch_v;
//!   dvi_out(set_rule); dvi_four(rule_ht); dvi_four(rule_wd);
//!   cur_v:=base_line; dvi_h:=dvi_h+rule_wd;
//!   end
//!
//! @ @d billion==float_constant(1000000000)
//! @d vet_glue(#)== glue_temp:=#;
//!   if glue_temp>billion then
//!            glue_temp:=billion
//!   else if glue_temp<-billion then
//!            glue_temp:=-billion
//!
//! @<Move right or output leaders@>=
//! begin g:=glue_ptr(p); rule_wd:=width(g)-cur_g;
//! if g_sign<>normal then
//!   begin if g_sign=stretching then
//!     begin if stretch_order(g)=g_order then
//!       begin cur_glue:=cur_glue+stretch(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//! @^real multiplication@>
//!       cur_g:=round(glue_temp);
//!       end;
//!     end
//!   else if shrink_order(g)=g_order then
//!       begin cur_glue:=cur_glue-shrink(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//!       cur_g:=round(glue_temp);
//!       end;
//!   end;
//! rule_wd:=rule_wd+cur_g;
//! if subtype(p)>=a_leaders then
//!   @<Output leaders in an hlist, |goto fin_rule| if a rule
//!     or to |next_p| if done@>;
//! goto move_past;
//! end
//!
//! @ @<Output leaders in an hlist...@>=
//! begin leader_box:=leader_ptr(p);
//! if type(leader_box)=rule_node then
//!   begin rule_ht:=height(leader_box); rule_dp:=depth(leader_box);
//!   goto fin_rule;
//!   end;
//! leader_wd:=width(leader_box);
//! if (leader_wd>0)and(rule_wd>0) then
//!   begin rule_wd:=rule_wd+10; {compensate for floating-point rounding}
//!   edge:=cur_h+rule_wd; lx:=0;
//!   @<Let |cur_h| be the position of the first box, and set |leader_wd+lx|
//!     to the spacing between corresponding parts of boxes@>;
//!   while cur_h+leader_wd<=edge do
//!     @<Output a leader box at |cur_h|,
//!       then advance |cur_h| by |leader_wd+lx|@>;
//!   cur_h:=edge-10; goto next_p;
//!   end;
//! end
//!
//! @ The calculations related to leaders require a bit of care. First, in the
//! case of |a_leaders| (aligned leaders), we want to move |cur_h| to
//! |left_edge| plus the smallest multiple of |leader_wd| for which the result
//! is not less than the current value of |cur_h|; i.e., |cur_h| should become
//! $|left_edge|+|leader_wd|\times\lceil
//! (|cur_h|-|left_edge|)/|leader_wd|\rceil$.  The program here should work in
//! all cases even though some implementations of \PASCAL\ give nonstandard
//! results for the |div| operation when |cur_h| is less than |left_edge|.
//!
//! In the case of |c_leaders| (centered leaders), we want to increase |cur_h|
//! by half of the excess space not occupied by the leaders; and in the
//! case of |x_leaders| (expanded leaders) we increase |cur_h|
//! by $1/(q+1)$ of this excess space, where $q$ is the number of times the
//! leader box will be replicated. Slight inaccuracies in the division might
//! accumulate; half of this rounding error is placed at each end of the leaders.
//!
//! @<Let |cur_h| be the position of the first box, ...@>=
//! if subtype(p)=a_leaders then
//!   begin save_h:=cur_h;
//!   cur_h:=left_edge+leader_wd*((cur_h-left_edge)@!div leader_wd);
//!   if cur_h<save_h then cur_h:=cur_h+leader_wd;
//!   end
//! else  begin lq:=rule_wd div leader_wd; {the number of box copies}
//!   lr:=rule_wd mod leader_wd; {the remaining space}
//!   if subtype(p)=c_leaders then cur_h:=cur_h+(lr div 2)
//!   else  begin lx:=lr div (lq+1);
//!     cur_h:=cur_h+((lr-(lq-1)*lx) div 2);
//!     end;
//!   end
//!
//! @ The `\\{synch}' operations here are intended to decrease the number of
//! bytes needed to specify horizontal and vertical motion in the \.{DVI} output.
//!
//! @<Output a leader box at |cur_h|, ...@>=
//! begin cur_v:=base_line+shift_amount(leader_box); synch_v; save_v:=dvi_v;@/
//! synch_h; save_h:=dvi_h; temp_ptr:=leader_box;
//! outer_doing_leaders:=doing_leaders; doing_leaders:=true;
//! if type(leader_box)=vlist_node then vlist_out@+else hlist_out;
//! doing_leaders:=outer_doing_leaders;
//! dvi_v:=save_v; dvi_h:=save_h; cur_v:=base_line;
//! cur_h:=save_h+leader_wd+lx;
//! end
//!
//! @ The |vlist_out| routine is similar to |hlist_out|, but a bit simpler.
//!
//! @p procedure vlist_out; {output a |vlist_node| box}
//! label move_past, fin_rule, next_p;
//! var left_edge: scaled; {the left coordinate for this box}
//! @!top_edge: scaled; {the top coordinate for this box}
//! @!save_h,@!save_v: scaled; {what |dvi_h| and |dvi_v| should pop to}
//! @!this_box: pointer; {pointer to containing box}
//! @!g_order: glue_ord; {applicable order of infinity for glue}
//! @!g_sign: normal..shrinking; {selects type of glue}
//! @!p:pointer; {current position in the vlist}
//! @!save_loc:integer; {\.{DVI} byte location upon entry}
//! @!leader_box:pointer; {the leader box being replicated}
//! @!leader_ht:scaled; {height of leader box being replicated}
//! @!lx:scaled; {extra space between leader boxes}
//! @!outer_doing_leaders:boolean; {were we doing leaders?}
//! @!edge:scaled; {bottom boundary of leader space}
//! @!glue_temp:real; {glue value before rounding}
//! @!cur_glue:real; {glue seen so far}
//! @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
//! begin cur_g:=0; cur_glue:=float_constant(0);
//! this_box:=temp_ptr; g_order:=glue_order(this_box);
//! g_sign:=glue_sign(this_box); p:=list_ptr(this_box);
//! incr(cur_s);
//! if cur_s>0 then dvi_out(push);
//! if cur_s>max_push then max_push:=cur_s;
//! save_loc:=dvi_offset+dvi_ptr; left_edge:=cur_h; cur_v:=cur_v-height(this_box);
//! top_edge:=cur_v;
//! while p<>null do @<Output node |p| for |vlist_out| and move to the next node,
//!   maintaining the condition |cur_h=left_edge|@>;
//! prune_movements(save_loc);
//! if cur_s>0 then dvi_pop(save_loc);
//! decr(cur_s);
//! end;
//!
//! @ @<Output node |p| for |vlist_out|...@>=
//! begin if is_char_node(p) then confusion("vlistout")
//! @:this can't happen vlistout}{\quad vlistout@>
//! else @<Output the non-|char_node| |p| for |vlist_out|@>;
//! next_p:p:=link(p);
//! end
//!
//! @ @<Output the non-|char_node| |p| for |vlist_out|@>=
//! begin case type(p) of
//! hlist_node,vlist_node:@<Output a box in a vlist@>;
//! rule_node: begin rule_ht:=height(p); rule_dp:=depth(p); rule_wd:=width(p);
//!   goto fin_rule;
//!   end;
//! whatsit_node: @<Output the whatsit node |p| in a vlist@>;
//! glue_node: @<Move down or output leaders@>;
//! kern_node:cur_v:=cur_v+width(p);
//! othercases do_nothing
//! endcases;@/
//! goto next_p;
//! fin_rule: @<Output a rule in a vlist, |goto next_p|@>;
//! move_past: cur_v:=cur_v+rule_ht;
//! end
//!
//! @ The |synch_v| here allows the \.{DVI} output to use one-byte commands
//! for adjusting |v| in most cases, since the baselineskip distance will
//! usually be constant.
//!
//! @<Output a box in a vlist@>=
//! if list_ptr(p)=null then cur_v:=cur_v+height(p)+depth(p)
//! else  begin cur_v:=cur_v+height(p); synch_v;
//!   save_h:=dvi_h; save_v:=dvi_v;
//!   cur_h:=left_edge+shift_amount(p); {shift the box right}
//!   temp_ptr:=p;
//!   if type(p)=vlist_node then vlist_out@+else hlist_out;
//!   dvi_h:=save_h; dvi_v:=save_v;
//!   cur_v:=save_v+depth(p); cur_h:=left_edge;
//!   end
//!
//! @ @<Output a rule in a vlist...@>=
//! if is_running(rule_wd) then rule_wd:=width(this_box);
//! rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
//! cur_v:=cur_v+rule_ht;
//! if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
//!   begin synch_h; synch_v;
//!   dvi_out(put_rule); dvi_four(rule_ht); dvi_four(rule_wd);
//!   end;
//! goto next_p
//!
//! @ @<Move down or output leaders@>=
//! begin g:=glue_ptr(p); rule_ht:=width(g)-cur_g;
//! if g_sign<>normal then
//!   begin if g_sign=stretching then
//!     begin if stretch_order(g)=g_order then
//!       begin cur_glue:=cur_glue+stretch(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//! @^real multiplication@>
//!       cur_g:=round(glue_temp);
//!       end;
//!     end
//!   else if shrink_order(g)=g_order then
//!       begin cur_glue:=cur_glue-shrink(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//!       cur_g:=round(glue_temp);
//!       end;
//!   end;
//! rule_ht:=rule_ht+cur_g;
//! if subtype(p)>=a_leaders then
//!   @<Output leaders in a vlist, |goto fin_rule| if a rule
//!     or to |next_p| if done@>;
//! goto move_past;
//! end
//!
//! @ @<Output leaders in a vlist...@>=
//! begin leader_box:=leader_ptr(p);
//! if type(leader_box)=rule_node then
//!   begin rule_wd:=width(leader_box); rule_dp:=0;
//!   goto fin_rule;
//!   end;
//! leader_ht:=height(leader_box)+depth(leader_box);
//! if (leader_ht>0)and(rule_ht>0) then
//!   begin rule_ht:=rule_ht+10; {compensate for floating-point rounding}
//!   edge:=cur_v+rule_ht; lx:=0;
//!   @<Let |cur_v| be the position of the first box, and set |leader_ht+lx|
//!     to the spacing between corresponding parts of boxes@>;
//!   while cur_v+leader_ht<=edge do
//!     @<Output a leader box at |cur_v|,
//!       then advance |cur_v| by |leader_ht+lx|@>;
//!   cur_v:=edge-10; goto next_p;
//!   end;
//! end
//!
//! @ @<Let |cur_v| be the position of the first box, ...@>=
//! if subtype(p)=a_leaders then
//!   begin save_v:=cur_v;
//!   cur_v:=top_edge+leader_ht*((cur_v-top_edge)@!div leader_ht);
//!   if cur_v<save_v then cur_v:=cur_v+leader_ht;
//!   end
//! else  begin lq:=rule_ht div leader_ht; {the number of box copies}
//!   lr:=rule_ht mod leader_ht; {the remaining space}
//!   if subtype(p)=c_leaders then cur_v:=cur_v+(lr div 2)
//!   else  begin lx:=lr div (lq+1);
//!     cur_v:=cur_v+((lr-(lq-1)*lx) div 2);
//!     end;
//!   end
//!
//! @ When we reach this part of the program, |cur_v| indicates the top of a
//! leader box, not its baseline.
//!
//! @<Output a leader box at |cur_v|, ...@>=
//! begin cur_h:=left_edge+shift_amount(leader_box); synch_h; save_h:=dvi_h;@/
//! cur_v:=cur_v+height(leader_box); synch_v; save_v:=dvi_v;
//! temp_ptr:=leader_box;
//! outer_doing_leaders:=doing_leaders; doing_leaders:=true;
//! if type(leader_box)=vlist_node then vlist_out@+else hlist_out;
//! doing_leaders:=outer_doing_leaders;
//! dvi_v:=save_v; dvi_h:=save_h; cur_h:=left_edge;
//! cur_v:=save_v-height(leader_box)+leader_ht+lx;
//! end
//!
