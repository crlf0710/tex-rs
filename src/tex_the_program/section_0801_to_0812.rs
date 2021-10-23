//! @ It's time now to dismantle the preamble list and to compute the column
//! widths. Let $w_{ij}$ be the maximum of the natural widths of all entries
//! that span columns $i$ through $j$, inclusive. The alignrecord for column~$i$
//! contains $w_{ii}$ in its |width| field, and there is also a linked list of
//! the nonzero $w_{ij}$ for increasing $j$, accessible via the |info| field;
//! these span nodes contain the value $j-i+|min_quarterword|$ in their
//! |link| fields. The values of $w_{ii}$ were initialized to |null_flag|, which
//! we regard as $-\infty$.
//!
//! The final column widths are defined by the formula
//! $$w_j=\max_{1\L i\L j}\biggl( w_{ij}-\sum_{i\L k<j}(t_k+w_k)\biggr),$$
//! where $t_k$ is the natural width of the tabskip glue between columns
//! $k$ and~$k+1$. However, if $w_{ij}=-\infty$ for all |i| in the range
//! |1<=i<=j| (i.e., if every entry that involved column~|j| also involved
//! column~|j+1|), we let $w_j=0$, and we zero out the tabskip glue after
//! column~|j|.
//!
//! \TeX\ computes these values by using the following scheme: First $w_1=w_{11}$.
//! Then replace $w_{2j}$ by $\max(w_{2j},w_{1j}-t_1-w_1)$, for all $j>1$.
//! Then $w_2=w_{22}$. Then replace $w_{3j}$ by $\max(w_{3j},w_{2j}-t_2-w_2)$
//! for all $j>2$; and so on. If any $w_j$ turns out to be $-\infty$, its
//! value is changed to zero and so is the next tabskip.
//!
//! @<Go through the preamble list,...@>=
//! q:=link(preamble);
//! repeat flush_list(u_part(q)); flush_list(v_part(q));
//! p:=link(link(q));
//! if width(q)=null_flag then
//!   @<Nullify |width(q)| and the tabskip glue following this column@>;
//! if info(q)<>end_span then
//!   @<Merge the widths in the span nodes of |q| with those of |p|,
//!     destroying the span nodes of |q|@>;
//! type(q):=unset_node; span_count(q):=min_quarterword; height(q):=0;
//! depth(q):=0; glue_order(q):=normal; glue_sign(q):=normal;
//! glue_stretch(q):=0; glue_shrink(q):=0; q:=p;
//! until q=null
//!
//! @ @<Nullify |width(q)| and the tabskip glue following this column@>=
//! begin width(q):=0; r:=link(q); s:=glue_ptr(r);
//! if s<>zero_glue then
//!   begin add_glue_ref(zero_glue); delete_glue_ref(s);
//!   glue_ptr(r):=zero_glue;
//!   end;
//! end
//!
//! @ Merging of two span-node lists is a typical exercise in the manipulation of
//! linearly linked data structures. The essential invariant in the following
//! |repeat| loop is that we want to dispense with node |r|, in |q|'s list,
//! and |u| is its successor; all nodes of |p|'s list up to and including |s|
//! have been processed, and the successor of |s| matches |r| or precedes |r|
//! or follows |r|, according as |link(r)=n| or |link(r)>n| or |link(r)<n|.
//!
//! @<Merge the widths...@>=
//! begin t:=width(q)+width(glue_ptr(link(q)));
//! r:=info(q); s:=end_span; info(s):=p; n:=min_quarterword+1;
//! repeat width(r):=width(r)-t; u:=info(r);
//! while link(r)>n do
//!   begin s:=info(s); n:=link(info(s))+1;
//!   end;
//! if link(r)<n then
//!   begin info(r):=info(s); info(s):=r; decr(link(r)); s:=r;
//!   end
//! else  begin if width(r)>width(info(s)) then width(info(s)):=width(r);
//!   free_node(r,span_node_size);
//!   end;
//! r:=u;
//! until r=end_span;
//! end
//!
//! @ Now the preamble list has been converted to a list of alternating unset
//! boxes and tabskip glue, where the box widths are equal to the final
//! column sizes. In case of \.{\\valign}, we change the widths to heights,
//! so that a correct error message will be produced if the alignment is
//! overfull or underfull.
//!
//! @<Package the preamble list...@>=
//! save_ptr:=save_ptr-2; pack_begin_line:=-mode_line;
//! if mode=-vmode then
//!   begin rule_save:=overfull_rule;
//!   overfull_rule:=0; {prevent rule from being packaged}
//!   p:=hpack(preamble,saved(1),saved(0)); overfull_rule:=rule_save;
//!   end
//! else  begin q:=link(preamble);
//!   repeat height(q):=width(q); width(q):=0; q:=link(link(q));
//!   until q=null;
//!   p:=vpack(preamble,saved(1),saved(0));
//!   q:=link(preamble);
//!   repeat width(q):=height(q); height(q):=0; q:=link(link(q));
//!   until q=null;
//!   end;
//! pack_begin_line:=0
//!
//! @ @<Set the glue in all the unset...@>=
//! q:=link(head); s:=head;
//! while q<>null do
//!   begin if not is_char_node(q) then
//!     if type(q)=unset_node then
//!       @<Set the unset box |q| and the unset boxes in it@>
//!     else if type(q)=rule_node then
//!       @<Make the running dimensions in rule |q| extend to the
//!         boundaries of the alignment@>;
//!   s:=q; q:=link(q);
//!   end
//!
//! @ @<Make the running dimensions in rule |q| extend...@>=
//! begin if is_running(width(q)) then width(q):=width(p);
//! if is_running(height(q)) then height(q):=height(p);
//! if is_running(depth(q)) then depth(q):=depth(p);
//! if o<>0 then
//!   begin r:=link(q); link(q):=null; q:=hpack(q,natural);
//!   shift_amount(q):=o; link(q):=r; link(s):=q;
//!   end;
//! end
//!
//! @ The unset box |q| represents a row that contains one or more unset boxes,
//! depending on how soon \.{\\cr} occurred in that row.
//!
//! @<Set the unset box |q| and the unset boxes in it@>=
//! begin if mode=-vmode then
//!   begin type(q):=hlist_node; width(q):=width(p);
//!   end
//! else  begin type(q):=vlist_node; height(q):=height(p);
//!   end;
//! glue_order(q):=glue_order(p); glue_sign(q):=glue_sign(p);
//! glue_set(q):=glue_set(p); shift_amount(q):=o;
//! r:=link(list_ptr(q)); s:=link(list_ptr(p));
//! repeat @<Set the glue in node |r| and change it from an unset node@>;
//! r:=link(link(r)); s:=link(link(s));
//! until r=null;
//! end
//!
//! @ A box made from spanned columns will be followed by tabskip glue nodes and
//! by empty boxes as if there were no spanning. This permits perfect alignment
//! of subsequent entries, and it prevents values that depend on floating point
//! arithmetic from entering into the dimensions of any boxes.
//!
//! @<Set the glue in node |r|...@>=
//! n:=span_count(r); t:=width(s); w:=t; u:=hold_head;
//! while n>min_quarterword do
//!   begin decr(n);
//!   @<Append tabskip glue and an empty box to list |u|,
//!     and update |s| and |t| as the prototype nodes are passed@>;
//!   end;
//! if mode=-vmode then
//!   @<Make the unset node |r| into an |hlist_node| of width |w|,
//!     setting the glue as if the width were |t|@>
//! else @<Make the unset node |r| into a |vlist_node| of height |w|,
//!     setting the glue as if the height were |t|@>;
//! shift_amount(r):=0;
//! if u<>hold_head then {append blank boxes to account for spanned nodes}
//!   begin link(u):=link(r); link(r):=link(hold_head); r:=u;
//!   end
//!
//! @ @<Append tabskip glue and an empty box to list |u|...@>=
//! s:=link(s); v:=glue_ptr(s); link(u):=new_glue(v); u:=link(u);
//! subtype(u):=tab_skip_code+1; t:=t+width(v);
//! if glue_sign(p)=stretching then
//!   begin if stretch_order(v)=glue_order(p) then
//!     t:=t+round(float(glue_set(p))*stretch(v));
//! @^real multiplication@>
//!   end
//! else if glue_sign(p)=shrinking then
//!   begin if shrink_order(v)=glue_order(p) then
//!     t:=t-round(float(glue_set(p))*shrink(v));
//!   end;
//! s:=link(s); link(u):=new_null_box; u:=link(u); t:=t+width(s);
//! if mode=-vmode then width(u):=width(s)@+else
//!   begin type(u):=vlist_node; height(u):=width(s);
//!   end
//!
//! @ @<Make the unset node |r| into an |hlist_node| of width |w|...@>=
//! begin height(r):=height(q); depth(r):=depth(q);
//! if t=width(r) then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   end
//! else if t>width(r) then
//!   begin glue_sign(r):=stretching;
//!   if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else glue_set(r):=unfloat((t-width(r))/glue_stretch(r));
//! @^real division@>
//!   end
//! else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
//!   if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else if (glue_order(r)=normal)and(width(r)-t>glue_shrink(r)) then
//!     set_glue_ratio_one(glue_set(r))
//!   else glue_set(r):=unfloat((width(r)-t)/glue_shrink(r));
//!   end;
//! width(r):=w; type(r):=hlist_node;
//! end
//!
//! @ @<Make the unset node |r| into a |vlist_node| of height |w|...@>=
//! begin width(r):=width(q);
//! if t=height(r) then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   end
//! else if t>height(r) then
//!   begin glue_sign(r):=stretching;
//!   if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else glue_set(r):=unfloat((t-height(r))/glue_stretch(r));
//! @^real division@>
//!   end
//! else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
//!   if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else if (glue_order(r)=normal)and(height(r)-t>glue_shrink(r)) then
//!     set_glue_ratio_one(glue_set(r))
//!   else glue_set(r):=unfloat((height(r)-t)/glue_shrink(r));
//!   end;
//! height(r):=w; type(r):=vlist_node;
//! end
//!
//! @ We now have a completed alignment, in the list that starts at |head|
//! and ends at |tail|. This list will be merged with the one that encloses
//! it. (In case the enclosing mode is |mmode|, for displayed formulas,
//! we will need to insert glue before and after the display; that part of the
//! program will be deferred until we're more familiar with such operations.)
//!
//! In restricted horizontal mode, the |clang| part of |aux| is undefined;
//! an over-cautious \PASCAL\ runtime system may complain about this.
//! @^dirty \PASCAL@>
//!
//! @<Insert the \(c)current list into its environment@>=
//! aux_save:=aux; p:=link(head); q:=tail; pop_nest;
//! if mode=mmode then @<Finish an alignment in a display@>
//! else  begin aux:=aux_save; link(tail):=p;
//!   if p<>null then tail:=q;
//!   if mode=vmode then build_page;
//!   end
//!
