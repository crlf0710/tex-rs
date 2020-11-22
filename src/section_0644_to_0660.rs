//! @* \[33] Packaging.
//! We're essentially done with the parts of \TeX\ that are concerned with
//! the input (|get_next|) and the output (|ship_out|). So it's time to
//! get heavily into the remaining part, which does the real work of typesetting.
//!
//! After lists are constructed, \TeX\ wraps them up and puts them into boxes.
//! Two major subroutines are given the responsibility for this task: |hpack|
//! applies to horizontal lists (hlists) and |vpack| applies to vertical lists
//! (vlists). The main duty of |hpack| and |vpack| is to compute the dimensions
//! of the resulting boxes, and to adjust the glue if one of those dimensions
//! is pre-specified. The computed sizes normally enclose all of the material
//! inside the new box; but some items may stick out if negative glue is used,
//! if the box is overfull, or if a \.{\\vbox} includes other boxes that have
//! been shifted left.
//!
//! The subroutine call |hpack(p,w,m)| returns a pointer to an |hlist_node|
//! for a box containing the hlist that starts at |p|. Parameter |w| specifies
//! a width; and parameter |m| is either `|exactly|' or `|additional|'.  Thus,
//! |hpack(p,w,exactly)| produces a box whose width is exactly |w|, while
//! |hpack(p,w,additional)| yields a box whose width is the natural width plus
//! |w|.  It is convenient to define a macro called `|natural|' to cover the
//! most common case, so that we can say |hpack(p,natural)| to get a box that
//! has the natural width of list |p|.
//!
//! Similarly, |vpack(p,w,m)| returns a pointer to a |vlist_node| for a
//! box containing the vlist that starts at |p|. In this case |w| represents
//! a height instead of a width; the parameter |m| is interpreted as in |hpack|.
//!
//! @d exactly=0 {a box dimension is pre-specified}
//! @d additional=1 {a box dimension is increased from the natural one}
//! @d natural==0,additional {shorthand for parameters to |hpack| and |vpack|}
//!
//! @ The parameters to |hpack| and |vpack| correspond to \TeX's primitives
//! like `\.{\\hbox} \.{to} \.{300pt}', `\.{\\hbox} \.{spread} \.{10pt}'; note
//! that `\.{\\hbox}' with no dimension following it is equivalent to
//! `\.{\\hbox} \.{spread} \.{0pt}'.  The |scan_spec| subroutine scans such
//! constructions in the user's input, including the mandatory left brace that
//! follows them, and it puts the specification onto |save_stack| so that the
//! desired box can later be obtained by executing the following code:
//! $$\vbox{\halign{#\hfil\cr
//! |save_ptr:=save_ptr-2;|\cr
//! |hpack(p,saved(1),saved(0)).|\cr}}$$
//! Special care is necessary to ensure that the special |save_stack| codes
//! are placed just below the new group code, because scanning can change
//! |save_stack| when \.{\\csname} appears.
//!
//! @p procedure scan_spec(@!c:group_code;@!three_codes:boolean);
//!   {scans a box specification and left brace}
//! label found;
//! var @!s:integer; {temporarily saved value}
//! @!spec_code:exactly..additional;
//! begin if three_codes then s:=saved(0);
//! if scan_keyword("to") then spec_code:=exactly
//! @.to@>
//! else if scan_keyword("spread") then spec_code:=additional
//! @.spread@>
//! else  begin spec_code:=additional; cur_val:=0;
//!   goto found;
//!   end;
//! scan_normal_dimen;
//! found: if three_codes then
//!   begin saved(0):=s; incr(save_ptr);
//!   end;
//! saved(0):=spec_code; saved(1):=cur_val; save_ptr:=save_ptr+2;
//! new_save_level(c); scan_left_brace;
//! end;
//!
//! @ To figure out the glue setting, |hpack| and |vpack| determine how much
//! stretchability and shrinkability are present, considering all four orders
//! of infinity. The highest order of infinity that has a nonzero coefficient
//! is then used as if no other orders were present.
//!
//! For example, suppose that the given list contains six glue nodes with
//! the respective stretchabilities 3pt, 8fill, 5fil, 6pt, $-3$fil, $-8$fill.
//! Then the total is essentially 2fil; and if a total additional space of 6pt
//! is to be achieved by stretching, the actual amounts of stretch will be
//! 0pt, 0pt, 15pt, 0pt, $-9$pt, and 0pt, since only `fil' glue will be
//! considered. (The `fill' glue is therefore not really stretching infinitely
//! with respect to `fil'; nobody would actually want that to happen.)
//!
//! The arrays |total_stretch| and |total_shrink| are used to determine how much
//! glue of each kind is present. A global variable |last_badness| is used
//! to implement \.{\\badness}.
//!
//! @<Glob...@>=
//! @!total_stretch, @!total_shrink: array[glue_ord] of scaled;
//!   {glue found by |hpack| or |vpack|}
//! @!last_badness:integer; {badness of the most recently packaged box}
//!
//! @ If the global variable |adjust_tail| is non-null, the |hpack| routine
//! also removes all occurrences of |ins_node|, |mark_node|, and |adjust_node|
//! items and appends the resulting material onto the list that ends at
//! location |adjust_tail|.
//!
//! @< Glob...@>=
//! @!adjust_tail:pointer; {tail of adjustment list}
//!
//! @ @<Set init...@>=adjust_tail:=null; last_badness:=0;
//!
//! @ Here now is |hpack|, which contains few if any surprises.
//!
//! @p function hpack(@!p:pointer;@!w:scaled;@!m:small_number):pointer;
//! label reswitch, common_ending, exit;
//! var r:pointer; {the box node that will be returned}
//! @!q:pointer; {trails behind |p|}
//! @!h,@!d,@!x:scaled; {height, depth, and natural width}
//! @!s:scaled; {shift amount}
//! @!g:pointer; {points to a glue specification}
//! @!o:glue_ord; {order of infinity}
//! @!f:internal_font_number; {the font in a |char_node|}
//! @!i:four_quarters; {font information about a |char_node|}
//! @!hd:eight_bits; {height and depth indices for a character}
//! begin last_badness:=0; r:=get_node(box_node_size); type(r):=hlist_node;
//! subtype(r):=min_quarterword; shift_amount(r):=0;
//! q:=r+list_offset; link(q):=p;@/
//! h:=0; @<Clear dimensions to zero@>;
//! while p<>null do @<Examine node |p| in the hlist, taking account of its effect
//!   on the dimensions of the new box, or moving it to the adjustment list;
//!   then advance |p| to the next node@>;
//! if adjust_tail<>null then link(adjust_tail):=null;
//! height(r):=h; depth(r):=d;@/
//! @<Determine the value of |width(r)| and the appropriate glue setting;
//!   then |return| or |goto common_ending|@>;
//! common_ending: @<Finish issuing a diagnostic message
//!       for an overfull or underfull hbox@>;
//! exit: hpack:=r;
//! end;
//!
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
