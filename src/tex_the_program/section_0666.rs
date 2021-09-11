//! ` `

// @<Report an overfull hbox and |goto common_ending|, if...@>=
pub(crate) macro Report_an_overfull_hbox_and_goto_common_ending__if_this_box_is_sufficiently_bad($globals:expr, $x:expr, $q:expr, $lbl_common_ending:lifetime) {{
    // if (-x-total_shrink[normal]>hfuzz)or(hbadness<100) then
    if -$x - $globals.total_shrink[glue_ord::normal] > hfuzz!($globals) || hbadness!($globals) < 100
    {
        // begin if (overfull_rule>0)and(-x-total_shrink[normal]>hfuzz) then
        if overfull_rule!($globals) > scaled::zero()
            && -$x - $globals.total_shrink[glue_ord::normal] > hfuzz!($globals)
        {
            // begin while link(q)<>null do q:=link(q);
            while link!($globals, $q) != null {
                $q = link!($globals, $q);
            }
            // link(q):=new_rule;
            let rule = new_rule($globals)?;
            link!($globals, $q) = rule;
            // width(link(q)):=overfull_rule;
            width!($globals, rule) = overfull_rule!($globals);
            // end;
        }
        // print_ln; print_nl("Overfull \hbox (");
        print_ln(make_globals_io_string_log_view!($globals));
        print_nl($globals, crate::strpool_str!("Overfull \\hbox ("));
        // @.Overfull \\hbox...@>
        // print_scaled(-x-total_shrink[normal]); print("pt too wide");
        print_scaled($globals, -$x - $globals.total_shrink[glue_ord::normal]);
        print($globals, crate::strpool_str!("pt too wide").get() as _);
        // goto common_ending;
        crate::goto_forward_label!($lbl_common_ending);
        // end
    }
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0057::print_ln;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0101::scaled;
    use crate::section_0103::print_scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0135::width;
    use crate::section_0139::new_rule;
    use crate::section_0150::glue_ord;
    use crate::section_0236::hbadness;
    use crate::section_0247::hfuzz;
    use crate::section_0247::overfull_rule;
}}
