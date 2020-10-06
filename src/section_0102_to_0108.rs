//! @ The following function is used to create a scaled integer from a given decimal
//! fraction $(.d_0d_1\ldots d_{k-1})$, where |0<=k<=17|. The digit $d_i$ is
//! given in |dig[i]|, and the calculation produces a correctly rounded result.
//!
//! @p function round_decimals(@!k:small_number) : scaled;
//!   {converts a decimal fraction}
//! var a:integer; {the accumulator}
//! begin a:=0;
//! while k>0 do
//!   begin decr(k); a:=(a+dig[k]*two) div 10;
//!   end;
//! round_decimals:=(a+1) div 2;
//! end;
//!
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
//! @ Physical sizes that a \TeX\ user specifies for portions of documents are
//! represented internally as scaled points. Thus, if we define an `sp' (scaled
//! @^sp@>
//! point) as a unit equal to $2^{-16}$ printer's points, every dimension
//! inside of \TeX\ is an integer number of sp. There are exactly
//! 4,736,286.72 sp per inch.  Users are not allowed to specify dimensions
//! larger than $2^{30}-1$ sp, which is a distance of about 18.892 feet (5.7583
//! meters); two such quantities can be added without overflow on a 32-bit
//! computer.
//!
//! The present implementation of \TeX\ does not check for overflow when
//! @^overflow in arithmetic@>
//! dimensions are added or subtracted. This could be done by inserting a
//! few dozen tests of the form `\ignorespaces|if x>=@'10000000000 then
//! @t\\{report\_overflow}@>|', but the chance of overflow is so remote that
//! such tests do not seem worthwhile.
//!
//! \TeX\ needs to do only a few arithmetic operations on scaled quantities,
//! other than addition and subtraction, and the following subroutines do most of
//! the work. A single computation might use several subroutine calls, and it is
//! desirable to avoid producing multiple error messages in case of arithmetic
//! overflow; so the routines set the global variable |arith_error| to |true|
//! instead of reporting errors directly to the user. Another global variable,
//! |remainder|, holds the remainder after a division.
//!
//! @<Glob...@>=
//! @!arith_error:boolean; {has arithmetic overflow occurred recently?}
//! @!remainder:scaled; {amount subtracted to get an exact division}
//!
//! @ The first arithmetical subroutine we need computes $nx+y$, where |x|
//! and~|y| are |scaled| and |n| is an integer. We will also use it to
//! multiply integers.
//!
//! @d nx_plus_y(#)==mult_and_add(#,@'7777777777)
//! @d mult_integers(#)==mult_and_add(#,0,@'17777777777)
//!
//! @p function mult_and_add(@!n:integer;@!x,@!y,@!max_answer:scaled):scaled;
//! begin if n<0 then
//!   begin negate(x); negate(n);
//!   end;
//! if n=0 then mult_and_add:=y
//! else if ((x<=(max_answer-y) div n)and(-x<=(max_answer+y) div n)) then
//!   mult_and_add:=n*x+y
//! else  begin arith_error:=true; mult_and_add:=0;
//!   end;
//! end;
//!
//! @ We also need to divide scaled dimensions by integers.
//!
//! @p function x_over_n(@!x:scaled;@!n:integer):scaled;
//! var negative:boolean; {should |remainder| be negated?}
//! begin negative:=false;
//! if n=0 then
//!   begin arith_error:=true; x_over_n:=0; remainder:=x;
//!   end
//! else  begin if n<0 then
//!     begin negate(x); negate(n); negative:=true;
//!     end;
//!   if x>=0 then
//!     begin x_over_n:=x div n; remainder:=x mod n;
//!     end
//!   else  begin x_over_n:=-((-x) div n); remainder:=-((-x) mod n);
//!     end;
//!   end;
//! if negative then negate(remainder);
//! end;
//!
//! @ Then comes the multiplication of a scaled number by a fraction |n/d|,
//! where |n| and |d| are nonnegative integers |<=@t$2^{16}$@>| and |d| is
//! positive. It would be too dangerous to multiply by~|n| and then divide
//! by~|d|, in separate operations, since overflow might well occur; and it
//! would be too inaccurate to divide by |d| and then multiply by |n|. Hence
//! this subroutine simulates 1.5-precision arithmetic.
//!
//! @p function xn_over_d(@!x:scaled; @!n,@!d:integer):scaled;
//! var positive:boolean; {was |x>=0|?}
//! @!t,@!u,@!v:nonnegative_integer; {intermediate quantities}
//! begin if x>=0 then positive:=true
//! else  begin negate(x); positive:=false;
//!   end;
//! t:=(x mod @'100000)*n;
//! u:=(x div @'100000)*n+(t div @'100000);
//! v:=(u mod d)*@'100000 + (t mod @'100000);
//! if u div d>=@'100000 then arith_error:=true
//! else u:=@'100000*(u div d) + (v div d);
//! if positive then
//!   begin xn_over_d:=u; remainder:=v mod d;
//!   end
//! else  begin xn_over_d:=-u; remainder:=-(v mod d);
//!   end;
//! end;
//!
//! @ The next subroutine is used to compute the ``badness'' of glue, when a
//! total~|t| is supposed to be made from amounts that sum to~|s|.  According
//! to {\sl The \TeX book}, the badness of this situation is $100(t/s)^3$;
//! however, badness is simply a heuristic, so we need not squeeze out the
//! last drop of accuracy when computing it. All we really want is an
//! approximation that has similar properties.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! The actual method used to compute the badness is easier to read from the
//! program than to describe in words. It produces an integer value that is a
//! reasonably close approximation to $100(t/s)^3$, and all implementations
//! of \TeX\ should use precisely this method. Any badness of $2^{13}$ or more is
//! treated as infinitely bad, and represented by 10000.
//!
//! It is not difficult to prove that $$\hbox{|badness(t+1,s)>=badness(t,s)
//! >=badness(t,s+1)|}.$$ The badness function defined here is capable of
//! computing at most 1095 distinct values, but that is plenty.
//!
//! @d inf_bad = 10000 {infinitely bad value}
//!
//! @p function badness(@!t,@!s:scaled):halfword; {compute badness, given |t>=0|}
//! var r:integer; {approximation to $\alpha t/s$, where $\alpha^3\approx
//!   100\cdot2^{18}$}
//! begin if t=0 then badness:=0
//! else if s<=0 then badness:=inf_bad
//! else  begin if t<=7230584 then  r:=(t*297) div s {$297^3=99.94\times2^{18}$}
//!   else if s>=1663497 then r:=t div (s div 297)
//!   else r:=t;
//!   if r>1290 then badness:=inf_bad {$1290^3<2^{31}<1291^3$}
//!   else badness:=(r*r*r+@'400000) div @'1000000;
//!   end; {that was $r^3/2^{18}$, rounded to the nearest integer}
//! end;
//!
