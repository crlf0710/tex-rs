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
