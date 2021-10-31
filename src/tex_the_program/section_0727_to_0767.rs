//! @ We use the fact that no character nodes appear in an mlist, hence
//! the field |type(q)| is always present.
//!
//! @<Process node-or-noad...@>=
//! begin @<Do first-pass processing based on |type(q)|; |goto done_with_noad|
//!   if a noad has been fully processed, |goto check_dimensions| if it
//!   has been translated into |new_hlist(q)|, or |goto done_with_node|
//!   if a node has been fully processed@>;
//! check_dimensions: z:=hpack(new_hlist(q),natural);
//! if height(z)>max_h then max_h:=height(z);
//! if depth(z)>max_d then max_d:=depth(z);
//! free_node(z,box_node_size);
//! done_with_noad: r:=q; r_type:=type(r);
//! done_with_node: q:=link(q);
//! end
//!
//! @ One of the things we must do on the first pass is change a |bin_noad| to
//! an |ord_noad| if the |bin_noad| is not in the context of a binary operator.
//! The values of |r| and |r_type| make this fairly easy.
//!
//! @<Do first-pass processing...@>=
//! reswitch: delta:=0;
//! case type(q) of
//! bin_noad: case r_type of
//!   bin_noad,op_noad,rel_noad,open_noad,punct_noad,left_noad:
//!     begin type(q):=ord_noad; goto reswitch;
//!     end;
//!   othercases do_nothing
//!   endcases;
//! rel_noad,close_noad,punct_noad,right_noad: begin@t@>@;@/
//!   @<Convert \(a)a final |bin_noad| to an |ord_noad|@>;
//!   if type(q)=right_noad then goto done_with_noad;
//!   end;
//! @t\4@>@<Cases for noads that can follow a |bin_noad|@>@;
//! @t\4@>@<Cases for nodes that can appear in an mlist, after which we
//!   |goto done_with_node|@>@;
//! othercases confusion("mlist1")
//! @:this can't happen mlist1}{\quad mlist1@>
//! endcases;@/
//! @<Convert \(n)|nucleus(q)| to an hlist and attach the sub/superscripts@>
//!
//! @ @<Convert \(a)a final |bin_noad| to an |ord_noad|@>=
//! if r_type=bin_noad then type(r):=ord_noad
//!
//! @ @<Cases for nodes that can appear in an mlist...@>=
//! style_node: begin cur_style:=subtype(q);
//!   @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
//!   goto done_with_node;
//!   end;
//! choice_node: @<Change this node to a style node followed by the correct choice,
//!    then |goto done_with_node|@>;
//! ins_node,mark_node,adjust_node,
//!   whatsit_node,penalty_node,disc_node: goto done_with_node;
//! rule_node: begin if height(q)>max_h then max_h:=height(q);
//!   if depth(q)>max_d then max_d:=depth(q); goto done_with_node;
//!   end;
//! glue_node: begin @<Convert \(m)math glue to ordinary glue@>;
//!   goto done_with_node;
//!   end;
//! kern_node: begin math_kern(q,cur_mu); goto done_with_node;
//!   end;
//!
//! @ @d choose_mlist(#)==begin p:=#(q); #(q):=null;@+end
//!
//! @<Change this node to a style node...@>=
//! begin case cur_style div 2 of
//! 0: choose_mlist(display_mlist); {|display_style=0|}
//! 1: choose_mlist(text_mlist); {|text_style=2|}
//! 2: choose_mlist(script_mlist); {|script_style=4|}
//! 3: choose_mlist(script_script_mlist); {|script_script_style=6|}
//! end; {there are no other cases}
//! flush_node_list(display_mlist(q));
//! flush_node_list(text_mlist(q));
//! flush_node_list(script_mlist(q));
//! flush_node_list(script_script_mlist(q));@/
//! type(q):=style_node; subtype(q):=cur_style; width(q):=0; depth(q):=0;
//! if p<>null then
//!   begin z:=link(q); link(q):=p;
//!   while link(p)<>null do p:=link(p);
//!   link(p):=z;
//!   end;
//! goto done_with_node;
//! end
//!
//! @ Conditional math glue (`\.{\\nonscript}') results in a |glue_node|
//! pointing to |zero_glue|, with |subtype(q)=cond_math_glue|; in such a case
//! the node following will be eliminated if it is a glue or kern node and if the
//! current size is different from |text_size|. Unconditional math glue
//! (`\.{\\muskip}') is converted to normal glue by multiplying the dimensions
//! by |cur_mu|.
//! @!@:non_script_}{\.{\\nonscript} primitive@>
//!
//! @<Convert \(m)math glue to ordinary glue@>=
//! if subtype(q)=mu_glue then
//!   begin x:=glue_ptr(q);
//!   y:=math_glue(x,cur_mu); delete_glue_ref(x); glue_ptr(q):=y;
//!   subtype(q):=normal;
//!   end
//! else if (cur_size<>text_size)and(subtype(q)=cond_math_glue) then
//!   begin p:=link(q);
//!   if p<>null then if (type(p)=glue_node)or(type(p)=kern_node) then
//!     begin link(q):=link(p); link(p):=null; flush_node_list(p);
//!     end;
//!   end
//!
//! @ @<Cases for noads that can follow a |bin_noad|@>=
//! left_noad: goto done_with_noad;
//! fraction_noad: begin make_fraction(q); goto check_dimensions;
//!   end;
//! op_noad: begin delta:=make_op(q);
//!   if subtype(q)=limits then goto check_dimensions;
//!   end;
//! ord_noad: make_ord(q);
//! open_noad,inner_noad: do_nothing;
//! radical_noad: make_radical(q);
//! over_noad: make_over(q);
//! under_noad: make_under(q);
//! accent_noad: make_math_accent(q);
//! vcenter_noad: make_vcenter(q);
//!
//! @ Most of the actual construction work of |mlist_to_hlist| is done
//! by procedures with names
//! like |make_fraction|, |make_radical|, etc. To illustrate
//! the general setup of such procedures, let's begin with a couple of
//! simple ones.
//!
//! @<Declare math...@>=
//! procedure make_over(@!q:pointer);
//! begin info(nucleus(q)):=@|
//!   overbar(clean_box(nucleus(q),cramped_style(cur_style)),@|
//!   3*default_rule_thickness,default_rule_thickness);
//! math_type(nucleus(q)):=sub_box;
//! end;
//!
//! @ @<Declare math...@>=
//! procedure make_under(@!q:pointer);
//! var p,@!x,@!y: pointer; {temporary registers for box construction}
//! @!delta:scaled; {overall height plus depth}
//! begin x:=clean_box(nucleus(q),cur_style);
//! p:=new_kern(3*default_rule_thickness); link(x):=p;
//! link(p):=fraction_rule(default_rule_thickness);
//! y:=vpack(x,natural);
//! delta:=height(y)+depth(y)+default_rule_thickness;
//! height(y):=height(x); depth(y):=delta-height(y);
//! info(nucleus(q)):=y; math_type(nucleus(q)):=sub_box;
//! end;
//!
//! @ @<Declare math...@>=
//! procedure make_vcenter(@!q:pointer);
//! var v:pointer; {the box that should be centered vertically}
//! @!delta:scaled; {its height plus depth}
//! begin v:=info(nucleus(q));
//! if type(v)<>vlist_node then confusion("vcenter");
//! @:this can't happen vcenter}{\quad vcenter@>
//! delta:=height(v)+depth(v);
//! height(v):=axis_height(cur_size)+half(delta);
//! depth(v):=delta-height(v);
//! end;
//!
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
//! @ A ligature found in a math formula does not create a |ligature_node|, because
//! there is no question of hyphenation afterwards; the ligature will simply be
//! stored in an ordinary |char_node|, after residing in an |ord_noad|.
//!
//! The |math_type| is converted to |math_text_char| here if we would not want to
//! apply an italic correction to the current character unless it belongs
//! to a math font (i.e., a font with |space=0|).
//!
//! No boundary characters enter into these ligatures.
//!
//! @<Declare math...@>=
//! procedure make_ord(@!q:pointer);
//! label restart,exit;
//! var a:integer; {address of lig/kern instruction}
//! @!p,@!r:pointer; {temporary registers for list manipulation}
//! begin restart:@t@>@;@/
//! if math_type(subscr(q))=empty then if math_type(supscr(q))=empty then
//!  if math_type(nucleus(q))=math_char then
//!   begin p:=link(q);
//!   if p<>null then if (type(p)>=ord_noad)and(type(p)<=punct_noad) then
//!     if math_type(nucleus(p))=math_char then
//!     if fam(nucleus(p))=fam(nucleus(q)) then
//!       begin math_type(nucleus(q)):=math_text_char;
//!       fetch(nucleus(q));
//!       if char_tag(cur_i)=lig_tag then
//!         begin a:=lig_kern_start(cur_f)(cur_i);
//!         cur_c:=character(nucleus(p));
//!         cur_i:=font_info[a].qqqq;
//!         if skip_byte(cur_i)>stop_flag then
//!           begin a:=lig_kern_restart(cur_f)(cur_i);
//!           cur_i:=font_info[a].qqqq;
//!           end;
//!         loop@+ begin @<If instruction |cur_i| is a kern with |cur_c|, attach
//!             the kern after~|q|; or if it is a ligature with |cur_c|, combine
//!             noads |q| and~|p| appropriately; then |return| if the cursor has
//!             moved past a noad, or |goto restart|@>;
//!           if skip_byte(cur_i)>=stop_flag then return;
//!           a:=a+qo(skip_byte(cur_i))+1;
//!           cur_i:=font_info[a].qqqq;
//!           end;
//!         end;
//!       end;
//!   end;
//! exit:end;
//!
//! @ Note that a ligature between an |ord_noad| and another kind of noad
//! is replaced by an |ord_noad|, when the two noads collapse into one.
//! But we could make a parenthesis (say) change shape when it follows
//! certain letters. Presumably a font designer will define such
//! ligatures only when this convention makes sense.
//!
//! \chardef\?='174 % vertical line to indicate character retention
//!
//! @<If instruction |cur_i| is a kern with |cur_c|, ...@>=
//! if next_char(cur_i)=cur_c then if skip_byte(cur_i)<=stop_flag then
//!   if op_byte(cur_i)>=kern_flag then
//!     begin p:=new_kern(char_kern(cur_f)(cur_i));
//!     link(p):=link(q); link(q):=p; return;
//!     end
//!   else  begin check_interrupt; {allow a way out of infinite ligature loop}
//!     case op_byte(cur_i) of
//!   qi(1),qi(5): character(nucleus(q)):=rem_byte(cur_i); {\.{=:\?}, \.{=:\?>}}
//!   qi(2),qi(6): character(nucleus(p)):=rem_byte(cur_i); {\.{\?=:}, \.{\?=:>}}
//!   qi(3),qi(7),qi(11):begin r:=new_noad; {\.{\?=:\?}, \.{\?=:\?>}, \.{\?=:\?>>}}
//!       character(nucleus(r)):=rem_byte(cur_i);
//!       fam(nucleus(r)):=fam(nucleus(q));@/
//!       link(q):=r; link(r):=p;
//!       if op_byte(cur_i)<qi(11) then math_type(nucleus(r)):=math_char
//!       else math_type(nucleus(r)):=math_text_char; {prevent combination}
//!       end;
//!     othercases begin link(q):=link(p);
//!       character(nucleus(q)):=rem_byte(cur_i); {\.{=:}}
//!       mem[subscr(q)]:=mem[subscr(p)]; mem[supscr(q)]:=mem[supscr(p)];@/
//!       free_node(p,noad_size);
//!       end
//!     endcases;
//!     if op_byte(cur_i)>qi(3) then return;
//!     math_type(nucleus(q)):=math_char; goto restart;
//!     end
//!
//! @ When we get to the following part of the program, we have ``fallen through''
//! from cases that did not lead to |check_dimensions| or |done_with_noad| or
//! |done_with_node|. Thus, |q|~points to a noad whose nucleus may need to be
//! converted to an hlist, and whose subscripts and superscripts need to be
//! appended if they are present.
//!
//! If |nucleus(q)| is not a |math_char|, the variable |delta| is the amount
//! by which a superscript should be moved right with respect to a subscript
//! when both are present.
//! @^subscripts@>
//! @^superscripts@>
//!
//! @<Convert \(n)|nucleus(q)| to an hlist and attach the sub/superscripts@>=
//! case math_type(nucleus(q)) of
//! math_char, math_text_char:
//!   @<Create a character node |p| for |nucleus(q)|, possibly followed
//!   by a kern node for the italic correction, and set |delta| to the
//!   italic correction if a subscript is present@>;
//! empty: p:=null;
//! sub_box: p:=info(nucleus(q));
//! sub_mlist: begin cur_mlist:=info(nucleus(q)); save_style:=cur_style;
//!   mlist_penalties:=false; mlist_to_hlist; {recursive call}
//! @^recursion@>
//!   cur_style:=save_style; @<Set up the values...@>;
//!   p:=hpack(link(temp_head),natural);
//!   end;
//! othercases confusion("mlist2")
//! @:this can't happen mlist2}{\quad mlist2@>
//! endcases;@/
//! new_hlist(q):=p;
//! if (math_type(subscr(q))=empty)and(math_type(supscr(q))=empty) then
//!   goto check_dimensions;
//! make_scripts(q,delta)
//!
//! @ @<Create a character node |p| for |nucleus(q)|...@>=
//! begin fetch(nucleus(q));
//! if char_exists(cur_i) then
//!   begin delta:=char_italic(cur_f)(cur_i); p:=new_character(cur_f,qo(cur_c));
//!   if (math_type(nucleus(q))=math_text_char)and(space(cur_f)<>0) then
//!     delta:=0; {no italic correction in mid-word of text font}
//!   if (math_type(subscr(q))=empty)and(delta<>0) then
//!     begin link(p):=new_kern(delta); delta:=0;
//!     end;
//!   end
//! else p:=null;
//! end
//!
//! @ The purpose of |make_scripts(q,delta)| is to attach the subscript and/or
//! superscript of noad |q| to the list that starts at |new_hlist(q)|,
//! given that the subscript and superscript aren't both empty. The superscript
//! will appear to the right of the subscript by a given distance |delta|.
//!
//! We set |shift_down| and |shift_up| to the minimum amounts to shift the
//! baseline of subscripts and superscripts based on the given nucleus.
//!
//! @<Declare math...@>=
//! procedure make_scripts(@!q:pointer;@!delta:scaled);
//! var p,@!x,@!y,@!z:pointer; {temporary registers for box construction}
//! @!shift_up,@!shift_down,@!clr:scaled; {dimensions in the calculation}
//! @!t:small_number; {subsidiary size code}
//! begin p:=new_hlist(q);
//! if is_char_node(p) then
//!   begin shift_up:=0; shift_down:=0;
//!   end
//! else  begin z:=hpack(p,natural);
//!   if cur_style<script_style then t:=script_size@+else t:=script_script_size;
//!   shift_up:=height(z)-sup_drop(t);
//!   shift_down:=depth(z)+sub_drop(t);
//!   free_node(z,box_node_size);
//!   end;
//! if math_type(supscr(q))=empty then
//!   @<Construct a subscript box |x| when there is no superscript@>
//! else  begin @<Construct a superscript box |x|@>;
//!   if math_type(subscr(q))=empty then shift_amount(x):=-shift_up
//!   else @<Construct a sub/superscript combination box |x|, with the
//!     superscript offset by |delta|@>;
//!   end;
//! if new_hlist(q)=null then new_hlist(q):=x
//! else  begin p:=new_hlist(q);
//!   while link(p)<>null do p:=link(p);
//!   link(p):=x;
//!   end;
//! end;
//!
//! @ When there is a subscript without a superscript, the top of the subscript
//! should not exceed the baseline plus four-fifths of the x-height.
//!
//! @<Construct a subscript box |x| when there is no superscript@>=
//! begin x:=clean_box(subscr(q),sub_style(cur_style));
//! width(x):=width(x)+script_space;
//! if shift_down<sub1(cur_size) then shift_down:=sub1(cur_size);
//! clr:=height(x)-(abs(math_x_height(cur_size)*4) div 5);
//! if shift_down<clr then shift_down:=clr;
//! shift_amount(x):=shift_down;
//! end
//!
//! @ The bottom of a superscript should never descend below the baseline plus
//! one-fourth of the x-height.
//!
//! @<Construct a superscript box |x|@>=
//! begin x:=clean_box(supscr(q),sup_style(cur_style));
//! width(x):=width(x)+script_space;
//! if odd(cur_style) then clr:=sup3(cur_size)
//! else if cur_style<text_style then clr:=sup1(cur_size)
//! else clr:=sup2(cur_size);
//! if shift_up<clr then shift_up:=clr;
//! clr:=depth(x)+(abs(math_x_height(cur_size)) div 4);
//! if shift_up<clr then shift_up:=clr;
//! end
//!
//! @ When both subscript and superscript are present, the subscript must be
//! separated from the superscript by at least four times |default_rule_thickness|.
//! If this condition would be violated, the subscript moves down, after which
//! both subscript and superscript move up so that the bottom of the superscript
//! is at least as high as the baseline plus four-fifths of the x-height.
//!
//! @<Construct a sub/superscript combination box |x|...@>=
//! begin y:=clean_box(subscr(q),sub_style(cur_style));
//! width(y):=width(y)+script_space;
//! if shift_down<sub2(cur_size) then shift_down:=sub2(cur_size);
//! clr:=4*default_rule_thickness-
//!   ((shift_up-depth(x))-(height(y)-shift_down));
//! if clr>0 then
//!   begin shift_down:=shift_down+clr;
//!   clr:=(abs(math_x_height(cur_size)*4) div 5)-(shift_up-depth(x));
//!   if clr>0 then
//!     begin shift_up:=shift_up+clr;
//!     shift_down:=shift_down-clr;
//!     end;
//!   end;
//! shift_amount(x):=delta; {superscript is |delta| to the right of the subscript}
//! p:=new_kern((shift_up-depth(x))-(height(y)-shift_down)); link(x):=p; link(p):=y;
//! x:=vpack(x,natural); shift_amount(x):=shift_down;
//! end
//!
//! @ We have now tied up all the loose ends of the first pass of |mlist_to_hlist|.
//! The second pass simply goes through and hooks everything together with the
//! proper glue and penalties. It also handles the |left_noad| and |right_noad| that
//! might be present, since |max_h| and |max_d| are now known. Variable |p| points
//! to a node at the current end of the final hlist.
//!
//! @<Make a second pass over the mlist, ...@>=
//! p:=temp_head; link(p):=null; q:=mlist; r_type:=0; cur_style:=style;
//! @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
//! while q<>null do
//!   begin @<If node |q| is a style node, change the style and |goto delete_q|;
//!     otherwise if it is not a noad, put it into the hlist,
//!     advance |q|, and |goto done|; otherwise set |s| to the size
//!     of noad |q|, set |t| to the associated type (|ord_noad..
//!     inner_noad|), and set |pen| to the associated penalty@>;
//!   @<Append inter-element spacing based on |r_type| and |t|@>;
//!   @<Append any |new_hlist| entries for |q|, and any appropriate penalties@>;
//!   r_type:=t;
//!   delete_q: r:=q; q:=link(q); free_node(r,s);
//!   done: end
//!
//! @ Just before doing the big |case| switch in the second pass, the program
//! sets up default values so that most of the branches are short.
//!
//! @<If node |q| is a style node, change the style...@>=
//! t:=ord_noad; s:=noad_size; pen:=inf_penalty;
//! case type(q) of
//! op_noad,open_noad,close_noad,punct_noad,inner_noad: t:=type(q);
//! bin_noad: begin t:=bin_noad; pen:=bin_op_penalty;
//!   end;
//! rel_noad: begin t:=rel_noad; pen:=rel_penalty;
//!   end;
//! ord_noad,vcenter_noad,over_noad,under_noad: do_nothing;
//! radical_noad: s:=radical_noad_size;
//! accent_noad: s:=accent_noad_size;
//! fraction_noad: s:=fraction_noad_size;
//! left_noad,right_noad: t:=make_left_right(q,style,max_d,max_h);
//! style_node: @<Change the current style and |goto delete_q|@>;
//! whatsit_node,penalty_node,rule_node,disc_node,adjust_node,ins_node,mark_node,
//!  glue_node,kern_node:@t@>@;@/
//!   begin link(p):=q; p:=q; q:=link(q); link(p):=null; goto done;
//!   end;
//! othercases confusion("mlist3")
//! @:this can't happen mlist3}{\quad mlist3@>
//! endcases
//!
//! @ The |make_left_right| function constructs a left or right delimiter of
//! the required size and returns the value |open_noad| or |close_noad|. The
//! |right_noad| and |left_noad| will both be based on the original |style|,
//! so they will have consistent sizes.
//!
//! We use the fact that |right_noad-left_noad=close_noad-open_noad|.
//!
//! @<Declare math...@>=
//! function make_left_right(@!q:pointer;@!style:small_number;
//!   @!max_d,@!max_h:scaled):small_number;
//! var delta,@!delta1,@!delta2:scaled; {dimensions used in the calculation}
//! begin if style<script_style then cur_size:=text_size
//! else cur_size:=16*((style-text_style) div 2);
//! delta2:=max_d+axis_height(cur_size);
//! delta1:=max_h+max_d-delta2;
//! if delta2>delta1 then delta1:=delta2; {|delta1| is max distance from axis}
//! delta:=(delta1 div 500)*delimiter_factor;
//! delta2:=delta1+delta1-delimiter_shortfall;
//! if delta<delta2 then delta:=delta2;
//! new_hlist(q):=var_delimiter(delimiter(q),cur_size,delta);
//! make_left_right:=type(q)-(left_noad-open_noad); {|open_noad| or |close_noad|}
//! end;
//!
//! @ @<Change the current style and |goto delete_q|@>=
//! begin cur_style:=subtype(q); s:=style_node_size;
//! @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
//! goto delete_q;
//! end
//!
//! @ The inter-element spacing in math formulas depends on an $8\times8$ table that
//! \TeX\ preloads as a 64-digit string. The elements of this string have the
//! following significance:
//! $$\vbox{\halign{#\hfil\cr
//! \.0 means no space;\cr
//! \.1 means a conditional thin space (\.{\\nonscript\\mskip\\thinmuskip});\cr
//! \.2 means a thin space (\.{\\mskip\\thinmuskip});\cr
//! \.3 means a conditional medium space
//!   (\.{\\nonscript\\mskip\\medmuskip});\cr
//! \.4 means a conditional thick space
//!   (\.{\\nonscript\\mskip\\thickmuskip});\cr
//! \.* means an impossible case.\cr}}$$
//! This is all pretty cryptic, but {\sl The \TeX book\/} explains what is
//! supposed to happen, and the string makes it happen.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! A global variable |magic_offset| is computed so that if |a| and |b| are
//! in the range |ord_noad..inner_noad|, then |str_pool[a*8+b+magic_offset]|
//! is the digit for spacing between noad types |a| and |b|.
//!
//! If \PASCAL\ had provided a good way to preload constant arrays, this part of
//! the program would not have been so strange.
//! @:PASCAL}{\PASCAL@>
//!
//! @d math_spacing=@;@/
//! @t\hskip-35pt@>
//! "0234000122*4000133**3**344*0400400*000000234000111*1111112341011"
//! @t$ \hskip-35pt$@>
//!
//! @<Glob...@>=
//! @!magic_offset:integer; {used to find inter-element spacing}
//!
//! @ @<Compute the magic offset@>=
//! magic_offset:=str_start[math_spacing]-9*ord_noad
//!
//! @ @<Append inter-element spacing based on |r_type| and |t|@>=
//! if r_type>0 then {not the first noad}
//!   begin case so(str_pool[r_type*8+t+magic_offset]) of
//!   "0": x:=0;
//!   "1": if cur_style<script_style then x:=thin_mu_skip_code@+else x:=0;
//!   "2": x:=thin_mu_skip_code;
//!   "3": if cur_style<script_style then x:=med_mu_skip_code@+else x:=0;
//!   "4": if cur_style<script_style then x:=thick_mu_skip_code@+else x:=0;
//!   othercases confusion("mlist4")
//! @:this can't happen mlist4}{\quad mlist4@>
//!   endcases;
//!   if x<>0 then
//!     begin y:=math_glue(glue_par(x),cur_mu);
//!     z:=new_glue(y); glue_ref_count(y):=null; link(p):=z; p:=z;@/
//!     subtype(z):=x+1; {store a symbolic subtype}
//!     end;
//!   end
//!
//! @ We insert a penalty node after the hlist entries of noad |q| if |pen|
//! is not an ``infinite'' penalty, and if the node immediately following |q|
//! is not a penalty node or a |rel_noad| or absent entirely.
//!
//! @<Append any |new_hlist| entries for |q|, and any appropriate penalties@>=
//! if new_hlist(q)<>null then
//!   begin link(p):=new_hlist(q);
//!   repeat p:=link(p);
//!   until link(p)=null;
//!   end;
//! if penalties then if link(q)<>null then if pen<inf_penalty then
//!   begin r_type:=type(link(q));
//!   if r_type<>penalty_node then if r_type<>rel_noad then
//!     begin z:=new_penalty(pen); link(p):=z; p:=z;
//!     end;
//!   end
//!
