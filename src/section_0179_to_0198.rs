//! @ We also need to declare some procedures that appear later in this
//! documentation.
//!
//! @p @<Declare procedures needed for displaying the elements of mlists@>@;
//! @<Declare the procedure called |print_skip_param|@>
//!
//! @ Since boxes can be inside of boxes, |show_node_list| is inherently recursive,
//! @^recursion@>
//! up to a given maximum number of levels.  The history of nesting is indicated
//! by the current string, which will be printed at the beginning of each line;
//! the length of this string, namely |cur_length|, is the depth of nesting.
//!
//! Recursive calls on |show_node_list| therefore use the following pattern:
//!
//! @d node_list_display(#)==
//!   begin append_char("."); show_node_list(#); flush_char;
//!   end {|str_room| need not be checked; see |show_box| below}
//!
//! @ A global variable called |depth_threshold| is used to record the maximum
//! depth of nesting for which |show_node_list| will show information.  If we
//! have |depth_threshold=0|, for example, only the top level information will
//! be given and no sublists will be traversed. Another global variable, called
//! |breadth_max|, tells the maximum number of items to show at each level;
//! |breadth_max| had better be positive, or you won't see anything.
//!
//! @<Glob...@>=
//! @!depth_threshold : integer; {maximum nesting depth in box displays}
//! @!breadth_max : integer; {maximum number of items shown at the same list level}
//!
//! @ Now we are ready for |show_node_list| itself. This procedure has been
//! written to be ``extra robust'' in the sense that it should not crash or get
//! into a loop even if the data structures have been messed up by bugs in
//! the rest of the program. You can safely call its parent routine
//! |show_box(p)| for arbitrary values of |p| when you are debugging \TeX.
//! However, in the presence of bad data, the procedure may
//! @^dirty \PASCAL@>@^debugging@>
//! fetch a |memory_word| whose variant is different from the way it was stored;
//! for example, it might try to read |mem[p].hh| when |mem[p]|
//! contains a scaled integer, if |p| is a pointer that has been
//! clobbered or chosen at random.
//!
//! @p procedure show_node_list(@!p:integer); {prints a node list symbolically}
//! label exit;
//! var n:integer; {the number of items already printed at this level}
//! @!g:real; {a glue ratio, as a floating point number}
//! begin if cur_length>depth_threshold then
//!   begin if p>null then print(" []");
//!     {indicate that there's been some truncation}
//!   return;
//!   end;
//! n:=0;
//! while p>mem_min do
//!   begin print_ln; print_current_string; {display the nesting history}
//!   if p>mem_end then {pointer out of range}
//!     begin print("Bad link, display aborted."); return;
//! @.Bad link...@>
//!     end;
//!   incr(n); if n>breadth_max then {time to stop}
//!     begin print("etc."); return;
//! @.etc@>
//!     end;
//!   @<Display node |p|@>;
//!   p:=link(p);
//!   end;
//! exit:
//! end;
//!
//! @ @<Display node |p|@>=
//! if is_char_node(p) then print_font_and_char(p)
//! else  case type(p) of
//!   hlist_node,vlist_node,unset_node: @<Display box |p|@>;
//!   rule_node: @<Display rule |p|@>;
//!   ins_node: @<Display insertion |p|@>;
//!   whatsit_node: @<Display the whatsit node |p|@>;
//!   glue_node: @<Display glue |p|@>;
//!   kern_node: @<Display kern |p|@>;
//!   math_node: @<Display math node |p|@>;
//!   ligature_node: @<Display ligature |p|@>;
//!   penalty_node: @<Display penalty |p|@>;
//!   disc_node: @<Display discretionary |p|@>;
//!   mark_node: @<Display mark |p|@>;
//!   adjust_node: @<Display adjustment |p|@>;
//!   @t\4@>@<Cases of |show_node_list| that arise in mlists only@>@;
//!   othercases print("Unknown node type!")
//!   endcases
//!
//! @ @<Display box |p|@>=
//! begin if type(p)=hlist_node then print_esc("h")
//! else if type(p)=vlist_node then print_esc("v")
//! else print_esc("unset");
//! print("box("); print_scaled(height(p)); print_char("+");
//! print_scaled(depth(p)); print(")x"); print_scaled(width(p));
//! if type(p)=unset_node then
//!   @<Display special fields of the unset node |p|@>
//! else  begin @<Display the value of |glue_set(p)|@>;
//!   if shift_amount(p)<>0 then
//!     begin print(", shifted "); print_scaled(shift_amount(p));
//!     end;
//!   end;
//! node_list_display(list_ptr(p)); {recursive call}
//! end
//!
//! @ @<Display special fields of the unset node |p|@>=
//! begin if span_count(p)<>min_quarterword then
//!   begin print(" ("); print_int(qo(span_count(p))+1);
//!   print(" columns)");
//!   end;
//! if glue_stretch(p)<>0 then
//!   begin print(", stretch "); print_glue(glue_stretch(p),glue_order(p),0);
//!   end;
//! if glue_shrink(p)<>0 then
//!   begin print(", shrink "); print_glue(glue_shrink(p),glue_sign(p),0);
//!   end;
//! end
//!
//! @ The code will have to change in this place if |glue_ratio| is
//! a structured type instead of an ordinary |real|. Note that this routine
//! should avoid arithmetic errors even if the |glue_set| field holds an
//! arbitrary random value. The following code assumes that a properly
//! formed nonzero |real| number has absolute value $2^{20}$ or more when
//! it is regarded as an integer; this precaution was adequate to prevent
//! floating point underflow on the author's computer.
//! @^system dependencies@>
//! @^dirty \PASCAL@>
//!
//! @<Display the value of |glue_set(p)|@>=
//! g:=float(glue_set(p));
//! if (g<>float_constant(0))and(glue_sign(p)<>normal) then
//!   begin print(", glue set ");
//!   if glue_sign(p)=shrinking then print("- ");
//!   if abs(mem[p+glue_offset].int)<@'4000000 then print("?.?")
//!   else if abs(g)>float_constant(20000) then
//!     begin if g>float_constant(0) then print_char(">")
//!     else print("< -");
//!     print_glue(20000*unity,glue_order(p),0);
//!     end
//!   else print_glue(round(unity*g),glue_order(p),0);
//! @^real multiplication@>
//!   end
//!
//! @ @<Display rule |p|@>=
//! begin print_esc("rule("); print_rule_dimen(height(p)); print_char("+");
//! print_rule_dimen(depth(p)); print(")x"); print_rule_dimen(width(p));
//! end
//!
//! @ @<Display insertion |p|@>=
//! begin print_esc("insert"); print_int(qo(subtype(p)));
//! print(", natural size "); print_scaled(height(p));
//! print("; split("); print_spec(split_top_ptr(p),0);
//! print_char(","); print_scaled(depth(p));
//! print("); float cost "); print_int(float_cost(p));
//! node_list_display(ins_ptr(p)); {recursive call}
//! end
//!
//! @ @<Display glue |p|@>=
//! if subtype(p)>=a_leaders then @<Display leaders |p|@>
//! else  begin print_esc("glue");
//!   if subtype(p)<>normal then
//!     begin print_char("(");
//!     if subtype(p)<cond_math_glue then
//!       print_skip_param(subtype(p)-1)
//!     else if subtype(p)=cond_math_glue then print_esc("nonscript")
//!     else print_esc("mskip");
//!     print_char(")");
//!     end;
//!   if subtype(p)<>cond_math_glue then
//!     begin print_char(" ");
//!     if subtype(p)<cond_math_glue then print_spec(glue_ptr(p),0)
//!     else print_spec(glue_ptr(p),"mu");
//!     end;
//!   end
//!
//! @ @<Display leaders |p|@>=
//! begin print_esc("");
//! if subtype(p)=c_leaders then print_char("c")
//! else if subtype(p)=x_leaders then print_char("x");
//! print("leaders "); print_spec(glue_ptr(p),0);
//! node_list_display(leader_ptr(p)); {recursive call}
//! end
//!
//! @ An ``explicit'' kern value is indicated implicitly by an explicit space.
//!
//! @<Display kern |p|@>=
//! if subtype(p)<>mu_glue then
//!   begin print_esc("kern");
//!   if subtype(p)<>normal then print_char(" ");
//!   print_scaled(width(p));
//!   if subtype(p)=acc_kern then print(" (for accent)");
//! @.for accent@>
//!   end
//! else  begin print_esc("mkern"); print_scaled(width(p)); print("mu");
//!   end
//!
//! @ @<Display math node |p|@>=
//! begin print_esc("math");
//! if subtype(p)=before then print("on")
//! else print("off");
//! if width(p)<>0 then
//!   begin print(", surrounded "); print_scaled(width(p));
//!   end;
//! end
//!
//! @ @<Display ligature |p|@>=
//! begin print_font_and_char(lig_char(p)); print(" (ligature ");
//! if subtype(p)>1 then print_char("|");
//! font_in_short_display:=font(lig_char(p)); short_display(lig_ptr(p));
//! if odd(subtype(p)) then print_char("|");
//! print_char(")");
//! end
//!
//! @ @<Display penalty |p|@>=
//! begin print_esc("penalty "); print_int(penalty(p));
//! end
//!
//! @ The |post_break| list of a discretionary node is indicated by a prefixed
//! `\.{\char'174}' instead of the `\..' before the |pre_break| list.
//!
//! @<Display discretionary |p|@>=
//! begin print_esc("discretionary");
//! if replace_count(p)>0 then
//!   begin print(" replacing "); print_int(replace_count(p));
//!   end;
//! node_list_display(pre_break(p)); {recursive call}
//! append_char("|"); show_node_list(post_break(p)); flush_char; {recursive call}
//! end
//!
//! @ @<Display mark |p|@>=
//! begin print_esc("mark"); print_mark(mark_ptr(p));
//! end
//!
//! @ @<Display adjustment |p|@>=
//! begin print_esc("vadjust"); node_list_display(adjust_ptr(p)); {recursive call}
//! end
//!
//! @ The recursive machinery is started by calling |show_box|.
//! @^recursion@>
//!
//! @p procedure show_box(@!p:pointer);
//! begin @<Assign the values |depth_threshold:=show_box_depth| and
//!   |breadth_max:=show_box_breadth|@>;
//! if breadth_max<=0 then breadth_max:=5;
//! if pool_ptr+depth_threshold>=pool_size then
//!   depth_threshold:=pool_size-pool_ptr-1;
//!   {now there's enough room for prefix string}
//! show_node_list(p); {the show starts at |p|}
//! print_ln;
//! end;
//!
