//! @ When we get to the present part of the program, |x| is the natural width
//! of the box being packaged.
//!
//! @<Determine the value of |width(r)| and the appropriate glue setting...@>=
//! if m=additional then w:=x+w;
//! width(r):=w; x:=w-x; {now |x| is the excess to be made up}
//! if x=0 then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   return;
//!   end
//! else if x>0 then @<Determine horizontal glue stretch setting, then |return|
//!     or \hbox{|goto common_ending|}@>
//! else @<Determine horizontal glue shrink setting, then |return|
//!     or \hbox{|goto common_ending|}@>
//!
