//! @ @<Set init...@>=
//! top_mark:=null; first_mark:=null; bot_mark:=null;
//! split_first_mark:=null; split_bot_mark:=null;
//!
//! @ @<Put each...@>=
//! primitive("topmark",top_bot_mark,top_mark_code);
//! @!@:top_mark_}{\.{\\topmark} primitive@>
//! primitive("firstmark",top_bot_mark,first_mark_code);
//! @!@:first_mark_}{\.{\\firstmark} primitive@>
//! primitive("botmark",top_bot_mark,bot_mark_code);
//! @!@:bot_mark_}{\.{\\botmark} primitive@>
//! primitive("splitfirstmark",top_bot_mark,split_first_mark_code);
//! @!@:split_first_mark_}{\.{\\splitfirstmark} primitive@>
//! primitive("splitbotmark",top_bot_mark,split_bot_mark_code);
//! @!@:split_bot_mark_}{\.{\\splitbotmark} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! top_bot_mark: case chr_code of
//!   first_mark_code: print_esc("firstmark");
//!   bot_mark_code: print_esc("botmark");
//!   split_first_mark_code: print_esc("splitfirstmark");
//!   split_bot_mark_code: print_esc("splitbotmark");
//!   othercases print_esc("topmark")
//!   endcases;
//!
//! @ The following code is activated when |cur_cmd=top_bot_mark| and
//! when |cur_chr| is a code like |top_mark_code|.
//!
//! @<Insert the \(a)appropriate mark text into the scanner@>=
//! begin if cur_mark[cur_chr]<>null then
//!   begin_token_list(cur_mark[cur_chr],mark_text);
//! end
//!
