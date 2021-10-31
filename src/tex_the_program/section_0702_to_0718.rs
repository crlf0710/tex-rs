//! @ We also need to compute the change in style between mlists and their
//! subsidiaries. The following macros define the subsidiary style for
//! an overlined nucleus (|cramped_style|), for a subscript or a superscript
//! (|sub_style| or |sup_style|), or for a numerator or denominator (|num_style|
//! or |denom_style|).
//!
//! @d cramped_style(#)==2*(# div 2)+cramped {cramp the style}
//! @d sub_style(#)==2*(# div 4)+script_style+cramped {smaller and cramped}
//! @d sup_style(#)==2*(# div 4)+script_style+(# mod 2) {smaller}
//! @d num_style(#)==#+2-2*(# div 6) {smaller unless already script-script}
//! @d denom_style(#)==2*(# div 2)+cramped+2-2*(# div 6) {smaller, cramped}
//!
//! @ When the style changes, the following piece of program computes associated
//! information:
//!
//! @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>=
//! begin if cur_style<script_style then cur_size:=text_size
//! else cur_size:=16*((cur_style-text_style) div 2);
//! cur_mu:=x_over_n(math_quad(cur_size),18);
//! end
//!
//! @ Here is a function that returns a pointer to a rule node having a given
//! thickness |t|. The rule will extend horizontally to the boundary of the vlist
//! that eventually contains it.
//!
//! @p function fraction_rule(@!t:scaled):pointer;
//!   {construct the bar for a fraction}
//! var p:pointer; {the new node}
//! begin p:=new_rule; height(p):=t; depth(p):=0; fraction_rule:=p;
//! end;
//!
//! @ The |overbar| function returns a pointer to a vlist box that consists of
//! a given box |b|, above which has been placed a kern of height |k| under a
//! fraction rule of thickness |t| under additional space of height |t|.
//!
//! @p function overbar(@!b:pointer;@!k,@!t:scaled):pointer;
//! var p,@!q:pointer; {nodes being constructed}
//! begin p:=new_kern(k); link(p):=b; q:=fraction_rule(t); link(q):=p;
//! p:=new_kern(t); link(p):=q; overbar:=vpack(p,natural);
//! end;
//!
//! @ The |var_delimiter| function, which finds or constructs a sufficiently
//! large delimiter, is the most interesting of the auxiliary functions that
//! currently concern us. Given a pointer |d| to a delimiter field in some noad,
//! together with a size code |s| and a vertical distance |v|, this function
//! returns a pointer to a box that contains the smallest variant of |d| whose
//! height plus depth is |v| or more. (And if no variant is large enough, it
//! returns the largest available variant.) In particular, this routine will
//! construct arbitrarily large delimiters from extensible components, if
//! |d| leads to such characters.
//!
//! The value returned is a box whose |shift_amount| has been set so that
//! the box is vertically centered with respect to the axis in the given size.
//! If a built-up symbol is returned, the height of the box before shifting
//! will be the height of its topmost component.
//!
//! @p@t\4@>@<Declare subprocedures for |var_delimiter|@>
//! function var_delimiter(@!d:pointer;@!s:small_number;@!v:scaled):pointer;
//! label found,continue;
//! var b:pointer; {the box that will be constructed}
//! @!f,@!g: internal_font_number; {best-so-far and tentative font codes}
//! @!c,@!x,@!y: quarterword; {best-so-far and tentative character codes}
//! @!m,@!n: integer; {the number of extensible pieces}
//! @!u: scaled; {height-plus-depth of a tentative character}
//! @!w: scaled; {largest height-plus-depth so far}
//! @!q: four_quarters; {character info}
//! @!hd: eight_bits; {height-depth byte}
//! @!r: four_quarters; {extensible pieces}
//! @!z: small_number; {runs through font family members}
//! @!large_attempt: boolean; {are we trying the ``large'' variant?}
//! begin f:=null_font; w:=0; large_attempt:=false;
//! z:=small_fam(d); x:=small_char(d);
//! loop@+  begin @<Look at the variants of |(z,x)|; set |f| and |c| whenever
//!     a better character is found; |goto found| as soon as a
//!     large enough variant is encountered@>;
//!   if large_attempt then goto found; {there were none large enough}
//!   large_attempt:=true; z:=large_fam(d); x:=large_char(d);
//!   end;
//! found: if f<>null_font then
//!   @<Make variable |b| point to a box for |(f,c)|@>
//! else  begin b:=new_null_box;
//!   width(b):=null_delimiter_space; {use this width if no delimiter was found}
//!   end;
//! shift_amount(b):=half(height(b)-depth(b)) - axis_height(s);
//! var_delimiter:=b;
//! end;
//!
//! @ The search process is complicated slightly by the facts that some of the
//! characters might not be present in some of the fonts, and they might not
//! be probed in increasing order of height.
//!
//! @<Look at the variants of |(z,x)|; set |f| and |c|...@>=
//! if (z<>0)or(x<>min_quarterword) then
//!   begin z:=z+s+16;
//!   repeat z:=z-16; g:=fam_fnt(z);
//!   if g<>null_font then
//!     @<Look at the list of characters starting with |x| in
//!       font |g|; set |f| and |c| whenever
//!       a better character is found; |goto found| as soon as a
//!       large enough variant is encountered@>;
//!   until z<16;
//!   end
//!
//! @ @<Look at the list of characters starting with |x|...@>=
//! begin y:=x;
//! if (qo(y)>=font_bc[g])and(qo(y)<=font_ec[g]) then
//!   begin continue: q:=char_info(g)(y);
//!   if char_exists(q) then
//!     begin if char_tag(q)=ext_tag then
//!       begin f:=g; c:=y; goto found;
//!       end;
//!     hd:=height_depth(q);
//!     u:=char_height(g)(hd)+char_depth(g)(hd);
//!     if u>w then
//!       begin f:=g; c:=y; w:=u;
//!       if u>=v then goto found;
//!       end;
//!     if char_tag(q)=list_tag then
//!       begin y:=rem_byte(q); goto continue;
//!       end;
//!     end;
//!   end;
//! end
//!
//! @ Here is a subroutine that creates a new box, whose list contains a
//! single character, and whose width includes the italic correction for
//! that character. The height or depth of the box will be negative, if
//! the height or depth of the character is negative; thus, this routine
//! may deliver a slightly different result than |hpack| would produce.
//!
//! @<Declare subprocedures for |var_delimiter|@>=
//! function char_box(@!f:internal_font_number;@!c:quarterword):pointer;
//! var q:four_quarters;
//! @!hd:eight_bits; {|height_depth| byte}
//! @!b,@!p:pointer; {the new box and its character node}
//! begin q:=char_info(f)(c); hd:=height_depth(q);
//! b:=new_null_box; width(b):=char_width(f)(q)+char_italic(f)(q);
//! height(b):=char_height(f)(hd); depth(b):=char_depth(f)(hd);
//! p:=get_avail; character(p):=c; font(p):=f; list_ptr(b):=p; char_box:=b;
//! end;
//!
//! @ When the following code is executed, |char_tag(q)| will be equal to
//! |ext_tag| if and only if a built-up symbol is supposed to be returned.
//!
//! @<Make variable |b| point to a box for |(f,c)|@>=
//! if char_tag(q)=ext_tag then
//!   @<Construct an extensible character in a new box |b|,
//!     using recipe |rem_byte(q)| and font |f|@>
//! else b:=char_box(f,c)
//!
//! @ When we build an extensible character, it's handy to have the
//! following subroutine, which puts a given character on top
//! of the characters already in box |b|:
//!
//! @<Declare subprocedures for |var_delimiter|@>=
//! procedure stack_into_box(@!b:pointer;@!f:internal_font_number;
//!   @!c:quarterword);
//! var p:pointer; {new node placed into |b|}
//! begin p:=char_box(f,c); link(p):=list_ptr(b); list_ptr(b):=p;
//! height(b):=height(p);
//! end;
//!
//! @ Another handy subroutine computes the height plus depth of
//! a given character:
//!
//! @<Declare subprocedures for |var_delimiter|@>=
//! function height_plus_depth(@!f:internal_font_number;@!c:quarterword):scaled;
//! var q:four_quarters;
//! @!hd:eight_bits; {|height_depth| byte}
//! begin q:=char_info(f)(c); hd:=height_depth(q);
//! height_plus_depth:=char_height(f)(hd)+char_depth(f)(hd);
//! end;
//!
//! @ @<Construct an extensible...@>=
//! begin b:=new_null_box;
//! type(b):=vlist_node;
//! r:=font_info[exten_base[f]+rem_byte(q)].qqqq;@/
//! @<Compute the minimum suitable height, |w|, and the corresponding
//!   number of extension steps, |n|; also set |width(b)|@>;
//! c:=ext_bot(r);
//! if c<>min_quarterword then stack_into_box(b,f,c);
//! c:=ext_rep(r);
//! for m:=1 to n do stack_into_box(b,f,c);
//! c:=ext_mid(r);
//! if c<>min_quarterword then
//!   begin stack_into_box(b,f,c); c:=ext_rep(r);
//!   for m:=1 to n do stack_into_box(b,f,c);
//!   end;
//! c:=ext_top(r);
//! if c<>min_quarterword then stack_into_box(b,f,c);
//! depth(b):=w-height(b);
//! end
//!
//! @ The width of an extensible character is the width of the repeatable
//! module. If this module does not have positive height plus depth,
//! we don't use any copies of it, otherwise we use as few as possible
//! (in groups of two if there is a middle part).
//!
//! @<Compute the minimum suitable height, |w|, and...@>=
//! c:=ext_rep(r); u:=height_plus_depth(f,c);
//! w:=0; q:=char_info(f)(c); width(b):=char_width(f)(q)+char_italic(f)(q);@/
//! c:=ext_bot(r);@+if c<>min_quarterword then w:=w+height_plus_depth(f,c);
//! c:=ext_mid(r);@+if c<>min_quarterword then w:=w+height_plus_depth(f,c);
//! c:=ext_top(r);@+if c<>min_quarterword then w:=w+height_plus_depth(f,c);
//! n:=0;
//! if u>0 then while w<v do
//!   begin w:=w+u; incr(n);
//!   if ext_mid(r)<>min_quarterword then w:=w+u;
//!   end
//!
//! @ The next subroutine is much simpler; it is used for numerators and
//! denominators of fractions as well as for displayed operators and
//! their limits above and below. It takes a given box~|b| and
//! changes it so that the new box is centered in a box of width~|w|.
//! The centering is done by putting \.{\\hss} glue at the left and right
//! of the list inside |b|, then packaging the new box; thus, the
//! actual box might not really be centered, if it already contains
//! infinite glue.
//!
//! The given box might contain a single character whose italic correction
//! has been added to the width of the box; in this case a compensating
//! kern is inserted.
//!
//! @p function rebox(@!b:pointer;@!w:scaled):pointer;
//! var p:pointer; {temporary register for list manipulation}
//! @!f:internal_font_number; {font in a one-character box}
//! @!v:scaled; {width of a character without italic correction}
//! begin if (width(b)<>w)and(list_ptr(b)<>null) then
//!   begin if type(b)=vlist_node then b:=hpack(b,natural);
//!   p:=list_ptr(b);
//!   if (is_char_node(p))and(link(p)=null) then
//!     begin f:=font(p); v:=char_width(f)(char_info(f)(character(p)));
//!     if v<>width(b) then link(p):=new_kern(width(b)-v);
//!     end;
//!   free_node(b,box_node_size);
//!   b:=new_glue(ss_glue); link(b):=p;
//!   while link(p)<>null do p:=link(p);
//!   link(p):=new_glue(ss_glue);
//!   rebox:=hpack(b,w,exactly);
//!   end
//! else  begin width(b):=w; rebox:=b;
//!   end;
//! end;
//!
//! @ Here is a subroutine that creates a new glue specification from another
//! one that is expressed in `\.{mu}', given the value of the math unit.
//!
//! @d mu_mult(#)==nx_plus_y(n,#,xn_over_d(#,f,@'200000))
//!
//! @p function math_glue(@!g:pointer;@!m:scaled):pointer;
//! var p:pointer; {the new glue specification}
//! @!n:integer; {integer part of |m|}
//! @!f:scaled; {fraction part of |m|}
//! begin n:=x_over_n(m,@'200000); f:=remainder;@/
//! if f<0 then
//!   begin decr(n); f:=f+@'200000;
//!   end;
//! p:=get_node(glue_spec_size);
//! width(p):=mu_mult(width(g)); {convert \.{mu} to \.{pt}}
//! stretch_order(p):=stretch_order(g);
//! if stretch_order(p)=normal then stretch(p):=mu_mult(stretch(g))
//! else stretch(p):=stretch(g);
//! shrink_order(p):=shrink_order(g);
//! if shrink_order(p)=normal then shrink(p):=mu_mult(shrink(g))
//! else shrink(p):=shrink(g);
//! math_glue:=p;
//! end;
//!
//! @ The |math_kern| subroutine removes |mu_glue| from a kern node, given
//! the value of the math unit.
//!
//! @p procedure math_kern(@!p:pointer;@!m:scaled);
//! var @!n:integer; {integer part of |m|}
//! @!f:scaled; {fraction part of |m|}
//! begin if subtype(p)=mu_glue then
//!   begin n:=x_over_n(m,@'200000); f:=remainder;@/
//!   if f<0 then
//!     begin decr(n); f:=f+@'200000;
//!     end;
//!   width(p):=mu_mult(width(p)); subtype(p):=explicit;
//!   end;
//! end;
//!
//! @ Sometimes it is necessary to destroy an mlist. The following
//! subroutine empties the current list, assuming that |abs(mode)=mmode|.
//!
//! @p procedure flush_math;
//! begin flush_node_list(link(head)); flush_node_list(incompleat_noad);
//! link(head):=null; tail:=head; incompleat_noad:=null;
//! end;
//!
