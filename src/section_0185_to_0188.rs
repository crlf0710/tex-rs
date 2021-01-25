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
