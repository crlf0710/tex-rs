//! ` `
// @<Dump the font information@>=
pub(crate) macro Dump_the_font_information($globals:expr) {{
    // dump_int(fmem_ptr);
    dump_int!($globals, $globals.fmem_ptr.get() as _);
    // for k:=0 to fmem_ptr-1 do dump_wd(font_info[k]);
    for k in 0..=$globals.fmem_ptr.get() - 1 {
        dump_wd!($globals, $globals.font_info[k]);
    }
    // dump_int(font_ptr);
    dump_int!($globals, $globals.font_ptr.get() as _);
    // for k:=null_font to font_ptr do
    for k in null_font.get()..=$globals.font_ptr.get() {
        // @<Dump the array info for internal font number |k|@>;
        crate::section_1322::Dump_the_array_info_for_internal_font_number_k!($globals, k);
    }
    // print_ln; print_int(fmem_ptr-7); print(" words of font info for ");
    print_ln(make_globals_io_string_log_view!($globals));
    print_int($globals, $globals.fmem_ptr.get() as integer - 7);
    print(
        $globals,
        crate::strpool_str!(" words of font info for ").get() as _,
    );
    // print_int(font_ptr-font_base); print(" preloaded font");
    print_int(
        $globals,
        $globals.font_ptr.get() as integer - font_base as integer,
    );
    print($globals, crate::strpool_str!(" preloaded font").get() as _);
    // if font_ptr<>font_base+1 then print_char("s")
    if $globals.font_ptr.get() as integer != font_base as integer + 1 {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b's'),
        );
    }

    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0012::font_base;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0057::print_ln;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0065::print_int;
    use crate::section_0232::null_font;
    use crate::section_1305::dump_int;
    use crate::section_1305::dump_wd;
}}
