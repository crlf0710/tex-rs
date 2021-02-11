//! @ @<Print a symbolic description of the new break node@>=
//! begin print_nl("@@@@"); print_int(serial(passive));
//! @.\AT!\AT!@>
//! print(": line "); print_int(line_number(q)-1);
//! print_char("."); print_int(fit_class);
//! if break_type=hyphenated then print_char("-");
//! print(" t="); print_int(total_demerits(q));
//! print(" -> @@@@");
//! if prev_break(passive)=null then print_char("0")
//! else print_int(serial(prev_break(passive)));
//! end
//!
