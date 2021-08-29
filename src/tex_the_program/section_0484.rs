//! @ Here we input on-line into the |buffer| array, prompting the user explicitly
//! if |n>=0|.  The value of |n| is set negative so that additional prompts
//! will not be given in the case of multi-line input.
//!
//! @<Input for \.{\\read} from the terminal@>=
//! if interaction>nonstop_mode then
//!   if n<0 then prompt_input("")
//!   else  begin wake_up_terminal;
//!     print_ln; sprint_cs(r); prompt_input("="); n:=-1;
//!     end
//! else fatal_error("*** (cannot \read from terminal in nonstop modes)")
//! @.cannot \\read@>
//!
