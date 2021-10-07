//! ` `

// @<Attach list |p| to the current...@>=
pub(crate) macro Attach_list_p_to_the_current_list_and_record_its_length__then_finish_up_and_return($globals:expr, $p:expr, $q:expr, $n:expr) {{
    // begin if (n>0)and(abs(mode)=mmode) then
    if $n > 0 && mode!($globals).get().abs() == mmode {
        // begin print_err("Illegal math "); print_esc("discretionary");
        // @.Illegal math \\disc...@>
        // help2("Sorry: The third part of a discretionary break must be")@/
        // ("empty, in math formulas. I had to delete your third part.");
        // flush_node_list(p); n:=0; error;
        // end
        todo!("illegal math");
    }
    // else link(tail):=p;
    else {
        link!($globals, tail!($globals)) = $p;
    }
    // if n<=max_quarterword then replace_count(tail):=n
    if $n <= max_quarterword as integer {
        replace_count!($globals, tail!($globals)) = $n as _;
    }
    // else  begin print_err("Discretionary list is too long");
    else {
        // @.Discretionary list is too long@>
        //   help2("Wow---I never thought anybody would tweak me here.")@/
        //   ("You can't seriously need such a huge discretionary list?");
        //   error;
        //   end;
        todo!("too long");
    }
    // if n>0 then tail:=q;
    if $n > 0 {
        tail!($globals) = $q;
    }
    // decr(save_ptr); return;
    decr!($globals.save_ptr);
    crate::return_nojump!();
    // end
    use crate::pascal::integer;
    use crate::section_0016::decr;
    use crate::section_0110::max_quarterword;
    use crate::section_0118::link;
    use crate::section_0145::replace_count;
    use crate::section_0211::mmode;
    use crate::section_0213::mode;
    use crate::section_0213::tail;
}}
