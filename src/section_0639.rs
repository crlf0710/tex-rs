//! ` `
// @<Flush the box from memory, showing statistics if requested@>=
macro_rules! Flush_the_box_from_memory__showing_statistics_if_requested {
    ($globals:expr, $p:expr) => {{
        // @!stat if tracing_stats>1 then
        region_stat! {
            if tracing_stats!($globals) > 1 {
                // begin print_nl("Memory usage before: ");
                // @.Memory usage...@>
                // print_int(var_used); print_char("&");
                // print_int(dyn_used); print_char(";");
                // end;
                todo!("memory usage stat");
            }
            // tats@/
        };
        // flush_node_list(p);
        flush_node_list($globals, $p)?;
        // @!stat if tracing_stats>1 then
        region_stat! {
            if tracing_stats!($globals) > 1 {
                // begin print(" after: ");
                // print_int(var_used); print_char("&");
                // print_int(dyn_used); print("; still untouched: ");
                // print_int(hi_mem_min-lo_mem_max-1); print_ln;
                // end;
                todo!("memory usage stat");
            }
            // tats
        };
        use crate::section_0202::flush_node_list;
    }}
}
