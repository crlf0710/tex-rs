//! @ The token-list parameters, \.{\\output} and \.{\\everypar}, etc., receive
//! their values in the following way. (For safety's sake, we place an
//! enclosing pair of braces around an \.{\\output} list.)
//
// @<Assignments@>=
macro_rules! Assignments_1226 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        // toks_register,assign_toks: begin q:=cur_cs;
        if $cur_cmd == toks_register || $cur_cmd == assign_toks {
            /// for temporary short-term use
            let (p, mut q): (pointer, pointer);

            q = $globals.cur_cs;
            // if cur_cmd=toks_register then
            if $cur_cmd == toks_register {
                // begin scan_eight_bit_int; p:=toks_base+cur_val;
                todo!("toks_register");
                // end
            }
            // else p:=cur_chr; {|p=every_par_loc| or |output_routine_loc| or \dots}
            else {
                /// `p = every_par_loc` or `output_routine_loc` or `...`
                const _ : () = ();
                p = $globals.cur_chr.get() as pointer;
            }
            // scan_optional_equals;
            scan_optional_equals($globals)?;
            // @<Get the next non-blank non-relax non-call token@>;
            Get_the_next_non_blank_non_relax_non_call_token!($globals);
            // if cur_cmd<>left_brace then @<If the right-hand side is a token parameter
            //     or token register, finish the assignment and |goto done|@>;
            if $cur_cmd != left_brace {
                todo!("if the right-hand side");
            }
            // back_input; cur_cs:=q; q:=scan_toks(false,false);
            back_input($globals);
            $globals.cur_cs = q;
            q = scan_toks($globals, false, false)?;
            // if link(def_ref)=null then {empty list: revert to the default}
            if link!($globals, $globals.def_ref) == null {
                /// empty list: revert to the default
                const _ : () = ();
                // begin define(p,undefined_cs,null); free_avail(def_ref);
                define!($globals, $a, p, undefined_cs, null);
                free_avail!($globals, $globals.def_ref);
                // end
            }
            // else  begin if p=output_routine_loc then {enclose in curlies}
            else {
                if p == output_routine_loc {
                    /// enclose in curlies
                    const _ : () = ();
                    // begin link(q):=get_avail; q:=link(q);
                    link!($globals, q) = get_avail($globals);
                    q = link!($globals, q);
                    // info(q):=right_brace_token+"}";
                    info_tok_assign!($globals, q, cur_tok_type::new(right_brace_token + b'}' as cur_tok_repr));
                    // q:=get_avail; info(q):=left_brace_token+"{";
                    q = get_avail($globals);
                    info_tok_assign!($globals, q, cur_tok_type::new(left_brace_token + b'{' as cur_tok_repr));
                    // link(q):=link(def_ref); link(def_ref):=q;
                    link!($globals, q) = link!($globals, $globals.def_ref);
                    link!($globals, $globals.def_ref) = q;
                    // end;
                }
                // define(p,call,def_ref);
                define!($globals, $a, p, call, $globals.def_ref);
                // end;
            }

            use crate::section_0115::pointer;
            use crate::section_0115::null;
            use crate::section_0120::get_avail;
            use crate::section_0207::left_brace;
            use crate::section_0230::output_routine_loc;
            use crate::section_0289::left_brace_token;
            use crate::section_0289::right_brace_token;
            use crate::section_0297::cur_tok_type;
            use crate::section_0297::cur_tok_repr;
            use crate::section_0325::back_input;
            use crate::section_0473::scan_toks;
            true
        }
        else {
            false
        }
        // end;
    }}
}
