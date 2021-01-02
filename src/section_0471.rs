//! ` `

// @<Scan the argument for command |c|@>=
macro_rules! Scan_the_argument_for_command_c {
    ($globals:expr, $c:expr, $save_scanner_status:expr) => {{
        // case c of
        match $c {
            // number_code,roman_numeral_code: scan_int;
            convert_code_kind::number_code | convert_code_kind::roman_numeral_code => {
                scan_int($globals)?;
            }
            // string_code, meaning_code: begin save_scanner_status:=scanner_status;
            convert_code_kind::string_code | convert_code_kind::meaning_code => {
                $save_scanner_status = $globals.scanner_status;
                // scanner_status:=normal; get_token; scanner_status:=save_scanner_status;
                $globals.scanner_status = scanner_status_kind::normal;
                get_token($globals)?;
                $globals.scanner_status = $save_scanner_status;
                // end;
            }
            // font_name_code: scan_font_ident;
            convert_code_kind::font_name_code => {
                scan_font_ident($globals)?;
            }
            // job_name_code: if job_name=0 then open_log_file;
            convert_code_kind::job_name_code => {
                if $globals.job_name == 0 {
                    open_log_file($globals);
                }
            }
        }
        // end {there are no other cases}
        use crate::section_0305::scanner_status_kind;
        use crate::section_0365::get_token;
        use crate::section_0440::scan_int;
        use crate::section_0534::open_log_file;
        use crate::section_0577::scan_font_ident;
    }}
}