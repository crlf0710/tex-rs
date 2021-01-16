//! @ Then there is a subroutine that prints glue stretch and shrink, possibly
//! followed by the name of finite units:
//!
//! @p procedure print_glue(@!d:scaled;@!order:integer;@!s:str_number);
//!   {prints a glue component}
//! begin print_scaled(d);
//! if (order<normal)or(order>filll) then print("foul")
//! else if order>normal then
//!   begin print("fil");
//!   while order>fil do
//!     begin print_char("l"); decr(order);
//!     end;
//!   end
//! else if s<>0 then print(s);
//! end;
//!
