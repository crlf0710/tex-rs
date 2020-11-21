//! @ @<Modify the glue specification in |main_p| according to the space factor@>=
//! if space_factor>=2000 then width(main_p):=width(main_p)+extra_space(cur_font);
//! stretch(main_p):=xn_over_d(stretch(main_p),space_factor,1000);
//! shrink(main_p):=xn_over_d(shrink(main_p),1000,space_factor)
//!
