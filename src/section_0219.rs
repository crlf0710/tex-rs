//! ` `
// @<Show the auxiliary...@>=
macro_rules! Show_the_auxiliary_field__a {
    ($globals:expr, $m:expr, $a:expr) => {{
        // case abs(m) div (max_command+1) of
        match $m.get().abs() as integer / (max_command as integer + 1) {
            // 0: begin print_nl("prevdepth ");
            0 => {
                todo!("abs(m) div (mc + 1) == 0");
                // if a.sc<=ignore_depth then print("ignored")
                // else print_scaled(a.sc);
                // if nest[p].pg_field<>0 then
                //   begin print(", prevgraf ");
                //   print_int(nest[p].pg_field); print(" line");
                //   if nest[p].pg_field<>1 then print_char("s");
                //   end;
                // end;
            }
            // 1: begin print_nl("spacefactor "); print_int(a.hh.lh);
            1 => {
                print_nl($globals, strpool_str!("spacefactor "));
                print_int($globals, $a[MEMORY_WORD_HH_LH] as _);
                // if m>0 then@+ if a.hh.rh>0 then
                if $m > 0 && $a[MEMORY_WORD_HH_RH] > 0 {
                    // begin print(", current language "); print_int(a.hh.rh);@+
                    print($globals, strpool_str!(", current language ").get() as _);
                    print_int($globals, $a[MEMORY_WORD_HH_RH] as _);
                    // end;
                }
                // end;
            }
            // 2: if a.int<>null then
            2 => {
                if $a[MEMORY_WORD_INT] != null as integer {
                    // begin print("this will be denominator of:"); show_box(a.int);@+
                    print($globals, strpool_str!("this will be denominator of:").get() as _);
                    show_box($globals, $a[MEMORY_WORD_INT] as _);
                }
                // end;
            }
            // end {there are no other cases}
            _ => {
                /// there are no other cases
                unreachable!();
            }
        }
        use crate::pascal::integer;
        use crate::section_0113::MEMORY_WORD_INT;
        use crate::section_0113::MEMORY_WORD_HH_LH;
        use crate::section_0113::MEMORY_WORD_HH_RH;
        use crate::section_0115::null;
        use crate::section_0209::max_command;
    }}
}