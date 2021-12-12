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
