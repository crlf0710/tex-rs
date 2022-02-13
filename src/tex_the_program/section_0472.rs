//! ` `

// @<Print the result of command |c|@>=
pub(crate) macro Print_the_result_of_command_c($globals:expr, $c:expr) {{
    // case c of
    match $c {
        // number_code: print_int(cur_val);
        convert_code_kind::number_code => {
            print_int($globals, $globals.cur_val);
        }
        // roman_numeral_code: print_roman_int(cur_val);
        convert_code_kind::roman_numeral_code => {
            print_roman_int($globals, $globals.cur_val);
        }
        // string_code:if cur_cs<>0 then sprint_cs(cur_cs)
        //   else print_char(cur_chr);
        convert_code_kind::string_code => {
            if $globals.cur_cs != 0 {
                sprint_cs($globals, $globals.cur_cs);
            } else {
                print_char(
                    make_globals_io_string_log_view!($globals),
                    $globals.cur_chr.into(),
                );
            }
        }
        // meaning_code: print_meaning;
        convert_code_kind::meaning_code => {
            print_meaning($globals);
        }
        // font_name_code: begin print(font_name[cur_val]);
        convert_code_kind::font_name_code => {
            todo!("font_name_code");
            // if font_size[cur_val]<>font_dsize[cur_val] then
            //   begin print(" at "); print_scaled(font_size[cur_val]);
            //   print("pt");
            //   end;
            // end;
        }
        // job_name_code: print(job_name);
        convert_code_kind::job_name_code => {
            print($globals, $globals.job_name.get() as _);
        }
    }
    // end {there are no other cases}
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0065::print_int;
    use crate::section_0069::print_roman_int;
    use crate::section_0263::sprint_cs;
    use crate::section_0296::print_meaning;
    use crate::section_0468::convert_code_kind;
}}
