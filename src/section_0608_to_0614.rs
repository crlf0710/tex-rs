//! @ The |info| fields in the entries of the down stack or the right stack
//! have six possible settings: |y_here| or |z_here| mean that the \.{DVI}
//! command refers to |y| or |z|, respectively (or to |w| or |x|, in the
//! case of horizontal motion); |yz_OK| means that the \.{DVI} command is
//! \\{down} (or \\{right}) but can be changed to either |y| or |z| (or
//! to either |w| or |x|); |y_OK| means that it is \\{down} and can be changed
//! to |y| but not |z|; |z_OK| is similar; and |d_fixed| means it must stay
//! \\{down}.
//!
//! The four settings |yz_OK|, |y_OK|, |z_OK|, |d_fixed| would not need to
//! be distinguished from each other if we were simply solving the
//! digit-subscripting problem mentioned above. But in \TeX's case there is
//! a complication because of the nested structure of |push| and |pop|
//! commands. Suppose we add parentheses to the digit-subscripting problem,
//! redefining hits so that $\delta_y\ldots \delta_y$ is a hit if all $y$'s between
//! the $\delta$'s are enclosed in properly nested parentheses, and if the
//! parenthesis level of the right-hand $\delta_y$ is deeper than or equal to
//! that of the left-hand one. Thus, `(' and `)' correspond to `|push|'
//! and `|pop|'. Now if we want to assign a subscript to the final 1 in the
//! sequence
//! $$2_y\,7_d\,1_d\,(\,8_z\,2_y\,8_z\,)\,1$$
//! we cannot change the previous $1_d$ to $1_y$, since that would invalidate
//! the $2_y\ldots2_y$ hit. But we can change it to $1_z$, scoring a hit
//! since the intervening $8_z$'s are enclosed in parentheses.
//!
//! The program below removes movement nodes that are introduced after a |push|,
//! before it outputs the corresponding |pop|.
//!
//! @d y_here=1 {|info| when the movement entry points to a |y| command}
//! @d z_here=2 {|info| when the movement entry points to a |z| command}
//! @d yz_OK=3 {|info| corresponding to an unconstrained \\{down} command}
//! @d y_OK=4 {|info| corresponding to a \\{down} that can't become a |z|}
//! @d z_OK=5 {|info| corresponding to a \\{down} that can't become a |y|}
//! @d d_fixed=6 {|info| corresponding to a \\{down} that can't change}
//!
//! @ When the |movement| procedure gets to the label |found|, the value of
//! |info(p)| will be either |y_here| or |z_here|. If it is, say, |y_here|,
//! the procedure generates a |y0| command (or a |w0| command), and marks
//! all |info| fields between |q| and |p| so that |y| is not OK in that range.
//!
//! @<Generate a |y0| or |z0| command...@>=
//! info(q):=info(p);
//! if info(q)=y_here then
//!   begin dvi_out(o+y0-down1); {|y0| or |w0|}
//!   while link(q)<>p do
//!     begin q:=link(q);
//!     case info(q) of
//!     yz_OK: info(q):=z_OK;
//!     y_OK: info(q):=d_fixed;
//!     othercases do_nothing
//!     endcases;
//!     end;
//!   end
//! else  begin dvi_out(o+z0-down1); {|z0| or |x0|}
//!   while link(q)<>p do
//!     begin q:=link(q);
//!     case info(q) of
//!     yz_OK: info(q):=y_OK;
//!     z_OK: info(q):=d_fixed;
//!     othercases do_nothing
//!     endcases;
//!     end;
//!   end
//!
//! @ @<Generate a |down| or |right|...@>=
//! info(q):=yz_OK;
//! if abs(w)>=@'40000000 then
//!   begin dvi_out(o+3); {|down4| or |right4|}
//!   dvi_four(w); return;
//!   end;
//! if abs(w)>=@'100000 then
//!   begin dvi_out(o+2); {|down3| or |right3|}
//!   if w<0 then w:=w+@'100000000;
//!   dvi_out(w div @'200000); w:=w mod @'200000; goto 2;
//!   end;
//! if abs(w)>=@'200 then
//!   begin dvi_out(o+1); {|down2| or |right2|}
//!   if w<0 then w:=w+@'200000;
//!   goto 2;
//!   end;
//! dvi_out(o); {|down1| or |right1|}
//! if w<0 then w:=w+@'400;
//! goto 1;
//! 2: dvi_out(w div @'400);
//! 1: dvi_out(w mod @'400); return
//!
//! @ As we search through the stack, we are in one of three states,
//! |y_seen|, |z_seen|, or |none_seen|, depending on whether we have
//! encountered |y_here| or |z_here| nodes. These states are encoded as
//! multiples of 6, so that they can be added to the |info| fields for quick
//! decision-making.
//! @^inner loop@>
//!
//! @d none_seen=0 {no |y_here| or |z_here| nodes have been encountered yet}
//! @d y_seen=6 {we have seen |y_here| but not |z_here|}
//! @d z_seen=12 {we have seen |z_here| but not |y_here|}
//!
//! @<Look at the other stack entries until deciding...@>=
//! p:=link(q); mstate:=none_seen;
//! while p<>null do
//!   begin if width(p)=w then @<Consider a node with matching width;
//!     |goto found| if it's a hit@>
//!   else  case mstate+info(p) of
//!     none_seen+y_here: mstate:=y_seen;
//!     none_seen+z_here: mstate:=z_seen;
//!     y_seen+z_here,z_seen+y_here: goto not_found;
//!     othercases do_nothing
//!     endcases;
//!   p:=link(p);
//!   end;
//! not_found:
//!
//! @ We might find a valid hit in a |y| or |z| byte that is already gone
//! from the buffer. But we can't change bytes that are gone forever; ``the
//! moving finger writes, $\ldots\,\,$.''
//!
//! @<Consider a node with matching width...@>=
//! case mstate+info(p) of
//! none_seen+yz_OK,none_seen+y_OK,z_seen+yz_OK,z_seen+y_OK:@t@>@;@/
//!   if location(p)<dvi_gone then goto not_found
//!   else @<Change buffered instruction to |y| or |w| and |goto found|@>;
//! none_seen+z_OK,y_seen+yz_OK,y_seen+z_OK:@t@>@;@/
//!   if location(p)<dvi_gone then goto not_found
//!   else @<Change buffered instruction to |z| or |x| and |goto found|@>;
//! none_seen+y_here,none_seen+z_here,y_seen+z_here,z_seen+y_here: goto found;
//! othercases do_nothing
//! endcases
//!
//! @ @<Change buffered instruction to |y| or |w| and |goto found|@>=
//! begin k:=location(p)-dvi_offset;
//! if k<0 then k:=k+dvi_buf_size;
//! dvi_buf[k]:=dvi_buf[k]+y1-down1;
//! info(p):=y_here; goto found;
//! end
//!
//! @ @<Change buffered instruction to |z| or |x| and |goto found|@>=
//! begin k:=location(p)-dvi_offset;
//! if k<0 then k:=k+dvi_buf_size;
//! dvi_buf[k]:=dvi_buf[k]+z1-down1;
//! info(p):=z_here; goto found;
//! end
//!
