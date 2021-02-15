//!
//! @ The |dvi_four| procedure outputs four bytes in two's complement notation,
//! without risking arithmetic overflow.
//!
//! @p procedure dvi_four(@!x:integer);
//! begin if x>=0 then dvi_out(x div @'100000000)
//! else  begin x:=x+@'10000000000;
//!   x:=x+@'10000000000;
//!   dvi_out((x div @'100000000) + 128);
//!   end;
//! x:=x mod @'100000000; dvi_out(x div @'200000);
//! x:=x mod @'200000; dvi_out(x div @'400);
//! dvi_out(x mod @'400);
//! end;
//!
//! @ A mild optimization of the output is performed by the |dvi_pop|
//! routine, which issues a |pop| unless it is possible to cancel a
//! `|push| |pop|' pair. The parameter to |dvi_pop| is the byte address
//! following the old |push| that matches the new |pop|.
//!
//! @p procedure dvi_pop(@!l:integer);
//! begin if (l=dvi_offset+dvi_ptr)and(dvi_ptr>0) then decr(dvi_ptr)
//! else dvi_out(pop);
//! end;
//!
//! @ Here's a procedure that outputs a font definition. Since \TeX82 uses at
//! most 256 different fonts per job, |fnt_def1| is always used as the command code.
//!
//! @p procedure dvi_font_def(@!f:internal_font_number);
//! var k:pool_pointer; {index into |str_pool|}
//! begin dvi_out(fnt_def1);
//! dvi_out(f-font_base-1);@/
//! dvi_out(qo(font_check[f].b0));
//! dvi_out(qo(font_check[f].b1));
//! dvi_out(qo(font_check[f].b2));
//! dvi_out(qo(font_check[f].b3));@/
//! dvi_four(font_size[f]);
//! dvi_four(font_dsize[f]);@/
//! dvi_out(length(font_area[f]));
//! dvi_out(length(font_name[f]));
//! @<Output the font name whose internal number is |f|@>;
//! end;
//!
//! @ @<Output the font name whose internal number is |f|@>=
//! for k:=str_start[font_area[f]] to str_start[font_area[f]+1]-1 do
//!   dvi_out(so(str_pool[k]));
//! for k:=str_start[font_name[f]] to str_start[font_name[f]+1]-1 do
//!   dvi_out(so(str_pool[k]))
//!
//! @ Versions of \TeX\ intended for small computers might well choose to omit
//! the ideas in the next few parts of this program, since it is not really
//! necessary to optimize the \.{DVI} code by making use of the |w0|, |x0|,
//! |y0|, and |z0| commands. Furthermore, the algorithm that we are about to
//! describe does not pretend to give an optimum reduction in the length
//! of the \.{DVI} code; after all, speed is more important than compactness.
//! But the method is surprisingly effective, and it takes comparatively little
//! time.
//!
//! We can best understand the basic idea by first considering a simpler problem
//! that has the same essential characteristics. Given a sequence of digits,
//! say $3\,1\,4\,1\,5\,9\,2\,6\,5\,3\,5\,8\,9$, we want to assign subscripts
//! $d$, $y$, or $z$ to each digit so as to maximize the number of ``$y$-hits''
//! and ``$z$-hits''; a $y$-hit is an instance of two appearances of the same
//! digit with the subscript $y$, where no $y$'s intervene between the two
//! appearances, and a $z$-hit is defined similarly. For example, the sequence
//! above could be decorated with subscripts as follows:
//! $$3_z\,1_y\,4_d\,1_y\,5_y\,9_d\,2_d\,6_d\,5_y\,3_z\,5_y\,8_d\,9_d.$$
//! There are three $y$-hits ($1_y\ldots1_y$ and $5_y\ldots5_y\ldots5_y$) and
//! one $z$-hit ($3_z\ldots3_z$); there are no $d$-hits, since the two appearances
//! of $9_d$ have $d$'s between them, but we don't count $d$-hits so it doesn't
//! matter how many there are. These subscripts are analogous to the \.{DVI}
//! commands called \\{down}, $y$, and $z$, and the digits are analogous to
//! different amounts of vertical motion; a $y$-hit or $z$-hit corresponds to
//! the opportunity to use the one-byte commands |y0| or |z0| in a \.{DVI} file.
//!
//! \TeX's method of assigning subscripts works like this: Append a new digit,
//! say $\delta$, to the right of the sequence. Now look back through the
//! sequence until one of the following things happens: (a)~You see
//! $\delta_y$ or $\delta_z$, and this was the first time you encountered a
//! $y$ or $z$ subscript, respectively.  Then assign $y$ or $z$ to the new
//! $\delta$; you have scored a hit. (b)~You see $\delta_d$, and no $y$
//! subscripts have been encountered so far during this search.  Then change
//! the previous $\delta_d$ to $\delta_y$ (this corresponds to changing a
//! command in the output buffer), and assign $y$ to the new $\delta$; it's
//! another hit.  (c)~You see $\delta_d$, and a $y$ subscript has been seen
//! but not a $z$.  Change the previous $\delta_d$ to $\delta_z$ and assign
//! $z$ to the new $\delta$. (d)~You encounter both $y$ and $z$ subscripts
//! before encountering a suitable $\delta$, or you scan all the way to the
//! front of the sequence. Assign $d$ to the new $\delta$; this assignment may
//! be changed later.
//!
//! The subscripts $3_z\,1_y\,4_d\ldots\,$ in the example above were, in fact,
//! produced by this procedure, as the reader can verify. (Go ahead and try it.)
//!
//! @ In order to implement such an idea, \TeX\ maintains a stack of pointers
//! to the \\{down}, $y$, and $z$ commands that have been generated for the
//! current page. And there is a similar stack for \\{right}, |w|, and |x|
//! commands. These stacks are called the down stack and right stack, and their
//! top elements are maintained in the variables |down_ptr| and |right_ptr|.
//!
//! Each entry in these stacks contains four fields: The |width| field is
//! the amount of motion down or to the right; the |location| field is the
//! byte number of the \.{DVI} command in question (including the appropriate
//! |dvi_offset|); the |link| field points to the next item below this one
//! on the stack; and the |info| field encodes the options for possible change
//! in the \.{DVI} command.
//!
//! @d movement_node_size=3 {number of words per entry in the down and right stacks}
//! @d location(#)==mem[#+2].int {\.{DVI} byte number for a movement command}
//!
//! @<Glob...@>=
//! @!down_ptr,@!right_ptr:pointer; {heads of the down and right stacks}
//!
//! @ @<Set init...@>=
//! down_ptr:=null; right_ptr:=null;
//!
//! @ Here is a subroutine that produces a \.{DVI} command for some specified
//! downward or rightward motion. It has two parameters: |w| is the amount
//! of motion, and |o| is either |down1| or |right1|. We use the fact that
//! the command codes have convenient arithmetic properties: |y1-down1=w1-right1|
//! and |z1-down1=x1-right1|.
//!
//! @p procedure movement(@!w:scaled;@!o:eight_bits);
//! label exit,found,not_found,2,1;
//! var mstate:small_number; {have we seen a |y| or |z|?}
//! @!p,@!q:pointer; {current and top nodes on the stack}
//! @!k:integer; {index into |dvi_buf|, modulo |dvi_buf_size|}
//! begin q:=get_node(movement_node_size); {new node for the top of the stack}
//! width(q):=w; location(q):=dvi_offset+dvi_ptr;
//! if o=down1 then
//!   begin link(q):=down_ptr; down_ptr:=q;
//!   end
//! else  begin link(q):=right_ptr; right_ptr:=q;
//!   end;
//! @<Look at the other stack entries until deciding what sort of \.{DVI} command
//!   to generate; |goto found| if node |p| is a ``hit''@>;
//! @<Generate a |down| or |right| command for |w| and |return|@>;
//! found: @<Generate a |y0| or |z0| command in order to reuse a previous
//!   appearance of~|w|@>;
//! exit:end;
//!
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
//! @ In case you are wondering when all the movement nodes are removed from
//! \TeX's memory, the answer is that they are recycled just before
//! |hlist_out| and |vlist_out| finish outputting a box. This restores the
//! down and right stacks to the state they were in before the box was output,
//! except that some |info|'s may have become more restrictive.
//!
//! @p procedure prune_movements(@!l:integer);
//!   {delete movement nodes with |location>=l|}
//! label done,exit;
//! var p:pointer; {node being deleted}
//! begin while down_ptr<>null do
//!   begin if location(down_ptr)<l then goto done;
//!   p:=down_ptr; down_ptr:=link(p); free_node(p,movement_node_size);
//!   end;
//! done: while right_ptr<>null do
//!   begin if location(right_ptr)<l then return;
//!   p:=right_ptr; right_ptr:=link(p); free_node(p,movement_node_size);
//!   end;
//! exit:end;
//!
