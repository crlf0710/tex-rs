//! @ @<Determine horizontal glue shrink setting...@>=
//! begin @<Determine the shrink order@>;
//! glue_order(r):=o; glue_sign(r):=shrinking;
//! if total_shrink[o]<>0 then glue_set(r):=unfloat((-x)/total_shrink[o])
//! @^real division@>
//! else  begin glue_sign(r):=normal;
//!   set_glue_ratio_zero(glue_set(r)); {there's nothing to shrink}
//!   end;
//! if (total_shrink[o]<-x)and(o=normal)and(list_ptr(r)<>null) then
//!   begin last_badness:=1000000;
//!   set_glue_ratio_one(glue_set(r)); {use the maximum shrinkage}
//!   @<Report an overfull hbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//!   end
//! else if o=normal then if list_ptr(r)<>null then
//!   @<Report a tight hbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//! return;
//! end
//!
//! @ @<Determine the shrink order@>=
//! if total_shrink[filll]<>0 then o:=filll
//! else if total_shrink[fill]<>0 then o:=fill
//! else if total_shrink[fil]<>0 then o:=fil
//! else o:=normal
//!
//! @ @<Report an overfull hbox and |goto common_ending|, if...@>=
//! if (-x-total_shrink[normal]>hfuzz)or(hbadness<100) then
//!   begin if (overfull_rule>0)and(-x-total_shrink[normal]>hfuzz) then
//!     begin while link(q)<>null do q:=link(q);
//!     link(q):=new_rule;
//!     width(link(q)):=overfull_rule;
//!     end;
//!   print_ln; print_nl("Overfull \hbox (");
//! @.Overfull \\hbox...@>
//!   print_scaled(-x-total_shrink[normal]); print("pt too wide");
//!   goto common_ending;
//!   end
//!
//! @ @<Report a tight hbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(-x,total_shrink[normal]);
//! if last_badness>hbadness then
//!   begin print_ln; print_nl("Tight \hbox (badness "); print_int(last_badness);
//! @.Tight \\hbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
//! @ The |vpack| subroutine is actually a special case of a slightly more
//! general routine called |vpackage|, which has four parameters. The fourth
//! parameter, which is |max_dimen| in the case of |vpack|, specifies the
//! maximum depth of the page box that is constructed. The depth is first
//! computed by the normal rules; if it exceeds this limit, the reference
//! point is simply moved down until the limiting depth is attained.
//!
//! @d vpack(#)==vpackage(#,max_dimen) {special case of unconstrained depth}
//!
//! @p function vpackage(@!p:pointer;@!h:scaled;@!m:small_number;@!l:scaled):
//!   pointer;
//! label common_ending, exit;
//! var r:pointer; {the box node that will be returned}
//! @!w,@!d,@!x:scaled; {width, depth, and natural height}
//! @!s:scaled; {shift amount}
//! @!g:pointer; {points to a glue specification}
//! @!o:glue_ord; {order of infinity}
//! begin last_badness:=0; r:=get_node(box_node_size); type(r):=vlist_node;
//! subtype(r):=min_quarterword; shift_amount(r):=0;
//! list_ptr(r):=p;@/
//! w:=0; @<Clear dimensions to zero@>;
//! while p<>null do @<Examine node |p| in the vlist, taking account of its effect
//!   on the dimensions of the new box; then advance |p| to the next node@>;
//! width(r):=w;
//! if d>l then
//!   begin x:=x+d-l; depth(r):=l;
//!   end
//! else depth(r):=d;
//! @<Determine the value of |height(r)| and the appropriate glue setting;
//!   then |return| or |goto common_ending|@>;
//! common_ending: @<Finish issuing a diagnostic message
//!       for an overfull or underfull vbox@>;
//! exit: vpackage:=r;
//! end;
//!
//! @ @<Examine node |p| in the vlist, taking account of its effect...@>=
//! begin if is_char_node(p) then confusion("vpack")
//! @:this can't happen vpack}{\quad vpack@>
//! else  case type(p) of
//!   hlist_node,vlist_node,rule_node,unset_node:
//!     @<Incorporate box dimensions into the dimensions of
//!       the vbox that will contain~it@>;
//!   whatsit_node:@<Incorporate a whatsit node into a vbox@>;
//!   glue_node: @<Incorporate glue into the vertical totals@>;
//!   kern_node: begin x:=x+d+width(p); d:=0;
//!     end;
//!   othercases do_nothing
//!   endcases;
//! p:=link(p);
//! end
//!
//! @ @<Incorporate box dimensions into the dimensions of the vbox...@>=
//! begin x:=x+d+height(p); d:=depth(p);
//! if type(p)>=rule_node then s:=0 @+else s:=shift_amount(p);
//! if width(p)+s>w then w:=width(p)+s;
//! end
//!
//! @ @<Incorporate glue into the vertical totals@>=
//! begin x:=x+d; d:=0;@/
//! g:=glue_ptr(p); x:=x+width(g);@/
//! o:=stretch_order(g); total_stretch[o]:=total_stretch[o]+stretch(g);
//! o:=shrink_order(g); total_shrink[o]:=total_shrink[o]+shrink(g);
//! if subtype(p)>=a_leaders then
//!   begin g:=leader_ptr(p);
//!   if width(g)>w then w:=width(g);
//!   end;
//! end
//!
//! @ When we get to the present part of the program, |x| is the natural height
//! of the box being packaged.
//!
//! @<Determine the value of |height(r)| and the appropriate glue setting...@>=
//! if m=additional then h:=x+h;
//! height(r):=h; x:=h-x; {now |x| is the excess to be made up}
//! if x=0 then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   return;
//!   end
//! else if x>0 then @<Determine vertical glue stretch setting, then |return|
//!     or \hbox{|goto common_ending|}@>
//! else @<Determine vertical glue shrink setting, then |return|
//!     or \hbox{|goto common_ending|}@>
//!
//! @ @<Determine vertical glue stretch setting...@>=
//! begin @<Determine the stretch order@>;
//! glue_order(r):=o; glue_sign(r):=stretching;
//! if total_stretch[o]<>0 then glue_set(r):=unfloat(x/total_stretch[o])
//! @^real division@>
//! else  begin glue_sign(r):=normal;
//!   set_glue_ratio_zero(glue_set(r)); {there's nothing to stretch}
//!   end;
//! if o=normal then if list_ptr(r)<>null then
//!   @<Report an underfull vbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//! return;
//! end
//!
//! @ @<Report an underfull vbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(x,total_stretch[normal]);
//! if last_badness>vbadness then
//!   begin print_ln;
//!   if last_badness>100 then print_nl("Underfull")@+else print_nl("Loose");
//!   print(" \vbox (badness "); print_int(last_badness);
//! @.Underfull \\vbox...@>
//! @.Loose \\vbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
//! @ @<Finish issuing a diagnostic message for an overfull or underfull vbox@>=
//! if output_active then print(") has occurred while \output is active")
//! else  begin if pack_begin_line<>0 then {it's actually negative}
//!     begin print(") in alignment at lines ");
//!     print_int(abs(pack_begin_line));
//!     print("--");
//!     end
//!   else print(") detected at line ");
//!   print_int(line);
//!   print_ln;@/
//!   end;
//! begin_diagnostic; show_box(r); end_diagnostic(true)
//!
//! @ @<Determine vertical glue shrink setting...@>=
//! begin @<Determine the shrink order@>;
//! glue_order(r):=o; glue_sign(r):=shrinking;
//! if total_shrink[o]<>0 then glue_set(r):=unfloat((-x)/total_shrink[o])
//! @^real division@>
//! else  begin glue_sign(r):=normal;
//!   set_glue_ratio_zero(glue_set(r)); {there's nothing to shrink}
//!   end;
//! if (total_shrink[o]<-x)and(o=normal)and(list_ptr(r)<>null) then
//!   begin last_badness:=1000000;
//!   set_glue_ratio_one(glue_set(r)); {use the maximum shrinkage}
//!   @<Report an overfull vbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//!   end
//! else if o=normal then if list_ptr(r)<>null then
//!   @<Report a tight vbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//! return;
//! end
//!
//! @ @<Report an overfull vbox and |goto common_ending|, if...@>=
//! if (-x-total_shrink[normal]>vfuzz)or(vbadness<100) then
//!   begin print_ln; print_nl("Overfull \vbox (");
//! @.Overfull \\vbox...@>
//!   print_scaled(-x-total_shrink[normal]); print("pt too high");
//!   goto common_ending;
//!   end
//!
//! @ @<Report a tight vbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(-x,total_shrink[normal]);
//! if last_badness>vbadness then
//!   begin print_ln; print_nl("Tight \vbox (badness "); print_int(last_badness);
//! @.Tight \\vbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
//! @ When a box is being appended to the current vertical list, the
//! baselineskip calculation is handled by the |append_to_vlist| routine.
//!
//! @p procedure append_to_vlist(@!b:pointer);
//! var d:scaled; {deficiency of space between baselines}
//! @!p:pointer; {a new glue node}
//! begin if prev_depth>ignore_depth then
//!   begin d:=width(baseline_skip)-prev_depth-height(b);
//!   if d<line_skip_limit then p:=new_param_glue(line_skip_code)
//!   else  begin p:=new_skip_param(baseline_skip_code);
//!     width(temp_ptr):=d; {|temp_ptr=glue_ptr(p)|}
//!     end;
//!   link(tail):=p; tail:=p;
//!   end;
//! link(tail):=b; tail:=b; prev_depth:=depth(b);
//! end;
//!
