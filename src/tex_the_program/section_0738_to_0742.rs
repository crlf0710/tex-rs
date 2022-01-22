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
