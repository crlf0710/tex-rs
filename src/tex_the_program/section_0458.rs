//! @ The necessary conversion factors can all be specified exactly as
//! fractions whose numerator and denominator sum to 32768 or less.
//! According to the definitions here, $\rm2660\,dd\approx1000.33297\,mm$;
//! this agrees well with the value $\rm1000.333\,mm$ cited by Bosshard
//! @^Bosshard, Hans Rudolf@>
//! in {\sl Technische Grundlagen zur Satzherstellung\/} (Bern, 1980).
//
// @d set_conversion_end(#)== denom:=#; end
// @d set_conversion(#)==@+begin num:=#; set_conversion_end
pub(crate) macro set_conversion($num:ident = $num_val:expr, $denom:ident = $denom_val:expr) {{
    $num = $num_val;
    $denom = $denom_val;
}}
//
// @<Scan for \(a)all other units and adjust |cur_val| and |f|...@>=
pub(crate) macro Scan_for_a_all_other_units_and_adjust_cur_val_and_f_accordingly__goto_done_in_the_case_of_scaled_points {
    ($globals:expr, $f:expr, $lbl_done:lifetime) => {{
        /// conversion ratio for the scanned units
        let (num, denom);
        crate::region_forward_label!(
            |'done2|
            {
                // if scan_keyword("in") then set_conversion(7227)(100)
                if scan_keyword($globals, crate::strpool_str!("in"))? {
                    set_conversion!(num = 7227, denom = 100);
                }
                // @.in@>
                // else if scan_keyword("pc") then set_conversion(12)(1)
                else if scan_keyword($globals, crate::strpool_str!("pc"))? {
                    set_conversion!(num = 12, denom = 1);
                }
                // @.pc@>
                // else if scan_keyword("cm") then set_conversion(7227)(254)
                else if scan_keyword($globals, crate::strpool_str!("cm"))? {
                    set_conversion!(num = 7227, denom = 254);
                }
                // @.cm@>
                // else if scan_keyword("mm") then set_conversion(7227)(2540)
                else if scan_keyword($globals, crate::strpool_str!("mm"))? {
                    set_conversion!(num = 7227, denom = 2540);
                }
                // @.mm@>
                // else if scan_keyword("bp") then set_conversion(7227)(7200)
                else if scan_keyword($globals, crate::strpool_str!("bp"))? {
                    set_conversion!(num = 7227, denom = 7200);
                }
                // @.bp@>
                // else if scan_keyword("dd") then set_conversion(1238)(1157)
                else if scan_keyword($globals, crate::strpool_str!("dd"))? {
                    set_conversion!(num = 1238, denom = 1157);
                }
                // @.dd@>
                // else if scan_keyword("cc") then set_conversion(14856)(1157)
                else if scan_keyword($globals, crate::strpool_str!("cc"))? {
                    set_conversion!(num = 14856, denom = 1157);
                }
                // @.cc@>
                // else if scan_keyword("sp") then goto done
                else if scan_keyword($globals, crate::strpool_str!("sp"))? {
                    crate::goto_forward_label!($lbl_done);
                }
                // @.sp@>
                // else @<Complain about unknown unit and |goto done2|@>;
                else {
                    crate::section_0459::Complain_about_unknown_unit_and_goto_done2!($globals, 'done2);
                }
                // cur_val:=xn_over_d(cur_val,num,denom);
                $globals.cur_val = xn_over_d($globals, scaled::new_from_inner($globals.cur_val), num, denom).inner();
                // f:=(num*f+@'200000*remainder) div denom;@/
                $f = (num * $f + 0o200000 * $globals.remainder.inner()) / denom;
                // cur_val:=cur_val+(f div @'200000); f:=f mod @'200000;
                $globals.cur_val = $globals.cur_val + ($f / 0o200000);
                $f = $f % 0o200000;
            }
            // done2:
            'done2 <-
        );
        use crate::section_0101::scaled;
        use crate::section_0107::xn_over_d;
        use crate::section_0407::scan_keyword;
    }}
}
