//! @ @d print_plus_end(#)==print(#);@+end
//! @d print_plus(#)==if page_so_far[#]<>0 then
//!   begin print(" plus "); print_scaled(page_so_far[#]); print_plus_end
//!
//! @p procedure print_totals;
//! begin print_scaled(page_total);
//! print_plus(2)("");
//! print_plus(3)("fil");
//! print_plus(4)("fill");
//! print_plus(5)("filll");
//! if page_shrink<>0 then
//!   begin print(" minus "); print_scaled(page_shrink);
//!   end;
//! end;
//!
