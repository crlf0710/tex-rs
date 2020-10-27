//! @ Conversely, here is a routine that takes three strings and prints a file
//! name that might have produced them. (The routine is system dependent, because
//! some operating systems put the file area last instead of first.)
//! @^system dependencies@>
//!
//! @<Basic printing...@>=
//! procedure print_file_name(@!n,@!a,@!e:integer);
//! begin slow_print(a); slow_print(n); slow_print(e);
//! end;
//!
