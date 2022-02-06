//! ` `
// @<Compute the badness, |b|, using |awful_bad| if the box is too full@>=
#[allow(unused_macros)]
pub(crate) macro Compute_the_badness_b__using_awful_bad_if_the_box_is_too_full($globals:expr, $b:expr, $h:expr) {{
    // if cur_height<h then
    if cur_height!($globals) < $h {
        // if (active_height[3]<>0) or (active_height[4]<>0) or
        //   (active_height[5]<>0) then b:=0
        if active_height!($globals)[3] != scaled::zero()
            || active_height!($globals)[4] != scaled::zero()
            || active_height!($globals)[5] != scaled::zero()
        {
            $b = 0;
        }
        // else b:=badness(h-cur_height,active_height[2])
        else {
            $b = badness(
                $globals,
                $h - cur_height!($globals),
                active_height!($globals)[2],
            ) as _;
        }
    }
    // else if cur_height-h>active_height[6] then b:=awful_bad
    else if cur_height!($globals) - $h > active_height!($globals)[6] {
        $b = awful_bad;
    }
    // else b:=badness(cur_height-h,active_height[6])
    else {
        $b = badness(
            $globals,
            cur_height!($globals) - $h,
            active_height!($globals)[6],
        ) as _;
    }
    use crate::section_0101::scaled;
    use crate::section_0108::badness;
    use crate::section_0833::awful_bad;
    use crate::section_0970::active_height;
    use crate::section_0970::cur_height;
}}
