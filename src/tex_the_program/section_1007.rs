//! ` `
// @<Compute the badness, |b|, of the current page...@>=
pub(crate) macro Compute_the_badness_b__of_the_current_page__using_awful_bad_if_the_box_is_too_full($globals:expr, $b:expr) {{
    // if page_total<page_goal then
    if page_total!($globals) < page_goal!($globals) {
        // if (page_so_far[3]<>0) or (page_so_far[4]<>0) or@|
        //   (page_so_far[5]<>0) then b:=0
        if $globals.page_so_far[3] != scaled::zero()
            || $globals.page_so_far[4] != scaled::zero()
            || $globals.page_so_far[5] != scaled::zero()
        {
            $b = 0;
        }
        // else b:=badness(page_goal-page_total,page_so_far[2])
        else {
            $b = badness(
                $globals,
                page_goal!($globals) - page_total!($globals),
                $globals.page_so_far[2],
            ) as _;
        }
    }
    // else if page_total-page_goal>page_shrink then b:=awful_bad
    else if page_total!($globals) - page_goal!($globals) > page_shrink!($globals) {
        $b = awful_bad;
    }
    // else b:=badness(page_total-page_goal,page_shrink)
    else {
        $b = badness(
            $globals,
            page_total!($globals) - page_goal!($globals),
            page_shrink!($globals),
        ) as _;
    }
    use crate::section_0101::scaled;
    use crate::section_0108::badness;
    use crate::section_0833::awful_bad;
    use crate::section_0982::page_goal;
    use crate::section_0982::page_shrink;
    use crate::section_0982::page_total;
}}
