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
//! @ The actual distances by which we want to move might be computed as the
//! sum of several separate movements. For example, there might be several
//! glue nodes in succession, or we might want to move right by the width of
//! some box plus some amount of glue. More importantly, the baselineskip
//! distances are computed in terms of glue together with the depth and
//! height of adjacent boxes, and we want the \.{DVI} file to lump these
//! three quantities together into a single motion.
//!
//! Therefore, \TeX\ maintains two pairs of global variables: |dvi_h| and |dvi_v|
//! are the |h| and |v| coordinates corresponding to the commands actually
//! output to the \.{DVI} file, while |cur_h| and |cur_v| are the coordinates
//! corresponding to the current state of the output routines. Coordinate
//! changes will accumulate in |cur_h| and |cur_v| without being reflected
//! in the output, until such a change becomes necessary or desirable; we
//! can call the |movement| procedure whenever we want to make |dvi_h=cur_h|
//! or |dvi_v=cur_v|.
//!
//! The current font reflected in the \.{DVI} output is called |dvi_f|;
//! there is no need for a `\\{cur\_f}' variable.
//!
//! The depth of nesting of |hlist_out| and |vlist_out| is called |cur_s|;
//! this is essentially the depth of |push| commands in the \.{DVI} output.
//!
//! @d synch_h==if cur_h<>dvi_h then
//!     begin movement(cur_h-dvi_h,right1); dvi_h:=cur_h;
//!     end
//! @d synch_v==if cur_v<>dvi_v then
//!     begin movement(cur_v-dvi_v,down1); dvi_v:=cur_v;
//!     end
//!
//! @<Glob...@>=
//! @!dvi_h,@!dvi_v:scaled; {a \.{DVI} reader program thinks we are here}
//! @!cur_h,@!cur_v:scaled; {\TeX\ thinks we are here}
//! @!dvi_f:internal_font_number; {the current font}
//! @!cur_s:integer; {current depth of output box nesting, initially $-1$}
//!
//! @ @<Initialize variables as |ship_out| begins@>=
//! dvi_h:=0; dvi_v:=0; cur_h:=h_offset; dvi_f:=null_font;
//! ensure_dvi_open;
//! if total_pages=0 then
//!   begin dvi_out(pre); dvi_out(id_byte); {output the preamble}
//! @^preamble of \.{DVI} file@>
//!   dvi_four(25400000); dvi_four(473628672); {conversion ratio for sp}
//!   prepare_mag; dvi_four(mag); {magnification factor is frozen}
//!   old_setting:=selector; selector:=new_string;
//!   print(" TeX output "); print_int(year); print_char(".");
//!   print_two(month); print_char("."); print_two(day);
//!   print_char(":"); print_two(time div 60);
//!   print_two(time mod 60);
//!   selector:=old_setting; dvi_out(cur_length);
//!   for s:=str_start[str_ptr] to pool_ptr-1 do dvi_out(so(str_pool[s]));
//!   pool_ptr:=str_start[str_ptr]; {flush the current string}
//!   end
//!
//! @ When |hlist_out| is called, its duty is to output the box represented
//! by the |hlist_node| pointed to by |temp_ptr|. The reference point of that
//! box has coordinates |(cur_h,cur_v)|.
//!
//! Similarly, when |vlist_out| is called, its duty is to output the box represented
//! by the |vlist_node| pointed to by |temp_ptr|. The reference point of that
//! box has coordinates |(cur_h,cur_v)|.
//! @^recursion@>
//!
//! @p procedure@?vlist_out; forward; {|hlist_out| and |vlist_out| are mutually
//!   recursive}
//!
//! @ The recursive procedures |hlist_out| and |vlist_out| each have local variables
//! |save_h| and |save_v| to hold the values of |dvi_h| and |dvi_v| just before
//! entering a new level of recursion.  In effect, the values of |save_h| and
//! |save_v| on \TeX's run-time stack correspond to the values of |h| and |v|
//! that a \.{DVI}-reading program will push onto its coordinate stack.
//!
//! @d move_past=13 {go to this label when advancing past glue or a rule}
//! @d fin_rule=14 {go to this label to finish processing a rule}
//! @d next_p=15 {go to this label when finished with node |p|}
//!
//! @p @t\4@>@<Declare procedures needed in |hlist_out|, |vlist_out|@>@t@>@/
//! procedure hlist_out; {output an |hlist_node| box}
//! label reswitch, move_past, fin_rule, next_p;
//! var base_line: scaled; {the baseline coordinate for this box}
//! @!left_edge: scaled; {the left coordinate for this box}
//! @!save_h,@!save_v: scaled; {what |dvi_h| and |dvi_v| should pop to}
//! @!this_box: pointer; {pointer to containing box}
//! @!g_order: glue_ord; {applicable order of infinity for glue}
//! @!g_sign: normal..shrinking; {selects type of glue}
//! @!p:pointer; {current position in the hlist}
//! @!save_loc:integer; {\.{DVI} byte location upon entry}
//! @!leader_box:pointer; {the leader box being replicated}
//! @!leader_wd:scaled; {width of leader box being replicated}
//! @!lx:scaled; {extra space between leader boxes}
//! @!outer_doing_leaders:boolean; {were we doing leaders?}
//! @!edge:scaled; {left edge of sub-box, or right edge of leader space}
//! @!glue_temp:real; {glue value before rounding}
//! @!cur_glue:real; {glue seen so far}
//! @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
//! begin cur_g:=0; cur_glue:=float_constant(0);
//! this_box:=temp_ptr; g_order:=glue_order(this_box);
//! g_sign:=glue_sign(this_box); p:=list_ptr(this_box);
//! incr(cur_s);
//! if cur_s>0 then dvi_out(push);
//! if cur_s>max_push then max_push:=cur_s;
//! save_loc:=dvi_offset+dvi_ptr; base_line:=cur_v; left_edge:=cur_h;
//! while p<>null do @<Output node |p| for |hlist_out| and move to the next node,
//!   maintaining the condition |cur_v=base_line|@>;
//! prune_movements(save_loc);
//! if cur_s>0 then dvi_pop(save_loc);
//! decr(cur_s);
//! end;
//!
//! @ We ought to give special care to the efficiency of one part of |hlist_out|,
//! since it belongs to \TeX's inner loop. When a |char_node| is encountered,
//! we save a little time by processing several nodes in succession until
//! reaching a non-|char_node|. The program uses the fact that |set_char_0=0|.
//! @^inner loop@>
//!
//! @<Output node |p| for |hlist_out|...@>=
//! reswitch: if is_char_node(p) then
//!   begin synch_h; synch_v;
//!   repeat f:=font(p); c:=character(p);
//!   if f<>dvi_f then @<Change font |dvi_f| to |f|@>;
//!   if c>=qi(128) then dvi_out(set1);
//!   dvi_out(qo(c));@/
//!   cur_h:=cur_h+char_width(f)(char_info(f)(c));
//!   p:=link(p);
//!   until not is_char_node(p);
//!   dvi_h:=cur_h;
//!   end
//! else @<Output the non-|char_node| |p| for |hlist_out|
//!     and move to the next node@>
//!
//! @ @<Change font |dvi_f| to |f|@>=
//! begin if not font_used[f] then
//!   begin dvi_font_def(f); font_used[f]:=true;
//!   end;
//! if f<=64+font_base then dvi_out(f-font_base-1+fnt_num_0)
//! else  begin dvi_out(fnt1); dvi_out(f-font_base-1);
//!   end;
//! dvi_f:=f;
//! end
//!
//! @ @<Output the non-|char_node| |p| for |hlist_out|...@>=
//! begin case type(p) of
//! hlist_node,vlist_node:@<Output a box in an hlist@>;
//! rule_node: begin rule_ht:=height(p); rule_dp:=depth(p); rule_wd:=width(p);
//!   goto fin_rule;
//!   end;
//! whatsit_node: @<Output the whatsit node |p| in an hlist@>;
//! glue_node: @<Move right or output leaders@>;
//! kern_node,math_node:cur_h:=cur_h+width(p);
//! ligature_node: @<Make node |p| look like a |char_node| and |goto reswitch|@>;
//! othercases do_nothing
//! endcases;@/
//! goto next_p;
//! fin_rule: @<Output a rule in an hlist@>;
//! move_past: cur_h:=cur_h+rule_wd;
//! next_p:p:=link(p);
//! end
//!
//! @ @<Output a box in an hlist@>=
//! if list_ptr(p)=null then cur_h:=cur_h+width(p)
//! else  begin save_h:=dvi_h; save_v:=dvi_v;
//!   cur_v:=base_line+shift_amount(p); {shift the box down}
//!   temp_ptr:=p; edge:=cur_h;
//!   if type(p)=vlist_node then vlist_out@+else hlist_out;
//!   dvi_h:=save_h; dvi_v:=save_v;
//!   cur_h:=edge+width(p); cur_v:=base_line;
//!   end
//!
//! @ @<Output a rule in an hlist@>=
//! if is_running(rule_ht) then rule_ht:=height(this_box);
//! if is_running(rule_dp) then rule_dp:=depth(this_box);
//! rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
//! if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
//!   begin synch_h; cur_v:=base_line+rule_dp; synch_v;
//!   dvi_out(set_rule); dvi_four(rule_ht); dvi_four(rule_wd);
//!   cur_v:=base_line; dvi_h:=dvi_h+rule_wd;
//!   end
//!
//! @ @d billion==float_constant(1000000000)
//! @d vet_glue(#)== glue_temp:=#;
//!   if glue_temp>billion then
//!            glue_temp:=billion
//!   else if glue_temp<-billion then
//!            glue_temp:=-billion
//!
//! @<Move right or output leaders@>=
//! begin g:=glue_ptr(p); rule_wd:=width(g)-cur_g;
//! if g_sign<>normal then
//!   begin if g_sign=stretching then
//!     begin if stretch_order(g)=g_order then
//!       begin cur_glue:=cur_glue+stretch(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//! @^real multiplication@>
//!       cur_g:=round(glue_temp);
//!       end;
//!     end
//!   else if shrink_order(g)=g_order then
//!       begin cur_glue:=cur_glue-shrink(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//!       cur_g:=round(glue_temp);
//!       end;
//!   end;
//! rule_wd:=rule_wd+cur_g;
//! if subtype(p)>=a_leaders then
//!   @<Output leaders in an hlist, |goto fin_rule| if a rule
//!     or to |next_p| if done@>;
//! goto move_past;
//! end
//!
//! @ @<Output leaders in an hlist...@>=
//! begin leader_box:=leader_ptr(p);
//! if type(leader_box)=rule_node then
//!   begin rule_ht:=height(leader_box); rule_dp:=depth(leader_box);
//!   goto fin_rule;
//!   end;
//! leader_wd:=width(leader_box);
//! if (leader_wd>0)and(rule_wd>0) then
//!   begin rule_wd:=rule_wd+10; {compensate for floating-point rounding}
//!   edge:=cur_h+rule_wd; lx:=0;
//!   @<Let |cur_h| be the position of the first box, and set |leader_wd+lx|
//!     to the spacing between corresponding parts of boxes@>;
//!   while cur_h+leader_wd<=edge do
//!     @<Output a leader box at |cur_h|,
//!       then advance |cur_h| by |leader_wd+lx|@>;
//!   cur_h:=edge-10; goto next_p;
//!   end;
//! end
//!
//! @ The calculations related to leaders require a bit of care. First, in the
//! case of |a_leaders| (aligned leaders), we want to move |cur_h| to
//! |left_edge| plus the smallest multiple of |leader_wd| for which the result
//! is not less than the current value of |cur_h|; i.e., |cur_h| should become
//! $|left_edge|+|leader_wd|\times\lceil
//! (|cur_h|-|left_edge|)/|leader_wd|\rceil$.  The program here should work in
//! all cases even though some implementations of \PASCAL\ give nonstandard
//! results for the |div| operation when |cur_h| is less than |left_edge|.
//!
//! In the case of |c_leaders| (centered leaders), we want to increase |cur_h|
//! by half of the excess space not occupied by the leaders; and in the
//! case of |x_leaders| (expanded leaders) we increase |cur_h|
//! by $1/(q+1)$ of this excess space, where $q$ is the number of times the
//! leader box will be replicated. Slight inaccuracies in the division might
//! accumulate; half of this rounding error is placed at each end of the leaders.
//!
//! @<Let |cur_h| be the position of the first box, ...@>=
//! if subtype(p)=a_leaders then
//!   begin save_h:=cur_h;
//!   cur_h:=left_edge+leader_wd*((cur_h-left_edge)@!div leader_wd);
//!   if cur_h<save_h then cur_h:=cur_h+leader_wd;
//!   end
//! else  begin lq:=rule_wd div leader_wd; {the number of box copies}
//!   lr:=rule_wd mod leader_wd; {the remaining space}
//!   if subtype(p)=c_leaders then cur_h:=cur_h+(lr div 2)
//!   else  begin lx:=lr div (lq+1);
//!     cur_h:=cur_h+((lr-(lq-1)*lx) div 2);
//!     end;
//!   end
//!
//! @ The `\\{synch}' operations here are intended to decrease the number of
//! bytes needed to specify horizontal and vertical motion in the \.{DVI} output.
//!
//! @<Output a leader box at |cur_h|, ...@>=
//! begin cur_v:=base_line+shift_amount(leader_box); synch_v; save_v:=dvi_v;@/
//! synch_h; save_h:=dvi_h; temp_ptr:=leader_box;
//! outer_doing_leaders:=doing_leaders; doing_leaders:=true;
//! if type(leader_box)=vlist_node then vlist_out@+else hlist_out;
//! doing_leaders:=outer_doing_leaders;
//! dvi_v:=save_v; dvi_h:=save_h; cur_v:=base_line;
//! cur_h:=save_h+leader_wd+lx;
//! end
//!
//! @ The |vlist_out| routine is similar to |hlist_out|, but a bit simpler.
//!
//! @p procedure vlist_out; {output a |vlist_node| box}
//! label move_past, fin_rule, next_p;
//! var left_edge: scaled; {the left coordinate for this box}
//! @!top_edge: scaled; {the top coordinate for this box}
//! @!save_h,@!save_v: scaled; {what |dvi_h| and |dvi_v| should pop to}
//! @!this_box: pointer; {pointer to containing box}
//! @!g_order: glue_ord; {applicable order of infinity for glue}
//! @!g_sign: normal..shrinking; {selects type of glue}
//! @!p:pointer; {current position in the vlist}
//! @!save_loc:integer; {\.{DVI} byte location upon entry}
//! @!leader_box:pointer; {the leader box being replicated}
//! @!leader_ht:scaled; {height of leader box being replicated}
//! @!lx:scaled; {extra space between leader boxes}
//! @!outer_doing_leaders:boolean; {were we doing leaders?}
//! @!edge:scaled; {bottom boundary of leader space}
//! @!glue_temp:real; {glue value before rounding}
//! @!cur_glue:real; {glue seen so far}
//! @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
//! begin cur_g:=0; cur_glue:=float_constant(0);
//! this_box:=temp_ptr; g_order:=glue_order(this_box);
//! g_sign:=glue_sign(this_box); p:=list_ptr(this_box);
//! incr(cur_s);
//! if cur_s>0 then dvi_out(push);
//! if cur_s>max_push then max_push:=cur_s;
//! save_loc:=dvi_offset+dvi_ptr; left_edge:=cur_h; cur_v:=cur_v-height(this_box);
//! top_edge:=cur_v;
//! while p<>null do @<Output node |p| for |vlist_out| and move to the next node,
//!   maintaining the condition |cur_h=left_edge|@>;
//! prune_movements(save_loc);
//! if cur_s>0 then dvi_pop(save_loc);
//! decr(cur_s);
//! end;
//!
//! @ @<Output node |p| for |vlist_out|...@>=
//! begin if is_char_node(p) then confusion("vlistout")
//! @:this can't happen vlistout}{\quad vlistout@>
//! else @<Output the non-|char_node| |p| for |vlist_out|@>;
//! next_p:p:=link(p);
//! end
//!
//! @ @<Output the non-|char_node| |p| for |vlist_out|@>=
//! begin case type(p) of
//! hlist_node,vlist_node:@<Output a box in a vlist@>;
//! rule_node: begin rule_ht:=height(p); rule_dp:=depth(p); rule_wd:=width(p);
//!   goto fin_rule;
//!   end;
//! whatsit_node: @<Output the whatsit node |p| in a vlist@>;
//! glue_node: @<Move down or output leaders@>;
//! kern_node:cur_v:=cur_v+width(p);
//! othercases do_nothing
//! endcases;@/
//! goto next_p;
//! fin_rule: @<Output a rule in a vlist, |goto next_p|@>;
//! move_past: cur_v:=cur_v+rule_ht;
//! end
//!
//! @ The |synch_v| here allows the \.{DVI} output to use one-byte commands
//! for adjusting |v| in most cases, since the baselineskip distance will
//! usually be constant.
//!
//! @<Output a box in a vlist@>=
//! if list_ptr(p)=null then cur_v:=cur_v+height(p)+depth(p)
//! else  begin cur_v:=cur_v+height(p); synch_v;
//!   save_h:=dvi_h; save_v:=dvi_v;
//!   cur_h:=left_edge+shift_amount(p); {shift the box right}
//!   temp_ptr:=p;
//!   if type(p)=vlist_node then vlist_out@+else hlist_out;
//!   dvi_h:=save_h; dvi_v:=save_v;
//!   cur_v:=save_v+depth(p); cur_h:=left_edge;
//!   end
//!
//! @ @<Output a rule in a vlist...@>=
//! if is_running(rule_wd) then rule_wd:=width(this_box);
//! rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
//! cur_v:=cur_v+rule_ht;
//! if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
//!   begin synch_h; synch_v;
//!   dvi_out(put_rule); dvi_four(rule_ht); dvi_four(rule_wd);
//!   end;
//! goto next_p
//!
//! @ @<Move down or output leaders@>=
//! begin g:=glue_ptr(p); rule_ht:=width(g)-cur_g;
//! if g_sign<>normal then
//!   begin if g_sign=stretching then
//!     begin if stretch_order(g)=g_order then
//!       begin cur_glue:=cur_glue+stretch(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//! @^real multiplication@>
//!       cur_g:=round(glue_temp);
//!       end;
//!     end
//!   else if shrink_order(g)=g_order then
//!       begin cur_glue:=cur_glue-shrink(g);
//!       vet_glue(float(glue_set(this_box))*cur_glue);
//!       cur_g:=round(glue_temp);
//!       end;
//!   end;
//! rule_ht:=rule_ht+cur_g;
//! if subtype(p)>=a_leaders then
//!   @<Output leaders in a vlist, |goto fin_rule| if a rule
//!     or to |next_p| if done@>;
//! goto move_past;
//! end
//!
//! @ @<Output leaders in a vlist...@>=
//! begin leader_box:=leader_ptr(p);
//! if type(leader_box)=rule_node then
//!   begin rule_wd:=width(leader_box); rule_dp:=0;
//!   goto fin_rule;
//!   end;
//! leader_ht:=height(leader_box)+depth(leader_box);
//! if (leader_ht>0)and(rule_ht>0) then
//!   begin rule_ht:=rule_ht+10; {compensate for floating-point rounding}
//!   edge:=cur_v+rule_ht; lx:=0;
//!   @<Let |cur_v| be the position of the first box, and set |leader_ht+lx|
//!     to the spacing between corresponding parts of boxes@>;
//!   while cur_v+leader_ht<=edge do
//!     @<Output a leader box at |cur_v|,
//!       then advance |cur_v| by |leader_ht+lx|@>;
//!   cur_v:=edge-10; goto next_p;
//!   end;
//! end
//!
//! @ @<Let |cur_v| be the position of the first box, ...@>=
//! if subtype(p)=a_leaders then
//!   begin save_v:=cur_v;
//!   cur_v:=top_edge+leader_ht*((cur_v-top_edge)@!div leader_ht);
//!   if cur_v<save_v then cur_v:=cur_v+leader_ht;
//!   end
//! else  begin lq:=rule_ht div leader_ht; {the number of box copies}
//!   lr:=rule_ht mod leader_ht; {the remaining space}
//!   if subtype(p)=c_leaders then cur_v:=cur_v+(lr div 2)
//!   else  begin lx:=lr div (lq+1);
//!     cur_v:=cur_v+((lr-(lq-1)*lx) div 2);
//!     end;
//!   end
//!
//! @ When we reach this part of the program, |cur_v| indicates the top of a
//! leader box, not its baseline.
//!
//! @<Output a leader box at |cur_v|, ...@>=
//! begin cur_h:=left_edge+shift_amount(leader_box); synch_h; save_h:=dvi_h;@/
//! cur_v:=cur_v+height(leader_box); synch_v; save_v:=dvi_v;
//! temp_ptr:=leader_box;
//! outer_doing_leaders:=doing_leaders; doing_leaders:=true;
//! if type(leader_box)=vlist_node then vlist_out@+else hlist_out;
//! doing_leaders:=outer_doing_leaders;
//! dvi_v:=save_v; dvi_h:=save_h; cur_h:=left_edge;
//! cur_v:=save_v-height(leader_box)+leader_ht+lx;
//! end
//!
//! @ The |hlist_out| and |vlist_out| procedures are now complete, so we are
//! ready for the |ship_out| routine that gets them started in the first place.
//!
//! @p procedure ship_out(@!p:pointer); {output the box |p|}
//! label done;
//! var page_loc:integer; {location of the current |bop|}
//! @!j,@!k:0..9; {indices to first ten count registers}
//! @!s:pool_pointer; {index into |str_pool|}
//! @!old_setting:0..max_selector; {saved |selector| setting}
//! begin if tracing_output>0 then
//!   begin print_nl(""); print_ln;
//!   print("Completed box being shipped out");
//! @.Completed box...@>
//!   end;
//! if term_offset>max_print_line-9 then print_ln
//! else if (term_offset>0)or(file_offset>0) then print_char(" ");
//! print_char("["); j:=9;
//! while (count(j)=0)and(j>0) do decr(j);
//! for k:=0 to j do
//!   begin print_int(count(k));
//!   if k<j then print_char(".");
//!   end;
//! update_terminal;
//! if tracing_output>0 then
//!   begin print_char("]");
//!   begin_diagnostic; show_box(p); end_diagnostic(true);
//!   end;
//! @<Ship box |p| out@>;
//! if tracing_output<=0 then print_char("]");
//! dead_cycles:=0;
//! update_terminal; {progress report}
//! @<Flush the box from memory, showing statistics if requested@>;
//! end;
//!
//! @ @<Flush the box from memory, showing statistics if requested@>=
//! @!stat if tracing_stats>1 then
//!   begin print_nl("Memory usage before: ");
//! @.Memory usage...@>
//!   print_int(var_used); print_char("&");
//!   print_int(dyn_used); print_char(";");
//!   end;
//! tats@/
//! flush_node_list(p);
//! @!stat if tracing_stats>1 then
//!   begin print(" after: ");
//!   print_int(var_used); print_char("&");
//!   print_int(dyn_used); print("; still untouched: ");
//!   print_int(hi_mem_min-lo_mem_max-1); print_ln;
//!   end;
//! tats
//!
//! @ @<Ship box |p| out@>=
//! @<Update the values of |max_h| and |max_v|; but if the page is too large,
//!   |goto done|@>;
//! @<Initialize variables as |ship_out| begins@>;
//! page_loc:=dvi_offset+dvi_ptr;
//! dvi_out(bop);
//! for k:=0 to 9 do dvi_four(count(k));
//! dvi_four(last_bop); last_bop:=page_loc;
//! cur_v:=height(p)+v_offset; temp_ptr:=p;
//! if type(p)=vlist_node then vlist_out@+else hlist_out;
//! dvi_out(eop); incr(total_pages); cur_s:=-1;
//! done:
//!
//! @ Sometimes the user will generate a huge page because other error messages
//! are being ignored. Such pages are not output to the \.{dvi} file, since they
//! may confuse the printing software.
//!
//! @<Update the values of |max_h| and |max_v|; but if the page is too large...@>=
//! if (height(p)>max_dimen)or@|(depth(p)>max_dimen)or@|
//!    (height(p)+depth(p)+v_offset>max_dimen)or@|
//!    (width(p)+h_offset>max_dimen) then
//!   begin print_err("Huge page cannot be shipped out");
//! @.Huge page...@>
//!   help2("The page just created is more than 18 feet tall or")@/
//!    ("more than 18 feet wide, so I suspect something went wrong.");
//!   error;
//!   if tracing_output<=0 then
//!     begin begin_diagnostic;
//!     print_nl("The following box has been deleted:");
//! @.The following...deleted@>
//!     show_box(p);
//!     end_diagnostic(true);
//!     end;
//!   goto done;
//!   end;
//! if height(p)+depth(p)+v_offset>max_v then max_v:=height(p)+depth(p)+v_offset;
//! if width(p)+h_offset>max_h then max_h:=width(p)+h_offset
//!
