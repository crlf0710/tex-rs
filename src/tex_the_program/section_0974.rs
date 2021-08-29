//! ` `

// @d deplorable==100000 {more than |inf_bad|, but less than |awful_bad|}
/// more than `inf_bad`, but less than `awful_bad`
pub(crate) const deplorable: integer = 100000;

// @<Check if node |p| is a new champion breakpoint; then \(go)...@>=
// if pi<inf_penalty then
//   begin @<Compute the badness, |b|, using |awful_bad|
//     if the box is too full@>;
//   if b<awful_bad then
//     if pi<=eject_penalty then b:=pi
//     else if b<inf_bad then b:=b+pi
//       else b:=deplorable;
//   if b<=least_cost then
//     begin best_place:=p; least_cost:=b;
//     best_height_plus_depth:=cur_height+prev_dp;
//     end;
//   if (b=awful_bad)or(pi<=eject_penalty) then goto done;
//   end
//

use crate::pascal::integer;
