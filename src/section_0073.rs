//! @ The global variable |interaction| has four settings, representing increasing
//! amounts of user interaction:
//!
//! @d batch_mode=0 {omits all stops and omits terminal output}
//! @d nonstop_mode=1 {omits all stops}
//! @d scroll_mode=2 {omits error stops}
//! @d error_stop_mode=3 {stops at every opportunity to interact}
//! @d print_err(#)==begin if interaction=error_stop_mode then wake_up_terminal;
//!   print_nl("! "); print(#);
//!   end
//!
//! @<Glob...@>=
//! @!interaction:batch_mode..error_stop_mode; {current level of interaction}
//!
