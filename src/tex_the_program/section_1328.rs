//! ` `
// @<Create the |format_ident|...@>=
macro_rules! Create_the_format_ident__open_the_format_file__and_inform_the_user_that_dumping_has_begun {
    ($globals:expr) => {{
        // selector:=new_string;
        $globals.selector = new_string.into();
        // print(" (preloaded format="); print(job_name); print_char(" ");
        print($globals, strpool_str!(" (preloaded format=").get() as _);
        print($globals, $globals.job_name.get() as _);
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b' '),
        );
        // print_int(year); print_char(".");
        print_int($globals, year!($globals));
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'.'),
        );
        // print_int(month); print_char("."); print_int(day); print_char(")");
        print_int($globals, month!($globals));
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'.'),
        );
        print_int($globals, day!($globals));
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b')'),
        );
        // if interaction=batch_mode then selector:=log_only
        if $globals.interaction == batch_mode {
            $globals.selector = log_only.into();
        }
        // else selector:=term_and_log;
        else {
            $globals.selector = term_and_log.into();
        }
        // str_room(1);
        str_room($globals, 1);
        // format_ident:=make_string;
        $globals.format_ident = make_string(make_globals_string_view!($globals));
        // pack_job_name(format_extension);
        pack_job_name($globals, strpool_str!(format_extension));
        // while not w_open_out(fmt_file) do
        while !w_open_out(
            make_globals_filename_view!($globals),
            &mut $globals.fmt_file,
        ) {
            // prompt_file_name("format file name",format_extension);
            prompt_file_name(
                $globals,
                strpool_str!("format file name"),
                strpool_str!(format_extension),
            )?;
        }
        // print_nl("Beginning to dump on file ");
        print_nl($globals, strpool_str!("Beginning to dump on file "));
        // @.Beginning to dump...@>
        // slow_print(w_make_name_string(fmt_file)); flush_string;
        let name_str = w_make_name_string(
            make_globals_io_string_view!($globals),
            &mut $globals.fmt_file,
        );
        slow_print($globals, name_str.get() as _);
        flush_string($globals);
        // print_nl(""); slow_print(format_ident)
        print_nl($globals, strpool_str!(""));
        slow_print($globals, $globals.format_ident.get() as _);
        use crate::section_0004::TeXGlobalsFilenameView;
        use crate::section_0004::TeXGlobalsIoStringLogView;
        use crate::section_0004::TeXGlobalsIoStringView;
        use crate::section_0004::TeXGlobalsStringView;
        use crate::section_0027::w_open_out;
        use crate::section_0042::str_room;
        use crate::section_0043::make_string;
        use crate::section_0044::flush_string;
        use crate::section_0054::log_only;
        use crate::section_0054::new_string;
        use crate::section_0054::term_and_log;
        use crate::section_0058::print_char;
        use crate::section_0059::print;
        use crate::section_0060::slow_print;
        use crate::section_0062::print_nl;
        use crate::section_0065::print_int;
        use crate::section_0073::batch_mode;
        use crate::section_0520::format_extension;
        use crate::section_0525::w_make_name_string;
        use crate::section_0529::pack_job_name;
        use crate::section_0530::prompt_file_name;
    }};
}
