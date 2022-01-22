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
