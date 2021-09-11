//! ` `

// @<Finish issuing a diagnostic message for an overfull or underfull hbox@>=
pub(crate) macro Finish_issuing_a_diagnostic_message_for_an_overfull_or_underfull_hbox($globals:expr, $r:expr) {{
    // if output_active then print(") has occurred while \output is active")
    if $globals.output_active {
        print(
            $globals,
            crate::strpool_str!(") has occurred while \\output is active").get() as _,
        );
    }
    // else  begin if pack_begin_line<>0 then
    else {
        if $globals.pack_begin_line != 0 {
            // begin if pack_begin_line>0 then print(") in paragraph at lines ")
            if $globals.pack_begin_line > 0 {
                print(
                    $globals,
                    crate::strpool_str!(") in paragraph at lines ").get() as _,
                );
            }
            // else print(") in alignment at lines ");
            else {
                print(
                    $globals,
                    crate::strpool_str!(") in alignment at lines ").get() as _,
                );
            }
            // print_int(abs(pack_begin_line));
            print_int($globals, $globals.pack_begin_line.abs());
            // print("--");
            print($globals, crate::strpool_str!("--").get() as _);
            // end
        }
        // else print(") detected at line ");
        else {
            print(
                $globals,
                crate::strpool_str!(") detected at line ").get() as _,
            );
        }
        // print_int(line);
        print_int($globals, $globals.line);
        // end;
    }
    // print_ln;@/
    print_ln(make_globals_io_string_log_view!($globals));
    // font_in_short_display:=null_font; short_display(list_ptr(r)); print_ln;@/
    $globals.font_in_short_display = null_font.get() as _;
    short_display($globals, list_ptr!($globals, $r) as _);
    print_ln(make_globals_io_string_log_view!($globals));
    // begin_diagnostic; show_box(r); end_diagnostic(true)
    begin_diagnostic($globals);
    show_box($globals, $r);
    end_diagnostic($globals, true);
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0057::print_ln;
    use crate::section_0059::print;
    use crate::section_0065::print_int;
    use crate::section_0135::list_ptr;
    use crate::section_0174::short_display;
    use crate::section_0198::show_box;
    use crate::section_0232::null_font;
    use crate::section_0245::begin_diagnostic;
    use crate::section_0245::end_diagnostic;
}}
