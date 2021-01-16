//! @ @<Put each...@>=
//! primitive("pagegoal",set_page_dimen,0);
//! @!@:page_goal_}{\.{\\pagegoal} primitive@>
//! primitive("pagetotal",set_page_dimen,1);
//! @!@:page_total_}{\.{\\pagetotal} primitive@>
//! primitive("pagestretch",set_page_dimen,2);
//! @!@:page_stretch_}{\.{\\pagestretch} primitive@>
//! primitive("pagefilstretch",set_page_dimen,3);
//! @!@:page_fil_stretch_}{\.{\\pagefilstretch} primitive@>
//! primitive("pagefillstretch",set_page_dimen,4);
//! @!@:page_fill_stretch_}{\.{\\pagefillstretch} primitive@>
//! primitive("pagefilllstretch",set_page_dimen,5);
//! @!@:page_filll_stretch_}{\.{\\pagefilllstretch} primitive@>
//! primitive("pageshrink",set_page_dimen,6);
//! @!@:page_shrink_}{\.{\\pageshrink} primitive@>
//! primitive("pagedepth",set_page_dimen,7);
//! @!@:page_depth_}{\.{\\pagedepth} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_page_dimen: case chr_code of
//! 0: print_esc("pagegoal");
//! 1: print_esc("pagetotal");
//! 2: print_esc("pagestretch");
//! 3: print_esc("pagefilstretch");
//! 4: print_esc("pagefillstretch");
//! 5: print_esc("pagefilllstretch");
//! 6: print_esc("pageshrink");
//! othercases print_esc("pagedepth")
//! endcases;
//!
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
