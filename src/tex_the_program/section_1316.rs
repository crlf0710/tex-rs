//! ` `
// @<Dump regions 5 and 6 of |eqtb|@>=
pub(crate) macro Dump_regions_5_and_6_of_eqtb
    ($globals:expr, $k:expr) {{
        /// all-purpose indices
        let (mut j, mut k, mut l): (integer, integer, integer);
        k = $k;
        // repeat j:=k;
        loop {
            crate::region_forward_label!(
            |'done2|
            {
            crate::region_forward_label!(
            |'found2|
            {
            j = k;
            // while j<eqtb_size do
            while j < eqtb_size as integer {
                // begin if eqtb[j].int=eqtb[j+1].int then goto found2;
                if $globals.eqtb[j as u16][MEMORY_WORD_INT] ==
                    $globals.eqtb[j as u16 + 1][MEMORY_WORD_INT] {
                    crate::goto_forward_label!('found2);
                }
                // incr(j);
                incr!(j);
                // end;
            }
            // l:=eqtb_size+1; goto done2; {|j=eqtb_size|}
            l = eqtb_size as integer + 1;
            /// `j=eqtb_size`
            const _ : () = ();
            crate::goto_forward_label!('done2);
            }
            // found2: incr(j); l:=j;
            'found2 <-
            );
            incr!(j);
            l = j;
            // while j<eqtb_size do
            while j < eqtb_size as integer {
                // begin if eqtb[j].int<>eqtb[j+1].int then goto done2;
                if $globals.eqtb[j as u16][MEMORY_WORD_INT] !=
                    $globals.eqtb[j as u16 + 1][MEMORY_WORD_INT] {
                    crate::goto_forward_label!('done2);
                }
                // incr(j);
                incr!(j);
                // end;
            }
            }
            // done2:dump_int(l-k);
            'done2 <-
            );
            dump_int!($globals, l - k);
            // while k<l do
            while k < l {
                // begin dump_wd(eqtb[k]); incr(k);
                dump_wd!($globals, $globals.eqtb[k as u16]);
                incr!(k);
                // end;
            }
            // k:=j+1; dump_int(k-l);
            k = j + 1;
            dump_int!($globals, k - l);
            // until k>eqtb_size
            if k > eqtb_size as integer {
                break;
            }
        }
        use crate::pascal::integer;
        use crate::section_0113::MEMORY_WORD_INT;
        use crate::section_0247::eqtb_size;
        use crate::section_0016::incr;
        use crate::section_1305::dump_int;
        use crate::section_1305::dump_wd;
    }}
