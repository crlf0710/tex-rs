//! @ Here is the most dreaded error message.
//!
//! @<Error hand...@>=
//! procedure overflow(@!s:str_number;@!n:integer); {stop due to finiteness}
//! begin normalize_selector;
//! print_err("TeX capacity exceeded, sorry [");
//! @.TeX capacity exceeded ...@>
//! print(s); print_char("="); print_int(n); print_char("]");
//! help2("If you really absolutely need more capacity,")@/
//!   ("you can ask a wizard to enlarge me.");
//! succumb;
//! end;
//!
