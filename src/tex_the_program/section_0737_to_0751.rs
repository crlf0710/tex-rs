//! @ According to the rules in the \.{DVI} file specifications, we ensure alignment
//! @^square roots@>
//! between a square root sign and the rule above its nucleus by assuming that the
//! baseline of the square-root symbol is the same as the bottom of the rule. The
//! height of the square-root symbol will be the thickness of the rule, and the
//! depth of the square-root symbol should exceed or equal the height-plus-depth
//! of the nucleus plus a certain minimum clearance~|clr|. The symbol will be
//! placed so that the actual clearance is |clr| plus half the excess.
//!
//! @<Declare math...@>=
//! procedure make_radical(@!q:pointer);
//! var x,@!y:pointer; {temporary registers for box construction}
//! @!delta,@!clr:scaled; {dimensions involved in the calculation}
//! begin x:=clean_box(nucleus(q),cramped_style(cur_style));
//! if cur_style<text_style then {display style}
//!   clr:=default_rule_thickness+(abs(math_x_height(cur_size)) div 4)
//! else  begin clr:=default_rule_thickness; clr:=clr + (abs(clr) div 4);
//!   end;
//! y:=var_delimiter(left_delimiter(q),cur_size,height(x)+depth(x)+clr+
//!   default_rule_thickness);
//! delta:=depth(y)-(height(x)+depth(x)+clr);
//! if delta>0 then clr:=clr+half(delta); {increase the actual clearance}
//! shift_amount(y):=-(height(x)+clr);
//! link(y):=overbar(x,clr,height(y));
//! info(nucleus(q)):=hpack(y,natural); math_type(nucleus(q)):=sub_box;
//! end;
//!
//! @ Slants are not considered when placing accents in math mode. The accenter is
//! centered over the accentee, and the accent width is treated as zero with
//! respect to the size of the final box.
//!
//! @<Declare math...@>=
//! procedure make_math_accent(@!q:pointer);
//! label done,done1;
//! var p,@!x,@!y:pointer; {temporary registers for box construction}
//! @!a:integer; {address of lig/kern instruction}
//! @!c:quarterword; {accent character}
//! @!f:internal_font_number; {its font}
//! @!i:four_quarters; {its |char_info|}
//! @!s:scaled; {amount to skew the accent to the right}
//! @!h:scaled; {height of character being accented}
//! @!delta:scaled; {space to remove between accent and accentee}
//! @!w:scaled; {width of the accentee, not including sub/superscripts}
//! begin fetch(accent_chr(q));
//! if char_exists(cur_i) then
//!   begin i:=cur_i; c:=cur_c; f:=cur_f;@/
//!   @<Compute the amount of skew@>;
//!   x:=clean_box(nucleus(q),cramped_style(cur_style)); w:=width(x); h:=height(x);
//!   @<Switch to a larger accent if available and appropriate@>;
//!   if h<x_height(f) then delta:=h@+else delta:=x_height(f);
//!   if (math_type(supscr(q))<>empty)or(math_type(subscr(q))<>empty) then
//!     if math_type(nucleus(q))=math_char then
//!       @<Swap the subscript and superscript into box |x|@>;
//!   y:=char_box(f,c);
//!   shift_amount(y):=s+half(w-width(y));
//!   width(y):=0; p:=new_kern(-delta); link(p):=x; link(y):=p;
//!   y:=vpack(y,natural); width(y):=width(x);
//!   if height(y)<h then @<Make the height of box |y| equal to |h|@>;
//!   info(nucleus(q)):=y;
//!   math_type(nucleus(q)):=sub_box;
//!   end;
//! end;
//!
//! @ @<Make the height of box |y|...@>=
//! begin p:=new_kern(h-height(y)); link(p):=list_ptr(y); list_ptr(y):=p;
//! height(y):=h;
//! end
//!
//! @ @<Switch to a larger accent if available and appropriate@>=
//! loop@+  begin if char_tag(i)<>list_tag then goto done;
//!   y:=rem_byte(i);
//!   i:=char_info(f)(y);
//!   if not char_exists(i) then goto done;
//!   if char_width(f)(i)>w then goto done;
//!   c:=y;
//!   end;
//! done:
//!
//! @ @<Compute the amount of skew@>=
//! s:=0;
//! if math_type(nucleus(q))=math_char then
//!   begin fetch(nucleus(q));
//!   if char_tag(cur_i)=lig_tag then
//!     begin a:=lig_kern_start(cur_f)(cur_i);
//!     cur_i:=font_info[a].qqqq;
//!     if skip_byte(cur_i)>stop_flag then
//!       begin a:=lig_kern_restart(cur_f)(cur_i);
//!       cur_i:=font_info[a].qqqq;
//!       end;
//!     loop@+ begin if qo(next_char(cur_i))=skew_char[cur_f] then
//!         begin if op_byte(cur_i)>=kern_flag then
//!           if skip_byte(cur_i)<=stop_flag then s:=char_kern(cur_f)(cur_i);
//!         goto done1;
//!         end;
//!       if skip_byte(cur_i)>=stop_flag then goto done1;
//!       a:=a+qo(skip_byte(cur_i))+1;
//!       cur_i:=font_info[a].qqqq;
//!       end;
//!     end;
//!   end;
//! done1:
//!
//! @ @<Swap the subscript and superscript into box |x|@>=
//! begin flush_node_list(x); x:=new_noad;
//! mem[nucleus(x)]:=mem[nucleus(q)];
//! mem[supscr(x)]:=mem[supscr(q)];
//! mem[subscr(x)]:=mem[subscr(q)];@/
//! mem[supscr(q)].hh:=empty_field;
//! mem[subscr(q)].hh:=empty_field;@/
//! math_type(nucleus(q)):=sub_mlist; info(nucleus(q)):=x;
//! x:=clean_box(nucleus(q),cur_style); delta:=delta+height(x)-h; h:=height(x);
//! end
//!
//! @ The |make_fraction| procedure is a bit different because it sets
//! |new_hlist(q)| directly rather than making a sub-box.
//!
//! @<Declare math...@>=
//! procedure make_fraction(@!q:pointer);
//! var p,@!v,@!x,@!y,@!z:pointer; {temporary registers for box construction}
//! @!delta,@!delta1,@!delta2,@!shift_up,@!shift_down,@!clr:scaled;
//!   {dimensions for box calculations}
//! begin if thickness(q)=default_code then thickness(q):=default_rule_thickness;
//! @<Create equal-width boxes |x| and |z| for the numerator and denominator,
//!   and compute the default amounts |shift_up| and |shift_down| by which they
//!   are displaced from the baseline@>;
//! if thickness(q)=0 then @<Adjust \(s)|shift_up| and |shift_down| for the case
//!   of no fraction line@>
//! else @<Adjust \(s)|shift_up| and |shift_down| for the case of a fraction line@>;
//! @<Construct a vlist box for the fraction, according to |shift_up| and
//!   |shift_down|@>;
//! @<Put the \(f)fraction into a box with its delimiters, and make |new_hlist(q)|
//!   point to it@>;
//! end;
//!
//! @ @<Create equal-width boxes |x| and |z| for the numerator and denom...@>=
//! x:=clean_box(numerator(q),num_style(cur_style));
//! z:=clean_box(denominator(q),denom_style(cur_style));
//! if width(x)<width(z) then x:=rebox(x,width(z))
//! else z:=rebox(z,width(x));
//! if cur_style<text_style then {display style}
//!   begin shift_up:=num1(cur_size); shift_down:=denom1(cur_size);
//!   end
//! else  begin shift_down:=denom2(cur_size);
//!   if thickness(q)<>0 then shift_up:=num2(cur_size)
//!   else shift_up:=num3(cur_size);
//!   end
//!
//! @ The numerator and denominator must be separated by a certain minimum
//! clearance, called |clr| in the following program. The difference between
//! |clr| and the actual clearance is twice |delta|.
//!
//! @<Adjust \(s)|shift_up| and |shift_down| for the case of no fraction line@>=
//! begin if cur_style<text_style then clr:=7*default_rule_thickness
//! else clr:=3*default_rule_thickness;
//! delta:=half(clr-((shift_up-depth(x))-(height(z)-shift_down)));
//! if delta>0 then
//!   begin shift_up:=shift_up+delta;
//!   shift_down:=shift_down+delta;
//!   end;
//! end
//!
//! @ In the case of a fraction line, the minimum clearance depends on the actual
//! thickness of the line.
//!
//! @<Adjust \(s)|shift_up| and |shift_down| for the case of a fraction line@>=
//! begin if cur_style<text_style then clr:=3*thickness(q)
//! else clr:=thickness(q);
//! delta:=half(thickness(q));
//! delta1:=clr-((shift_up-depth(x))-(axis_height(cur_size)+delta));
//! delta2:=clr-((axis_height(cur_size)-delta)-(height(z)-shift_down));
//! if delta1>0 then shift_up:=shift_up+delta1;
//! if delta2>0 then shift_down:=shift_down+delta2;
//! end
//!
//! @ @<Construct a vlist box for the fraction...@>=
//! v:=new_null_box; type(v):=vlist_node;
//! height(v):=shift_up+height(x); depth(v):=depth(z)+shift_down;
//! width(v):=width(x); {this also equals |width(z)|}
//! if thickness(q)=0 then
//!   begin p:=new_kern((shift_up-depth(x))-(height(z)-shift_down));
//!   link(p):=z;
//!   end
//! else  begin y:=fraction_rule(thickness(q));@/
//!   p:=new_kern((axis_height(cur_size)-delta)-@|(height(z)-shift_down));@/
//!   link(y):=p; link(p):=z;@/
//!   p:=new_kern((shift_up-depth(x))-(axis_height(cur_size)+delta));
//!   link(p):=y;
//!   end;
//! link(x):=p; list_ptr(v):=x
//!
//! @ @<Put the \(f)fraction into a box with its delimiters...@>=
//! if cur_style<text_style then delta:=delim1(cur_size)
//! else delta:=delim2(cur_size);
//! x:=var_delimiter(left_delimiter(q), cur_size, delta); link(x):=v;@/
//! z:=var_delimiter(right_delimiter(q), cur_size, delta); link(v):=z;@/
//! new_hlist(q):=hpack(x,natural)
//!
//! @ If the nucleus of an |op_noad| is a single character, it is to be
//! centered vertically with respect to the axis, after first being enlarged
//! (via a character list in the font) if we are in display style.  The normal
//! convention for placing displayed limits is to put them above and below the
//! operator in display style.
//!
//! The italic correction is removed from the character if there is a subscript
//! and the limits are not being displayed. The |make_op|
//! routine returns the value that should be used as an offset between
//! subscript and superscript.
//!
//! After |make_op| has acted, |subtype(q)| will be |limits| if and only if
//! the limits have been set above and below the operator. In that case,
//! |new_hlist(q)| will already contain the desired final box.
//!
//! @<Declare math...@>=
//! function make_op(@!q:pointer):scaled;
//! var delta:scaled; {offset between subscript and superscript}
//! @!p,@!v,@!x,@!y,@!z:pointer; {temporary registers for box construction}
//! @!c:quarterword;@+@!i:four_quarters; {registers for character examination}
//! @!shift_up,@!shift_down:scaled; {dimensions for box calculation}
//! begin if (subtype(q)=normal)and(cur_style<text_style) then
//!   subtype(q):=limits;
//! if math_type(nucleus(q))=math_char then
//!   begin fetch(nucleus(q));
//!   if (cur_style<text_style)and(char_tag(cur_i)=list_tag) then {make it larger}
//!     begin c:=rem_byte(cur_i); i:=char_info(cur_f)(c);
//!     if char_exists(i) then
//!       begin cur_c:=c; cur_i:=i; character(nucleus(q)):=c;
//!       end;
//!     end;
//!   delta:=char_italic(cur_f)(cur_i); x:=clean_box(nucleus(q),cur_style);
//!   if (math_type(subscr(q))<>empty)and(subtype(q)<>limits) then
//!     width(x):=width(x)-delta; {remove italic correction}
//!   shift_amount(x):=half(height(x)-depth(x)) - axis_height(cur_size);
//!     {center vertically}
//!   math_type(nucleus(q)):=sub_box; info(nucleus(q)):=x;
//!   end
//! else delta:=0;
//! if subtype(q)=limits then
//!   @<Construct a box with limits above and below it, skewed by |delta|@>;
//! make_op:=delta;
//! end;
//!
//! @ The following program builds a vlist box |v| for displayed limits. The
//! width of the box is not affected by the fact that the limits may be skewed.
//!
//! @<Construct a box with limits above and below it...@>=
//! begin x:=clean_box(supscr(q),sup_style(cur_style));
//! y:=clean_box(nucleus(q),cur_style);
//! z:=clean_box(subscr(q),sub_style(cur_style));
//! v:=new_null_box; type(v):=vlist_node; width(v):=width(y);
//! if width(x)>width(v) then width(v):=width(x);
//! if width(z)>width(v) then width(v):=width(z);
//! x:=rebox(x,width(v)); y:=rebox(y,width(v)); z:=rebox(z,width(v));@/
//! shift_amount(x):=half(delta); shift_amount(z):=-shift_amount(x);
//! height(v):=height(y); depth(v):=depth(y);
//! @<Attach the limits to |y| and adjust |height(v)|, |depth(v)| to
//!   account for their presence@>;
//! new_hlist(q):=v;
//! end
//!
//! @ We use |shift_up| and |shift_down| in the following program for the
//! amount of glue between the displayed operator |y| and its limits |x| and
//! |z|. The vlist inside box |v| will consist of |x| followed by |y| followed
//! by |z|, with kern nodes for the spaces between and around them.
//!
//! @<Attach the limits to |y| and adjust |height(v)|, |depth(v)|...@>=
//! if math_type(supscr(q))=empty then
//!   begin free_node(x,box_node_size); list_ptr(v):=y;
//!   end
//! else  begin shift_up:=big_op_spacing3-depth(x);
//!   if shift_up<big_op_spacing1 then shift_up:=big_op_spacing1;
//!   p:=new_kern(shift_up); link(p):=y; link(x):=p;@/
//!   p:=new_kern(big_op_spacing5); link(p):=x; list_ptr(v):=p;
//!   height(v):=height(v)+big_op_spacing5+height(x)+depth(x)+shift_up;
//!   end;
//! if math_type(subscr(q))=empty then free_node(z,box_node_size)
//! else  begin shift_down:=big_op_spacing4-height(z);
//!   if shift_down<big_op_spacing2 then shift_down:=big_op_spacing2;
//!   p:=new_kern(shift_down); link(y):=p; link(p):=z;@/
//!   p:=new_kern(big_op_spacing5); link(z):=p;
//!   depth(v):=depth(v)+big_op_spacing5+height(z)+depth(z)+shift_down;
//!   end
//!
