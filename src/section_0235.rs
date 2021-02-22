//! @ @<Show the halfword code in |eqtb[n]|@>=
//! if n<math_code_base then
//!   begin if n<lc_code_base then
//!     begin print_esc("catcode"); print_int(n-cat_code_base);
//!     end
//!   else if n<uc_code_base then
//!     begin print_esc("lccode"); print_int(n-lc_code_base);
//!     end
//!   else if n<sf_code_base then
//!     begin print_esc("uccode"); print_int(n-uc_code_base);
//!     end
//!   else  begin print_esc("sfcode"); print_int(n-sf_code_base);
//!     end;
//!   print_char("="); print_int(equiv(n));
//!   end
//! else  begin print_esc("mathcode"); print_int(n-math_code_base);
//!   print_char("="); print_int(ho(equiv(n)));
//!   end
//!
