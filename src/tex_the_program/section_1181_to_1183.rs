//! @ @<Declare act...@>=
//! procedure math_fraction;
//! var c:small_number; {the type of generalized fraction we are scanning}
//! begin c:=cur_chr;
//! if incompleat_noad<>null then
//!   @<Ignore the fraction operation and complain about this ambiguous case@>
//! else  begin incompleat_noad:=get_node(fraction_noad_size);
//!   type(incompleat_noad):=fraction_noad;
//!   subtype(incompleat_noad):=normal;
//!   math_type(numerator(incompleat_noad)):=sub_mlist;
//!   info(numerator(incompleat_noad)):=link(head);
//!   mem[denominator(incompleat_noad)].hh:=empty_field;
//!   mem[left_delimiter(incompleat_noad)].qqqq:=null_delimiter;
//!   mem[right_delimiter(incompleat_noad)].qqqq:=null_delimiter;@/
//!   link(head):=null; tail:=head;
//!   @<Use code |c| to distinguish between generalized fractions@>;
//!   end;
//! end;
//!
//! @ @<Use code |c|...@>=
//! if c>=delimited_code then
//!   begin scan_delimiter(left_delimiter(incompleat_noad),false);
//!   scan_delimiter(right_delimiter(incompleat_noad),false);
//!   end;
//! case c mod delimited_code of
//! above_code: begin scan_normal_dimen;
//!   thickness(incompleat_noad):=cur_val;
//!   end;
//! over_code: thickness(incompleat_noad):=default_code;
//! atop_code: thickness(incompleat_noad):=0;
//! end {there are no other cases}
//!
//! @ @<Ignore the fraction...@>=
//! begin if c>=delimited_code then
//!   begin scan_delimiter(garbage,false); scan_delimiter(garbage,false);
//!   end;
//! if c mod delimited_code=above_code then scan_normal_dimen;
//! print_err("Ambiguous; you need another { and }");
//! @.Ambiguous...@>
//! help3("I'm ignoring this fraction specification, since I don't")@/
//!   ("know whether a construction like `x \over y \over z'")@/
//!   ("means `{x \over y} \over z' or `x \over {y \over z}'.");
//! error;
//! end
//!
