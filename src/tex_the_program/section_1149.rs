//! @ A displayed equation is considered to be three lines long, so we
//! calculate the length and offset of line number |prev_graf+2|.
//
// @<Calculate the length, |l|, ...@>=
pub(crate) macro Calculate_the_length__l__and_the_shift_amount__s__of_the_display_lines($globals:expr, $l:expr, $s:expr) {{
    // if par_shape_ptr=null then
    if par_shape_ptr!($globals) == null {
        // if (hang_indent<>0)and@|
        //  (((hang_after>=0)and(prev_graf+2>hang_after))or@|
        //   (prev_graf+1<-hang_after)) then
        if hang_indent!($globals) != scaled::zero()
            && ((hang_after!($globals) >= 0 && prev_graf!($globals) + 2 > hang_after!($globals))
                || prev_graf!($globals) + 1 < -hang_after!($globals))
        {
            // begin l:=hsize-abs(hang_indent);
            $l = hsize!($globals) - hang_indent!($globals).abs();
            // if hang_indent>0 then s:=hang_indent@+else s:=0;
            if hang_indent!($globals) > scaled::zero() {
                $s = hang_indent!($globals);
            } else {
                $s = scaled::zero();
            }
            // end
        }
        // else  begin l:=hsize; s:=0;
        else {
            $l = hsize!($globals);
            $s = scaled::zero();
            // end
        }
    }
    // else  begin n:=info(par_shape_ptr);
    else {
        // if prev_graf+2>=n then p:=par_shape_ptr+2*n
        // else p:=par_shape_ptr+2*(prev_graf+2);
        // s:=mem[p-1].sc; l:=mem[p].sc;
        // end
        todo!("par_shape_ptr != null");
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0213::prev_graf;
    use crate::section_0230::par_shape_ptr;
    use crate::section_0236::hang_after;
    use crate::section_0247::hang_indent;
    use crate::section_0247::hsize;
}}
