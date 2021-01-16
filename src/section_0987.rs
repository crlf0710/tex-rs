//! @ Here is a procedure that is called when the |page_contents| is changing
//! from |empty| to |inserts_only| or |box_there|.
//!
//! @d set_page_so_far_zero(#)==page_so_far[#]:=0
//!
//! @p procedure freeze_page_specs(@!s:small_number);
//! begin page_contents:=s;
//! page_goal:=vsize; page_max_depth:=max_depth;
//! page_depth:=0; do_all_six(set_page_so_far_zero);
//! least_page_cost:=awful_bad;
//! @!stat if tracing_pages>0 then
//!   begin begin_diagnostic;
//!   print_nl("%% goal height="); print_scaled(page_goal);
//! @.goal height@>
//!   print(", max depth="); print_scaled(page_max_depth);
//!   end_diagnostic(false);
//!   end;@;@+tats@;@/
//! end;
//!
