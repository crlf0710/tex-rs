//! ` `
// @<Show the auxiliary...@>=
pub(crate) macro Show_the_auxiliary_field__a($globals:expr, $m:expr, $a:expr, $p:expr) {{
    // case abs(m) div (max_command+1) of
    match $m.get().abs() as integer / (max_command as integer + 1) {
        // 0: begin print_nl("prevdepth ");
        0 => {
            print_nl($globals, crate::strpool_str!("prevdepth "));
            // if a.sc<=ignore_depth then print("ignored")
            if $a[MEMORY_WORD_SC] <= ignore_depth {
                print($globals, crate::strpool_str!("ignored").get() as _);
            } else {
                // else print_scaled(a.sc);
                print_scaled($globals, $a[MEMORY_WORD_SC]);
            }
            // if nest[p].pg_field<>0 then
            if $globals.nest[$p].pg_field != 0 {
                // begin print(", prevgraf ");
                print($globals, crate::strpool_str!(", prevgraf ").get() as _);
                // print_int(nest[p].pg_field); print(" line");
                print_int($globals, $globals.nest[$p].pg_field);
                print($globals, crate::strpool_str!(" line").get() as _);
                // if nest[p].pg_field<>1 then print_char("s");
                if $globals.nest[$p].pg_field != 1 {
                    print_char(
                        make_globals_io_string_log_view!($globals),
                        ASCII_code_literal!(b's'),
                    );
                }
                // end;
            }
            // end;
        }
        // 1: begin print_nl("spacefactor "); print_int(a.hh.lh);
        1 => {
            print_nl($globals, crate::strpool_str!("spacefactor "));
            print_int($globals, $a[MEMORY_WORD_HH_LH] as _);
            // if m>0 then@+ if a.hh.rh>0 then
            if $m > 0 && $a[MEMORY_WORD_HH_RH] > 0 {
                // begin print(", current language "); print_int(a.hh.rh);@+
                print(
                    $globals,
                    crate::strpool_str!(", current language ").get() as _,
                );
                print_int($globals, $a[MEMORY_WORD_HH_RH] as _);
                // end;
            }
            // end;
        }
        // 2: if a.int<>null then
        2 => {
            if $a[MEMORY_WORD_INT] != null as integer {
                // begin print("this will begin denominator of:"); show_box(a.int);@+
                print(
                    $globals,
                    crate::strpool_str!("this will begin denominator of:").get() as _,
                );
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
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0103::print_scaled;
    use crate::section_0113::MEMORY_WORD_HH_LH;
    use crate::section_0113::MEMORY_WORD_HH_RH;
    use crate::section_0113::MEMORY_WORD_INT;
    use crate::section_0115::null;
    use crate::section_0198::show_box;
    use crate::section_0209::max_command;
    use crate::section_0212::ignore_depth;
}}
