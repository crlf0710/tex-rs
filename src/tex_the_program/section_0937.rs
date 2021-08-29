//! ` `

// @<Append a new letter or hyphen@>=
macro_rules! Append_a_new_letter_or_hyphen {
    ($globals:expr, $n:expr, $p:expr) => {{
        // if cur_chr="-" then @<Append the value |n| to list |p|@>
        if $globals.cur_chr.get() == b'-' as chr_code_repr {
            Append_the_value_n_to_list_p!($globals, $n, $p);
        }
        // else  begin if lc_code(cur_chr)=0 then
        else {
            if lc_code!(
                $globals,
                ASCII_code::from($globals.cur_chr.get() as integer)
            ) == 0
            {
                // begin print_err("Not a letter");
                print_err!($globals, strpool_str!("Not a letter"));
                // @.Not a letter@>
                // help2("Letters in \hyphenation words must have \lccode>0.")@/
                //   ("Proceed; I'll ignore the character I just read.");
                help2!($globals, strpool_str!("Letters in \\hyphenation words must have \\lccode>0."),
                    strpool_str!("Proceed; I'll ignore the character I just read."));
                // error;
                error($globals)?;
                // end
            }
            // else if n<63 then
            else if $n < 63 {
                // begin incr(n); hc[n]:=lc_code(cur_chr);
                incr!($n);
                $globals.hc[$n.get() as usize] = ASCII_code::from(lc_code!(
                    $globals,
                    ASCII_code::from($globals.cur_chr.get() as integer)
                ) as integer);
                // end;
            }
            // end
        }
        use crate::pascal::integer;
        use crate::section_0018::ASCII_code;
        use crate::section_0082::error;
        use crate::section_0297::chr_code_repr;
    }};
}
