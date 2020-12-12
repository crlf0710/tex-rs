//! @ @<Recover from an unbalanced write command@>=
//! begin print_err("Unbalanced write command");
//! @.Unbalanced write...@>
//! help2("On this page there's a \write with fewer real {'s than }'s.")@/
//! ("I can't handle that very well; good luck."); error;
//! repeat get_token;
//! until cur_tok=end_write_token;
//! end
//!
