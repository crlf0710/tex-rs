//! @ @<Display the insertion split cost@>=
//! begin begin_diagnostic; print_nl("% split"); print_int(n);
//! @.split@>
//! print(" to "); print_scaled(w);
//! print_char(","); print_scaled(best_height_plus_depth);@/
//! print(" p=");
//! if q=null then print_int(eject_penalty)
//! else if type(q)=penalty_node then print_int(penalty(q))
//! else print_char("0");
//! end_diagnostic(false);
//! end
//!
