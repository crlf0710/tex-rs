//! ` `
// @<Compute the demerits, |d|, from |r| to |cur_p|@>=
pub(crate) macro Compute_the_demerits_d_from_r_to_cur_p($globals:expr, $r:expr, $d:expr, $b:expr, $pi:expr, $break_type:expr, $fit_class:expr) {{
    // begin d:=line_penalty+b;
    $d = line_penalty!($globals) + $b as integer;
    // if abs(d)>=10000 then d:=100000000@+else d:=d*d;
    if $d.abs() >= 10000 {
        $d = 100000000;
    } else {
        $d = $d * $d;
    }
    // if pi<>0 then
    if $pi != 0 {
        // if pi>0 then d:=d+pi*pi
        if $pi > 0 {
            $d = $d + $pi * $pi;
        }
        // else if pi>eject_penalty then d:=d-pi*pi;
        else if $pi > eject_penalty {
            $d = $d - $pi * $pi;
        }
    }
    // if (break_type=hyphenated)and(type(r)=hyphenated) then
    if $break_type == hyphenated && r#type!($globals, $r) == hyphenated {
        // if cur_p<>null then d:=d+double_hyphen_demerits
        if $globals.cur_p != null {
            $d += double_hyphen_demerits!($globals);
        }
        // else d:=d+final_hyphen_demerits;
        else {
            $d += final_hyphen_demerits!($globals);
        }
    }
    // if abs(fit_class-fitness(r))>1 then d:=d+adj_demerits;
    if ($fit_class as integer - fitness!($globals, $r) as integer).abs() > 1 {
        $d = $d + adj_demerits!($globals);
    }
    // end
    use crate::pascal::integer;
    use crate::section_0115::null;
    use crate::section_0133::r#type;
    use crate::section_0157::eject_penalty;
    use crate::section_0236::adj_demerits;
    use crate::section_0236::double_hyphen_demerits;
    use crate::section_0236::final_hyphen_demerits;
    use crate::section_0236::line_penalty;
    use crate::section_0819::fitness;
    use crate::section_0819::hyphenated;
}}
