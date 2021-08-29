//! @ The table of equivalents usually contains repeated information, so we dump it
//! in compressed form: The sequence of $n+2$ values $(n,x_1,\ldots,x_n,m)$ in the
//! format file represents $n+m$ consecutive entries of |eqtb|, with |m| extra
//! copies of $x_n$, namely $(x_1,\ldots,x_n,x_n,\ldots,x_n)$.
//
// @<Dump regions 1 to 4 of |eqtb|@>=
macro_rules! Dump_regions_1_to_4_of_eqtb {
    ($globals:expr, $k:expr) => {{
        /// all-purpose indices
        let (mut j, mut k, mut l): (integer, integer, integer);
        // k:=active_base;
        k = active_base as integer;
        // repeat j:=k;
        loop {
            region_forward_label!(
            |'done1|
            {
            region_forward_label!(
            |'found1|
            {
            j = k;
            // while j<int_base-1 do
            while j < int_base as integer - 1 {
                // begin if (equiv(j)=equiv(j+1))and(eq_type(j)=eq_type(j+1))and@|
                //   (eq_level(j)=eq_level(j+1)) then goto found1;
                if equiv!($globals, j) == equiv!($globals, j + 1) &&
                    eq_type!($globals, j) == eq_type!($globals, j + 1) &&
                    eq_level!($globals, j) == eq_level!($globals, j + 1) {
                    goto_forward_label!('found1);
                }
                // incr(j);
                incr!(j);
                // end;
            }
            // l:=int_base; goto done1; {|j=int_base-1|}
            l = int_base as integer;
            /// `j=int_base-1`
            const _ : () = ();
            goto_forward_label!('done1);
            }
            // found1: incr(j); l:=j;
            'found1 <- 
            );
            incr!(j);
            l = j;
            // while j<int_base-1 do
            while j < int_base as integer - 1 {
                // begin if (equiv(j)<>equiv(j+1))or(eq_type(j)<>eq_type(j+1))or@|
                //   (eq_level(j)<>eq_level(j+1)) then goto done1;
                if equiv!($globals, j) != equiv!($globals, j + 1) ||
                    eq_type!($globals, j) != eq_type!($globals, j + 1) ||
                    eq_level!($globals, j) != eq_level!($globals, j + 1) {
                    goto_forward_label!('done1);
                }
                // incr(j);
                incr!(j);
                // end;
            }
            }
            // done1:dump_int(l-k);
            'done1 <-
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
            // until k=int_base
            if k == int_base as integer {
                break;
            }
        }
        $k = k;
        use crate::pascal::integer;
        use crate::section_0222::active_base;
        use crate::section_0230::int_base;
    }}
}
