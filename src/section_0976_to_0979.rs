//! @ Vertical lists that are subject to the |vert_break| procedure should not
//! contain infinite shrinkability, since that would permit any amount of
//! information to ``fit'' on one page.
//!
//! @<Update the current height and depth measurements with...@>=
//! if type(p)=kern_node then q:=p
//! else  begin q:=glue_ptr(p);
//!   active_height[2+stretch_order(q)]:=@|
//!     active_height[2+stretch_order(q)]+stretch(q);@/
//!   active_height[6]:=active_height[6]+shrink(q);
//!   if (shrink_order(q)<>normal)and(shrink(q)<>0) then
//!     begin@t@>@;@/
//!     print_err("Infinite glue shrinkage found in box being split");@/
//! @.Infinite glue shrinkage...@>
//!     help4("The box you are \vsplitting contains some infinitely")@/
//!       ("shrinkable glue, e.g., `\vss' or `\vskip 0pt minus 1fil'.")@/
//!       ("Such glue doesn't belong there; but you can safely proceed,")@/
//!       ("since the offensive shrinkability has been made finite.");
//!     error; r:=new_spec(q); shrink_order(r):=normal; delete_glue_ref(q);
//!     glue_ptr(p):=r; q:=r;
//!     end;
//!   end;
//! cur_height:=cur_height+prev_dp+width(q); prev_dp:=0
//!
//! @ Now we are ready to consider |vsplit| itself. Most of
//! its work is accomplished by the two subroutines that we have just considered.
//!
//! Given the number of a vlist box |n|, and given a desired page height |h|,
//! the |vsplit| function finds the best initial segment of the vlist and
//! returns a box for a page of height~|h|. The remainder of the vlist, if
//! any, replaces the original box, after removing glue and penalties and
//! adjusting for |split_top_skip|. Mark nodes in the split-off box are used to
//! set the values of |split_first_mark| and |split_bot_mark|; we use the
//! fact that |split_first_mark=null| if and only if |split_bot_mark=null|.
//!
//! The original box becomes ``void'' if and only if it has been entirely
//! extracted.  The extracted box is ``void'' if and only if the original
//! box was void (or if it was, erroneously, an hlist box).
//!
//! @p function vsplit(@!n:eight_bits; @!h:scaled):pointer;
//!   {extracts a page of height |h| from box |n|}
//! label exit,done;
//! var v:pointer; {the box to be split}
//! p:pointer; {runs through the vlist}
//! q:pointer; {points to where the break occurs}
//! begin v:=box(n);
//! if split_first_mark<>null then
//!   begin delete_token_ref(split_first_mark); split_first_mark:=null;
//!   delete_token_ref(split_bot_mark); split_bot_mark:=null;
//!   end;
//! @<Dispense with trivial cases of void or bad boxes@>;
//! q:=vert_break(list_ptr(v),h,split_max_depth);
//! @<Look at all the marks in nodes before the break, and set the final
//!   link to |null| at the break@>;
//! q:=prune_page_top(q); p:=list_ptr(v); free_node(v,box_node_size);
//! if q=null then box(n):=null {the |eq_level| of the box stays the same}
//! else box(n):=vpack(q,natural);
//! vsplit:=vpackage(p,h,exactly,split_max_depth);
//! exit: end;
//!
//! @ @<Dispense with trivial cases of void or bad boxes@>=
//! if v=null then
//!   begin vsplit:=null; return;
//!   end;
//! if type(v)<>vlist_node then
//!   begin print_err(""); print_esc("vsplit"); print(" needs a ");
//!   print_esc("vbox");
//! @:vsplit_}{\.{\\vsplit needs a \\vbox}@>
//!   help2("The box you are trying to split is an \hbox.")@/
//!   ("I can't split such a box, so I'll leave it alone.");
//!   error; vsplit:=null; return;
//!   end
//!
//! @ It's possible that the box begins with a penalty node that is the
//! ``best'' break, so we must be careful to handle this special case correctly.
//!
//! @<Look at all the marks...@>=
//! p:=list_ptr(v);
//! if p=q then list_ptr(v):=null
//! else loop@+begin if type(p)=mark_node then
//!     if split_first_mark=null then
//!       begin split_first_mark:=mark_ptr(p);
//!       split_bot_mark:=split_first_mark;
//!       token_ref_count(split_first_mark):=@|
//!         token_ref_count(split_first_mark)+2;
//!       end
//!     else  begin delete_token_ref(split_bot_mark);
//!       split_bot_mark:=mark_ptr(p);
//!       add_token_ref(split_bot_mark);
//!       end;
//!   if link(p)=q then
//!     begin link(p):=null; goto done;
//!     end;
//!   p:=link(p);
//!   end;
//! done:
//!
