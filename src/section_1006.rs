//! @ @<Display the page break cost@>=
//! begin begin_diagnostic; print_nl("%");
//! print(" t="); print_totals;@/
//! print(" g="); print_scaled(page_goal);@/
//! print(" b=");
//! if b=awful_bad then print_char("*")@+else print_int(b);
//! @.*\relax@>
//! print(" p="); print_int(pi);
//! print(" c=");
//! if c=awful_bad then print_char("*")@+else print_int(c);
//! if c<=least_page_cost then print_char("#");
//! end_diagnostic(false);
//! end
//!
