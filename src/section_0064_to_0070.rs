//! @ An array of digits in the range |0..15| is printed by |print_the_digs|.
//!
//! @<Basic print...@>=
//! procedure print_the_digs(@!k:eight_bits);
//!   {prints |dig[k-1]|$\,\ldots\,$|dig[0]|}
//! begin while k>0 do
//!   begin decr(k);
//!   if dig[k]<10 then print_char("0"+dig[k])
//!   else print_char("A"-10+dig[k]);
//!   end;
//! end;
//!
//! @ The following procedure, which prints out the decimal representation of a
//! given integer |n|, has been written carefully so that it works properly
//! if |n=0| or if |(-n)| would cause overflow. It does not apply |mod| or |div|
//! to negative arguments, since such operations are not implemented consistently
//! by all \PASCAL\ compilers.
//!
//! @<Basic print...@>=
//! procedure print_int(@!n:integer); {prints an integer in decimal form}
//! var k:0..23; {index to current digit; we assume that $|n|<10^{23}$}
//! @!m:integer; {used to negate |n| in possibly dangerous cases}
//! begin k:=0;
//! if n<0 then
//!   begin print_char("-");
//!   if n>-100000000 then negate(n)
//!   else  begin m:=-1-n; n:=m div 10; m:=(m mod 10)+1; k:=1;
//!     if m<10 then dig[0]:=m
//!     else  begin dig[0]:=0; incr(n);
//!       end;
//!     end;
//!   end;
//! repeat dig[k]:=n mod 10; n:=n div 10; incr(k);
//! until n=0;
//! print_the_digs(k);
//! end;
//!
//! @ Here is a trivial procedure to print two digits; it is usually called with
//! a parameter in the range |0<=n<=99|.
//!
//! @p procedure print_two(@!n:integer); {prints two least significant digits}
//! begin n:=abs(n) mod 100; print_char("0"+(n div 10));
//! print_char("0"+(n mod 10));
//! end;
//!
//! @ Hexadecimal printing of nonnegative integers is accomplished by |print_hex|.
//!
//! @p procedure print_hex(@!n:integer);
//!   {prints a positive integer in hexadecimal form}
//! var k:0..22; {index to current digit; we assume that $0\L n<16^{22}$}
//! begin k:=0; print_char("""");
//! repeat dig[k]:=n mod 16; n:=n div 16; incr(k);
//! until n=0;
//! print_the_digs(k);
//! end;
//!
//! @ Old versions of \TeX\ needed a procedure called |print_ASCII| whose function
//! is now subsumed by |print|. We retain the old name here as a possible aid to
//! future software arch\ae ologists.
//!
//! @d print_ASCII == print
//!
//! @ Roman numerals are produced by the |print_roman_int| routine.  Readers
//! who like puzzles might enjoy trying to figure out how this tricky code
//! works; therefore no explanation will be given. Notice that 1990 yields
//! \.{mcmxc}, not \.{mxm}.
//!
//! @p procedure print_roman_int(@!n:integer);
//! label exit;
//! var j,@!k: pool_pointer; {mysterious indices into |str_pool|}
//! @!u,@!v: nonnegative_integer; {mysterious numbers}
//! begin j:=str_start["m2d5c2l5x2v5i"]; v:=1000;
//! loop@+  begin while n>=v do
//!     begin print_char(so(str_pool[j])); n:=n-v;
//!     end;
//!   if n<=0 then return; {nonpositive input produces no output}
//!   k:=j+2; u:=v div (so(str_pool[k-1])-"0");
//!   if str_pool[k-1]=si("2") then
//!     begin k:=k+2; u:=u div (so(str_pool[k-1])-"0");
//!     end;
//!   if n+u>=v then
//!     begin print_char(so(str_pool[k])); n:=n+u;
//!     end
//!   else  begin j:=j+2; v:=v div (so(str_pool[j-1])-"0");
//!     end;
//!   end;
//! exit:end;
//!
//! @ The |print| subroutine will not print a string that is still being
//! created. The following procedure will.
//!
//! @p procedure print_current_string; {prints a yet-unmade string}
//! var j:pool_pointer; {points to current character code}
//! begin j:=str_start[str_ptr];
//! while j<pool_ptr do
//!   begin print_char(so(str_pool[j])); incr(j);
//!   end;
//! end;
//!
