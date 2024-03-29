//! ` `
// @<Flush the box from memory, showing statistics if requested@>=
pub(crate) macro Flush_the_box_from_memory__showing_statistics_if_requested {
    ($globals:expr, $p:expr) => {{
        // @!stat if tracing_stats>1 then
        crate::region_stat! {
            if tracing_stats!($globals) > 1 {
                // begin print_nl("Memory usage before: ");
                print_nl($globals, crate::strpool_str!("Memory usage before: "));
                // @.Memory usage...@>
                // print_int(var_used); print_char("&");
                print_int($globals, $globals.var_used);
                print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'&'));
                // print_int(dyn_used); print_char(";");
                print_int($globals, $globals.dyn_used);
                print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b';'));
                // end;
            }
            // tats@/
            use crate::section_0004::make_globals_io_string_log_view;
            use crate::section_0018::ASCII_code_literal;
            use crate::section_0057::print_ln;
            use crate::section_0058::print_char;
            use crate::section_0059::print;
            use crate::section_0062::print_nl;
            use crate::section_0065::print_int;
            use crate::section_0236::tracing_stats;
        };
        crate::region_non_stat! {
            crate::submit_strpool_str!("Memory usage before: ");
        };
        // flush_node_list(p);
        flush_node_list($globals, $p)?;
        // @!stat if tracing_stats>1 then
        crate::region_stat! {
            if tracing_stats!($globals) > 1 {
                // begin print(" after: ");
                print($globals, crate::strpool_str!(" after: ").get() as _);
                // print_int(var_used); print_char("&");
                print_int($globals, $globals.var_used);
                print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'&'));
                // print_int(dyn_used); print("; still untouched: ");
                print_int($globals, $globals.dyn_used);
                print($globals, crate::strpool_str!("; still untouched: ").get() as _);
                // print_int(hi_mem_min-lo_mem_max-1); print_ln;
                print_int($globals, $globals.hi_mem_min as integer - $globals.lo_mem_max as integer - 1);
                print_ln(make_globals_io_string_log_view!($globals));
                // end;
            }
            // tats
            use crate::section_0004::make_globals_io_string_log_view;
            use crate::section_0018::ASCII_code_literal;
            use crate::section_0057::print_ln;
            use crate::section_0058::print_char;
            use crate::section_0059::print;
            use crate::section_0065::print_int;
            use crate::section_0236::tracing_stats;
        };
        crate::region_non_stat! {
            crate::submit_strpool_str!(" after: ");
            crate::submit_strpool_str!("; still untouched: ");
        }
        use crate::pascal::integer;
        use crate::section_0202::flush_node_list;
    }}
}
