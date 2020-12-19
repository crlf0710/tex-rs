//! @ Here is a trivial procedure to print two digits; it is usually called with
//! a parameter in the range |0<=n<=99|.
//!
//! @p procedure print_two(@!n:integer); {prints two least significant digits}
//! begin n:=abs(n) mod 100; print_char("0"+(n div 10));
//! print_char("0"+(n mod 10));
//! end;
//!
