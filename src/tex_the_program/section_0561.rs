//! @ There are programs called \.{TFtoPL} and \.{PLtoTF} that convert
//! between the \.{TFM} format and a symbolic property-list format
//! that can be easily edited. These programs contain extensive
//! diagnostic information, so \TeX\ does not have to bother giving
//! precise details about why it rejects a particular \.{TFM} file.
//! @.TFtoPL@> @.PLtoTF@>
//
// @d start_font_error_message==print_err("Font "); sprint_cs(u);
pub(crate) macro start_font_error_message($globals:expr, $nom:expr, $aire:expr, $s:expr) {{
    // print_char("="); print_file_name(nom,aire,"");
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'='),
    );
    print_file_name(
        $globals,
        $nom.get() as _,
        $aire.get() as _,
        crate::strpool_str!("").get() as _,
    );
    // if s>=0 then
    if $s.inner() >= 0 {
        // begin print(" at "); print_scaled(s); print("pt");
        print($globals, crate::strpool_str!(" at ").get() as _);
        print_scaled($globals, $s);
        print($globals, crate::strpool_str!("pt").get() as _);
        // end
    }
    // else if s<>-1000 then
    else if $s.inner() != -1000 {
        // begin print(" scaled "); print_int(-s);
        print($globals, crate::strpool_str!(" scaled ").get() as _);
        print_int($globals, -$s.inner());
        // end
    }
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0065::print_int;
    use crate::section_0103::print_scaled;
    use crate::section_0518::print_file_name;
}}

// @<Report that the font won't be loaded@>=
pub(crate) macro Report_that_the_font_wont_be_loaded($globals:expr, $nom:expr, $aire:expr, $s:expr, $file_opened:expr) {{
    // start_font_error_message;
    start_font_error_message!($globals, $nom, $aire, $s);
    // @.Font x=xx not loadable...@>
    // if file_opened then print(" not loadable: Bad metric (TFM) file")
    if $file_opened {
        print(
            $globals,
            crate::strpool_str!(" not loadable: Bad metric (TFM) file").get() as _,
        );
    }
    // else print(" not loadable: Metric (TFM) file not found");
    else {
        print(
            $globals,
            crate::strpool_str!(" not loadable: Metric (TFM) file not found").get() as _,
        );
    }
    // help5("I wasn't able to read the size data for this font,")@/
    // ("so I will ignore the font specification.")@/
    // ("[Wizards can fix TFM files using TFtoPL/PLtoTF.]")@/
    // ("You might try inserting a different font spec;")@/
    // ("e.g., type `I\font<same font id>=<substitute font name>'.");
    // error
    error($globals)?;
    use crate::section_0059::print;
    use crate::section_0082::error;
}}
