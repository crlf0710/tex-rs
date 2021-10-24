//! @ @<Make the unset node |r| into an |hlist_node| of width |w|...@>=
//! begin height(r):=height(q); depth(r):=depth(q);
//! if t=width(r) then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   end
//! else if t>width(r) then
//!   begin glue_sign(r):=stretching;
//!   if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else glue_set(r):=unfloat((t-width(r))/glue_stretch(r));
//! @^real division@>
//!   end
//! else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
//!   if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else if (glue_order(r)=normal)and(width(r)-t>glue_shrink(r)) then
//!     set_glue_ratio_one(glue_set(r))
//!   else glue_set(r):=unfloat((width(r)-t)/glue_shrink(r));
//!   end;
//! width(r):=w; type(r):=hlist_node;
//! end
//!
