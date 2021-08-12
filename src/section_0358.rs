//! @ The present point in the program is reached only when the |expand|
//! routine has inserted a special marker into the input. In this special
//! case, |info(loc)| is known to be a control sequence token, and |link(loc)=null|.
//
// @d no_expand_flag=257 {this characterizes a special variant of |relax|}
/// this characterizes a special variant of `relax`
pub(crate) const no_expand_flag: chr_code_repr = crate::pascal::CHAR_MAX_REPR as chr_code_repr + 1;

// @<Get the next token, suppressing expansion@>=
macro_rules! Get_the_next_token__suppressing_expansion {
    ($globals:expr) => {{
        // begin cur_cs:=info(loc)-cs_token_flag; loc:=null;@/
        $globals.cur_cs = info_tok!($globals, loc!($globals)).get_cs().unwrap();
        loc!($globals) = null;
        // cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
        $globals.cur_cmd = eq_type!($globals, $globals.cur_cs);
        $globals.cur_chr = chr_code_type::new(equiv!($globals, $globals.cur_cs) as _);
        // if cur_cmd>max_command then
        if $globals.cur_cmd > max_command {
            // begin cur_cmd:=relax; cur_chr:=no_expand_flag;
            $globals.cur_cmd = relax;
            $globals.cur_chr = chr_code_type::new(no_expand_flag);
            // end;
        }
        // end
        use crate::section_0207::relax;
        use crate::section_0209::max_command;
        use crate::section_0358::no_expand_flag;
    }}
}

use crate::section_0297::chr_code_repr;