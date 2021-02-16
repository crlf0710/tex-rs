//! @ @<Flush the box from memory, showing statistics if requested@>=
//! @!stat if tracing_stats>1 then
//!   begin print_nl("Memory usage before: ");
//! @.Memory usage...@>
//!   print_int(var_used); print_char("&");
//!   print_int(dyn_used); print_char(";");
//!   end;
//! tats@/
//! flush_node_list(p);
//! @!stat if tracing_stats>1 then
//!   begin print(" after: ");
//!   print_int(var_used); print_char("&");
//!   print_int(dyn_used); print("; still untouched: ");
//!   print_int(hi_mem_min-lo_mem_max-1); print_ln;
//!   end;
//! tats
//!
