//! @ Conversely, here is a procedure analogous to |print_int|. If the output
//! of this procedure is subsequently read by \TeX\ and converted by the
//! |round_decimals| routine above, it turns out that the original value will
//! be reproduced exactly; the ``simplest'' such decimal number is output,
//! but there is always at least one digit following the decimal point.
//!
//! The invariant relation in the \&{repeat} loop is that a sequence of
//! decimal digits yet to be printed will yield the original number if and only if
//! they form a fraction~$f$ in the range $s-\delta\L10\cdot2^{16}f<s$.
//! We can stop if and only if $f=0$ satisfies this condition; the loop will
//! terminate before $s$ can possibly become zero.
//!
//! @p procedure print_scaled(@!s:scaled); {prints scaled real, rounded to five
//!   digits}
//! var delta:scaled; {amount of allowable inaccuracy}
//! begin if s<0 then
//!   begin print_char("-"); negate(s); {print the sign, if negative}
//!   end;
//! print_int(s div unity); {print the integer part}
//! print_char(".");
//! s:=10*(s mod unity)+5; delta:=10;
//! repeat if delta>unity then s:=s+@'100000-50000; {round the last digit}
//! print_char("0"+(s div unity)); s:=10*(s mod unity); delta:=delta*10;
//! until s<=delta;
//! end;
//!
