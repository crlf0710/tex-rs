//! @ @<Clear dimensions to zero@>=
//! d:=0; x:=0;
//! total_stretch[normal]:=0; total_shrink[normal]:=0;
//! total_stretch[fil]:=0; total_shrink[fil]:=0;
//! total_stretch[fill]:=0; total_shrink[fill]:=0;
//! total_stretch[filll]:=0; total_shrink[filll]:=0
//!
//! @ @<Examine node |p| in the hlist, taking account of its effect...@>=
//! @^inner loop@>
//! begin reswitch: while is_char_node(p) do
//!   @<Incorporate character dimensions into the dimensions of
//!     the hbox that will contain~it, then move to the next node@>;
//! if p<>null then
//!   begin case type(p) of
//!   hlist_node,vlist_node,rule_node,unset_node:
//!     @<Incorporate box dimensions into the dimensions of
//!       the hbox that will contain~it@>;
//!   ins_node,mark_node,adjust_node: if adjust_tail<>null then
//!     @<Transfer node |p| to the adjustment list@>;
//!   whatsit_node:@<Incorporate a whatsit node into an hbox@>;
//!   glue_node:@<Incorporate glue into the horizontal totals@>;
//!   kern_node,math_node: x:=x+width(p);
//!   ligature_node: @<Make node |p| look like a |char_node|
//!     and |goto reswitch|@>;
//!   othercases do_nothing
//!   endcases;@/
//!   p:=link(p);
//!   end;
//! end
//!
//!
//! @ @<Make node |p| look like a |char_node| and |goto reswitch|@>=
//! begin mem[lig_trick]:=mem[lig_char(p)]; link(lig_trick):=link(p);
//! p:=lig_trick; goto reswitch;
//! end
//!
//! @ The code here implicitly uses the fact that running dimensions are
//! indicated by |null_flag|, which will be ignored in the calculations
//! because it is a highly negative number.
//!
//! @<Incorporate box dimensions into the dimensions of the hbox...@>=
//! begin x:=x+width(p);
//! if type(p)>=rule_node then s:=0 @+else s:=shift_amount(p);
//! if height(p)-s>h then h:=height(p)-s;
//! if depth(p)+s>d then d:=depth(p)+s;
//! end
//!
//! @ The following code is part of \TeX's inner loop; i.e., adding another
//! character of text to the user's input will cause each of these instructions
//! to be exercised one more time.
//! @^inner loop@>
//!
//! @<Incorporate character dimensions into the dimensions of the hbox...@>=
//! begin f:=font(p); i:=char_info(f)(character(p)); hd:=height_depth(i);
//! x:=x+char_width(f)(i);@/
//! s:=char_height(f)(hd);@+if s>h then h:=s;
//! s:=char_depth(f)(hd);@+if s>d then d:=s;
//! p:=link(p);
//! end
//!
//! @ Although node |q| is not necessarily the immediate predecessor of node |p|,
//! it always points to some node in the list preceding |p|. Thus, we can delete
//! nodes by moving |q| when necessary. The algorithm takes linear time, and the
//! extra computation does not intrude on the inner loop unless it is necessary
//! to make a deletion.
//! @^inner loop@>
//!
//! @<Transfer node |p| to the adjustment list@>=
//! begin while link(q)<>p do q:=link(q);
//! if type(p)=adjust_node then
//!   begin link(adjust_tail):=adjust_ptr(p);
//!   while link(adjust_tail)<>null do adjust_tail:=link(adjust_tail);
//!   p:=link(p); free_node(link(q),small_node_size);
//!   end
//! else  begin link(adjust_tail):=p; adjust_tail:=p; p:=link(p);
//!   end;
//! link(q):=p; p:=q;
//! end
//!
//! @ @<Incorporate glue into the horizontal totals@>=
//! begin g:=glue_ptr(p); x:=x+width(g);@/
//! o:=stretch_order(g); total_stretch[o]:=total_stretch[o]+stretch(g);
//! o:=shrink_order(g); total_shrink[o]:=total_shrink[o]+shrink(g);
//! if subtype(p)>=a_leaders then
//!   begin g:=leader_ptr(p);
//!   if height(g)>h then h:=height(g);
//!   if depth(g)>d then d:=depth(g);
//!   end;
//! end
//!
//! @ When we get to the present part of the program, |x| is the natural width
//! of the box being packaged.
//!
//! @<Determine the value of |width(r)| and the appropriate glue setting...@>=
//! if m=additional then w:=x+w;
//! width(r):=w; x:=w-x; {now |x| is the excess to be made up}
//! if x=0 then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   return;
//!   end
//! else if x>0 then @<Determine horizontal glue stretch setting, then |return|
//!     or \hbox{|goto common_ending|}@>
//! else @<Determine horizontal glue shrink setting, then |return|
//!     or \hbox{|goto common_ending|}@>
//!
//! @ @<Determine horizontal glue stretch setting...@>=
//! begin @<Determine the stretch order@>;
//! glue_order(r):=o; glue_sign(r):=stretching;
//! if total_stretch[o]<>0 then glue_set(r):=unfloat(x/total_stretch[o])
//! @^real division@>
//! else  begin glue_sign(r):=normal;
//!   set_glue_ratio_zero(glue_set(r)); {there's nothing to stretch}
//!   end;
//! if o=normal then if list_ptr(r)<>null then
//!   @<Report an underfull hbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//! return;
//! end
//!
//! @ @<Determine the stretch order@>=
//! if total_stretch[filll]<>0 then o:=filll
//! else if total_stretch[fill]<>0 then o:=fill
//! else if total_stretch[fil]<>0 then o:=fil
//! else o:=normal
//!
//! @ @<Report an underfull hbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(x,total_stretch[normal]);
//! if last_badness>hbadness then
//!   begin print_ln;
//!   if last_badness>100 then print_nl("Underfull")@+else print_nl("Loose");
//!   print(" \hbox (badness "); print_int(last_badness);
//! @.Underfull \\hbox...@>
//! @.Loose \\hbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
