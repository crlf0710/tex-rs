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
