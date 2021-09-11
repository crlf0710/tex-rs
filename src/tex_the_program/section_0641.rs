//! @ Sometimes the user will generate a huge page because other error messages
//! are being ignored. Such pages are not output to the \.{dvi} file, since they
//! may confuse the printing software.
//
// @<Update the values of |max_h| and |max_v|; but if the page is too large...@>=
pub(crate) macro Update_the_values_of_max_h_and_max_v__but_if_the_page_is_too_large__goto_done($globals:expr, $p:expr) {{
    // if (height(p)>max_dimen)or@|(depth(p)>max_dimen)or@|
    //    (height(p)+depth(p)+v_offset>max_dimen)or@|
    //    (width(p)+h_offset>max_dimen) then
    if height!($globals, $p) > scaled::new_from_inner(max_dimen)
        || depth!($globals, $p) > scaled::new_from_inner(max_dimen)
        || height!($globals, $p) + depth!($globals, $p) + v_offset!($globals)
            > scaled::new_from_inner(max_dimen)
        || width!($globals, $p) + h_offset!($globals) > scaled::new_from_inner(max_dimen)
    {
        //   begin print_err("Huge page cannot be shipped out");
        // @.Huge page...@>
        //   help2("The page just created is more than 18 feet tall or")@/
        //    ("more than 18 feet wide, so I suspect something went wrong.");
        //   error;
        //   if tracing_output<=0 then
        //     begin begin_diagnostic;
        //     print_nl("The following box has been deleted:");
        // @.The following...deleted@>
        //     show_box(p);
        //     end_diagnostic(true);
        //     end;
        //   goto done;
        //   end;
        todo!("update the values of max_h and max_v");
    }
    // if height(p)+depth(p)+v_offset>max_v then max_v:=height(p)+depth(p)+v_offset;
    if height!($globals, $p) + depth!($globals, $p) + v_offset!($globals) > $globals.max_v {
        $globals.max_v = height!($globals, $p) + depth!($globals, $p) + v_offset!($globals);
    }
    // if width(p)+h_offset>max_h then max_h:=width(p)+h_offset
    if width!($globals, $p) + h_offset!($globals) > $globals.max_h {
        $globals.max_h = width!($globals, $p) + h_offset!($globals);
    }
    use crate::section_0101::scaled;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0247::h_offset;
    use crate::section_0247::v_offset;
    use crate::section_0421::max_dimen;
}}
