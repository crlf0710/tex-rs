
//! @ When debugging, we may want to print a |memory_word| without knowing
//! what type it is; so we print it in all modes.
//! @^dirty \PASCAL@>@^debugging@>
//!
//! @p @!debug procedure print_word(@!w:memory_word);
//!   {prints |w| in all ways}
//! begin print_int(w.int); print_char(" ");@/
//! print_scaled(w.sc); print_char(" ");@/
//! print_scaled(round(unity*float(w.gr))); print_ln;@/
//! @^real multiplication@>
//! print_int(w.hh.lh); print_char("="); print_int(w.hh.b0); print_char(":");
//! print_int(w.hh.b1); print_char(";"); print_int(w.hh.rh); print_char(" ");@/
//! print_int(w.qqqq.b0); print_char(":"); print_int(w.qqqq.b1); print_char(":");
//! print_int(w.qqqq.b2); print_char(":"); print_int(w.qqqq.b3);
//! end;
//! gubed
//!