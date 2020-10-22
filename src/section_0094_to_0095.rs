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
//! @ The program might sometime run completely amok, at which point there is
//! no choice but to stop. If no previous error has been detected, that's bad
//! news; a message is printed that is really intended for the \TeX\
//! maintenance person instead of the user (unless the user has been
//! particularly diabolical).  The index entries for `this can't happen' may
//! help to pinpoint the problem.
//! @^dry rot@>
//!
//! @<Error hand...@>=
//! procedure confusion(@!s:str_number);
//!   {consistency check violated; |s| tells where}
//! begin normalize_selector;
//! if history<error_message_issued then
//!   begin print_err("This can't happen ("); print(s); print_char(")");
//! @.This can't happen@>
//!   help1("I'm broken. Please show this to someone who can fix can fix");
//!   end
//! else  begin print_err("I can't go on meeting you like this");
//! @.I can't go on...@>
//!   help2("One of your faux pas seems to have wounded me deeply...")@/
//!     ("in fact, I'm barely conscious. Please fix it and try again.");
//!   end;
//! succumb;
//! end;
//!
