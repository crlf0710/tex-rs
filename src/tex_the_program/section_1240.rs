//! ` `

// @<Compute result of |multiply| or |divide|...@>=
pub(crate) macro Compute_result_of_multiply_or_divide__put_it_in_cur_val($globals:expr, $l:expr, $p:expr, $q:expr) {{
    // begin scan_int;
    scan_int($globals)?;
    // if p<glue_val then
    if $p < glue_val {
        // if q=multiply then
        if $q == multiply as pointer {
            // if p=int_val then cur_val:=mult_integers(eqtb[l].int,cur_val)
            if $p == int_val {
                $globals.cur_val = mult_integers!(
                    $globals,
                    $globals.eqtb[$l][MEMORY_WORD_INT],
                    $globals.cur_val
                );
            }
            // else cur_val:=nx_plus_y(eqtb[l].int,cur_val,0)
            else {
                $globals.cur_val = nx_plus_y!(
                    $globals,
                    $globals.eqtb[$l][MEMORY_WORD_INT],
                    scaled::new_from_inner($globals.cur_val),
                    scaled::zero()
                )
                .inner() as _;
            }
        }
        // else cur_val:=x_over_n(eqtb[l].int,cur_val)
        else {
            $globals.cur_val = x_over_n(
                $globals,
                scaled::new_from_inner($globals.eqtb[$l][MEMORY_WORD_INT]),
                $globals.cur_val,
            )
            .inner() as _;
        }
    }
    // else  begin s:=equiv(l); r:=new_spec(s);
    else {
        /// for list manipulation
        let (r, s): (pointer, pointer);
        s = equiv!($globals, $l);
        r = new_spec($globals, s)?;
        // if q=multiply then
        if $q == multiply as pointer {
            // begin width(r):=nx_plus_y(width(s),cur_val,0);
            width!($globals, r) = nx_plus_y!(
                $globals,
                width!($globals, s).inner(),
                scaled::new_from_inner($globals.cur_val),
                scaled::zero()
            );
            // stretch(r):=nx_plus_y(stretch(s),cur_val,0);
            stretch!($globals, r) = nx_plus_y!(
                $globals,
                stretch!($globals, s).inner(),
                scaled::new_from_inner($globals.cur_val),
                scaled::zero()
            );
            // shrink(r):=nx_plus_y(shrink(s),cur_val,0);
            shrink!($globals, r) = nx_plus_y!(
                $globals,
                shrink!($globals, s).inner(),
                scaled::new_from_inner($globals.cur_val),
                scaled::zero()
            );
        // end
        }
        // else  begin width(r):=x_over_n(width(s),cur_val);
        else {
            width!($globals, r) = x_over_n($globals, width!($globals, s), $globals.cur_val);
            // stretch(r):=x_over_n(stretch(s),cur_val);
            stretch!($globals, r) = x_over_n($globals, stretch!($globals, s), $globals.cur_val);
            // shrink(r):=x_over_n(shrink(s),cur_val);
            shrink!($globals, r) = x_over_n($globals, shrink!($globals, s), $globals.cur_val);
            // end;
        }
        // cur_val:=r;
        $globals.cur_val = r as _;
        // end;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0105::mult_integers;
    use crate::section_0105::nx_plus_y;
    use crate::section_0106::x_over_n;
    use crate::section_0113::MEMORY_WORD_INT;
    use crate::section_0115::pointer;
    use crate::section_0135::width;
    use crate::section_0150::shrink;
    use crate::section_0150::stretch;
    use crate::section_0151::new_spec;
    use crate::section_0209::*;
    use crate::section_0221::equiv;
    use crate::section_0410::cur_val_level_kind::*;
    use crate::section_0440::scan_int;
}}
