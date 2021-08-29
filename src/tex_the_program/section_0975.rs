//! ` `
// @<Compute the badness, |b|, using |awful_bad| if the box is too full@>=
#[allow(unused_macros)]
macro_rules! Compute_the_badness_b__using_awful_bad_if_the_box_is_too_full {
    ($globals:expr, $b:expr, $h:expr) => {{
        // if cur_height<h then
        if cur_height!($globals) < $h {
            // if (active_height[3]<>0) or (active_height[4]<>0) or
            //   (active_height[5]<>0) then b:=0
            if $globals.active_height[3] != scaled::zero() || $globals.active_height[4] != scaled::zero()
                || $globals.active_height[5] != scaled::zero() {
                $b = 0;
            }
            // else b:=badness(h-cur_height,active_height[2])
            else {
                $b = badness($globals, $h - cur_height!($globals), $globals.active_height[2]);
            }
        }
        // else if cur_height-h>active_height[6] then b:=awful_bad
        else if cur_height!($globals) - $h > $globals.active_height[6] {
            $b = awful_bad;
        }
        // else b:=badness(cur_height-h,active_height[6])
        else {
            $b = badness($globals, cur_height!($globals) - $h, $globals.active_height[6]);
        }
    }}
}
