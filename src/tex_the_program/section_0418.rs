//! ` `
// @<Fetch the |space_factor| or the |prev_depth|@>=
pub(crate) macro Fetch_the_space_factor_or_the_prev_depth($globals:expr, $m:expr) {{
    // if abs(mode)<>m then
    if mode!($globals).get().abs() != $m.get() as _ {
        // begin print_err("Improper "); print_cmd_chr(set_aux,m);
        // @.Improper \\spacefactor@>
        // @.Improper \\prevdepth@>
        // help4("You can refer to \spacefactor only in horizontal mode;")@/
        //   ("you can refer to \prevdepth only in vertical mode; and")@/
        //   ("neither of these is meaningful inside \write. So")@/
        //   ("I'm forgetting what you said and using zero instead.");
        // error;
        // if level<>tok_val then scanned_result(0)(dimen_val)
        // else scanned_result(0)(int_val);
        // end
        todo!("error");
    }
    // else if m=vmode then scanned_result(prev_depth)(dimen_val)
    else if $m.get() as integer == vmode as _ {
        scanned_result!(
            $globals,
            prev_depth!($globals).inner(),
            cur_val_level_kind::dimen_val
        );
    }
    // else scanned_result(space_factor)(int_val)
    else {
        scanned_result!(
            $globals,
            space_factor!($globals) as _,
            cur_val_level_kind::int_val
        );
    }
    use crate::pascal::integer;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
    use crate::section_0213::prev_depth;
    use crate::section_0213::space_factor;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0413::scanned_result;
}}
