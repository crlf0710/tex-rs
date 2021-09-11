//! ` `

// @<Print the banner...@>=
pub(crate) macro Print_the_banner_line__including_the_date_and_time($globals:expr) {{
    // begin wlog(banner);
    wlog(make_globals_log_view!($globals), banner);
    // slow_print(format_ident); print("  ");
    slow_print($globals, $globals.format_ident.get() as _);
    print($globals, crate::strpool_str!("  ").get() as _);
    // print_int(sys_day); print_char(" ");
    print_int($globals, $globals.sys_day);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b' '),
    );
    // months:='JANFEBMARAPRMAYJUNJULAUGSEPOCTNOVDEC';
    let months = b" JANFEBMARAPRMAYJUNJULAUGSEPOCTNOVDEC";
    /// NOTE: pascal string stores its length at the first byte
    const _: () = ();
    // for k:=3*sys_month-2 to 3*sys_month do wlog(months[k]);
    for k in 3 * $globals.sys_month - 2..=3 * $globals.sys_month {
        wlog(make_globals_log_view!($globals), months[k as usize] as char);
    }
    // print_char(" "); print_int(sys_year); print_char(" ");
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b' '),
    );
    print_int($globals, $globals.sys_year);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b' '),
    );
    // print_two(sys_time div 60); print_char(":"); print_two(sys_time mod 60);
    print_two($globals, $globals.sys_time / 60);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b':'),
    );
    print_two($globals, $globals.sys_time % 60);
    // end
    use crate::section_0002::banner;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::make_globals_log_view;
    use crate::section_0004::TeXGlobalsLogView;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0056::wlog;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0060::slow_print;
    use crate::section_0065::print_int;
    use crate::section_0066::print_two;
}}
