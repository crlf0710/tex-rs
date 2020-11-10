//! @ A control sequence that has been \.{\\def}'ed by the user is expanded by
//! \TeX's |macro_call| procedure.
//!
//! Before we get into the details of |macro_call|, however, let's consider the
//! treatment of primitives like \.{\\topmark}, since they are essentially
//! macros without parameters. The token lists for such marks are kept in a
//! global array of five pointers; we refer to the individual entries of this
//! array by symbolic names |top_mark|, etc. The value of |top_mark| is either
//! |null| or a pointer to the reference count of a token list.
//!
//! @d top_mark_code=0 {the mark in effect at the previous page break}
//! @d first_mark_code=1 {the first mark between |top_mark| and |bot_mark|}
//! @d bot_mark_code=2 {the mark in effect at the current page break}
//! @d split_first_mark_code=3 {the first mark found by \.{\\vsplit}}
//! @d split_bot_mark_code=4 {the last mark found by \.{\\vsplit}}
//! @d top_mark==cur_mark[top_mark_code]
//! @d first_mark==cur_mark[first_mark_code]
//! @d bot_mark==cur_mark[bot_mark_code]
//! @d split_first_mark==cur_mark[split_first_mark_code]
//! @d split_bot_mark==cur_mark[split_bot_mark_code]
//!
//! @<Glob...@>=
//! @!cur_mark:array[top_mark_code..split_bot_mark_code] of pointer;
//!   {token lists for marks}
//!
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
//! @ Now let's consider |macro_call| itself, which is invoked when \TeX\ is
//! scanning a control sequence whose |cur_cmd| is either |call|, |long_call|,
//! |outer_call|, or |long_outer_call|.  The control sequence definition
//! appears in the token list whose reference count is in location |cur_chr|
//! of |mem|.
//!
//! The global variable |long_state| will be set to |call| or to |long_call|,
//! depending on whether or not the control sequence disallows \.{\\par}
//! in its parameters. The |get_next| routine will set |long_state| to
//! |outer_call| and emit \.{\\par}, if a file ends or if an \.{\\outer}
//! control sequence occurs in the midst of an argument.
//!
//! @<Glob...@>=
//! @!long_state:call..long_outer_call; {governs the acceptance of \.{\\par}}
//!
//! @ The parameters, if any, must be scanned before the macro is expanded.
//! Parameters are token lists without reference counts. They are placed on
//! an auxiliary stack called |pstack| while they are being scanned, since
//! the |param_stack| may be losing entries during the matching process.
//! (Note that |param_stack| can't be gaining entries, since |macro_call| is
//! the only routine that puts anything onto |param_stack|, and it
//! is not recursive.)
//!
//! @<Glob...@>=
//! @!pstack:array[0..8] of pointer; {arguments supplied to a macro}
//!
