//! @ \TeX\ gets to the following part of the program when the first `\.\$' ending
//! a display has been scanned.
//!
//! @<Check that another \.\$ follows@>=
//! begin get_x_token;
//! if cur_cmd<>math_shift then
//!   begin print_err("Display math should end with $$");
//! @.Display math...with \$\$@>
//!   help2("The `$' that I just saw supposedly matches a previous `$$'.")@/
//!     ("So I shall assume that you typed `$$' both times.");
//!   back_error;
//!   end;
//! end
//!
//! @ We have saved the worst for last: The fussiest part of math mode processing
//! occurs when a displayed formula is being centered and placed with an optional
//! equation number.
//!
//! @<Local variables for finishing...@>=
//! @!b:pointer; {box containing the equation}
//! @!w:scaled; {width of the equation}
//! @!z:scaled; {width of the line}
//! @!e:scaled; {width of equation number}
//! @!q:scaled; {width of equation number plus space to separate from equation}
//! @!d:scaled; {displacement of equation in the line}
//! @!s:scaled; {move the line right this much}
//! @!g1,@!g2:small_number; {glue parameter codes for before and after}
//! @!r:pointer; {kern node used to position the display}
//! @!t:pointer; {tail of adjustment list}
