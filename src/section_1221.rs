//! ` `

// @<Assignments@>=
macro_rules! Assignments_1221 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        if $cur_cmd == r#let {
            // let:  begin n:=cur_chr;
            let n = $globals.cur_chr.get();

            /// for temporary short-term use
            let p;
            // get_r_token; p:=cur_cs;
            get_r_token($globals)?;
            p = $globals.cur_cs;
            // if n=normal then
            if n == let_kind::normal as chr_code_repr {
                // begin repeat get_token;
                loop {
                    get_token($globals)?;
                    // until cur_cmd<>spacer;
                    if $globals.cur_cmd != spacer {
                        break;
                    }
                }
                // if cur_tok=other_token+"=" then
                if $globals.cur_tok == other_token + b'=' as cur_tok_repr {
                    // begin get_token;
                    get_token($globals)?;
                    // if cur_cmd=spacer then get_token;
                    if $globals.cur_cmd == spacer {
                        get_token($globals)?;
                    }
                    // end;
                }
                // end
            }
            // else  begin get_token; q:=cur_tok; get_token; back_input;
            else {
                /// for temporary short-term use
                let q;
                
                get_token($globals)?;
                q = $globals.cur_tok;
                get_token($globals)?;
                back_input($globals);
                // cur_tok:=q; back_input; {look ahead, then back up}
                $globals.cur_tok = q;
                back_input($globals);
                /// look ahead, then back up
                const _ : () = ();
                // end; {note that |back_input| doesn't affect |cur_cmd|, |cur_chr|}
                /// note that `back_input` doesn't affect `cur_cmd`, `cur_chr`
                const _ : () = ();
            }
            // if cur_cmd>=call then add_token_ref(cur_chr);
            if $globals.cur_cmd >= call {
                add_token_ref!($globals, $globals.cur_chr.get() as pointer);
            }
            // define(p,cur_cmd,cur_chr);
            define!($globals, $a, p as _, $globals.cur_cmd, $globals.cur_chr.get() as _);
            // end;
            use crate::section_0115::pointer;
            use crate::section_0135::let_kind;
            use crate::section_0207::spacer;
            use crate::section_0289::other_token;
            use crate::section_0297::cur_tok_repr;
            use crate::section_0297::chr_code_repr;
            use crate::section_0325::back_input;
            use crate::section_0365::get_token;
            use crate::section_1215::get_r_token;
            true
        } else {
            false
        }
    }}
}
