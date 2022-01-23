//! @ The numerator and denominator must be separated by a certain minimum
//! clearance, called |clr| in the following program. The difference between
//! |clr| and the actual clearance is twice |delta|.
//
// @<Adjust \(s)|shift_up| and |shift_down| for the case of no fraction line@>=
// begin if cur_style<text_style then clr:=7*default_rule_thickness
// else clr:=3*default_rule_thickness;
// delta:=half(clr-((shift_up-depth(x))-(height(z)-shift_down)));
// if delta>0 then
//   begin shift_up:=shift_up+delta;
//   shift_down:=shift_down+delta;
//   end;
// end
//
