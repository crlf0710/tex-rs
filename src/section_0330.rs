//! @ In order to keep the stack from overflowing during a long sequence of
//! inserted `\.{\\show}' commands, the following routine removes completed
//! error-inserted lines from memory.
//!
//! @p procedure clear_for_error_prompt;
//! begin while (state<>token_list)and terminal_input and@|
//!   (input_ptr>0)and(loc>limit) do end_file_reading;
//! print_ln; clear_terminal;
//! end;
//!
