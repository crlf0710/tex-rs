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
