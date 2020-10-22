//! @ The present point in the program is reached only when the |expand|
//! routine has inserted a special marker into the input. In this special
//! case, |info(loc)| is known to be a control sequence token, and |link(loc)=null|.
//!
//! @d no_expand_flag=257 {this characterizes a special variant of |relax|}
//!
//! @<Get the next token, suppressing expansion@>=
//! begin cur_cs:=info(loc)-cs_token_flag; loc:=null;@/
//! cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
//! if cur_cmd>max_command then
//!   begin cur_cmd:=relax; cur_chr:=no_expand_flag;
//!   end;
//! end
//!
//! @ @<Insert macro parameter...@>=
//! begin begin_token_list(param_stack[param_start+cur_chr-1],parameter);
//! goto restart;
//! end
//!
