//! @ The value of |hash_prime| should be roughly 85\pct! of |hash_size|, and it
//! should be a prime number.  The theory of hashing tells us to expect fewer
//! than two table probes, on the average, when the search is successful.
//! [See J.~S. Vitter, {\sl Journal of the ACM\/ \bf30} (1983), 231--258.]
//! @^Vitter, Jeffrey Scott@>
//
// @<Compute the hash code |h|@>=
macro_rules! Compute_the_hash_code_h {
    ($globals:expr, $h:expr, $j:expr, $l:expr) => {
        #[cfg(not(feature = "unicode_support"))]
        {
            // h:=buffer[j];
            $h = $globals.buffer[$j as u16].0 as integer;
            // for k:=j+1 to j+l-1 do
            for k in $j + 1..=$j + $l - 1 {
                // begin h:=h+h+buffer[k];
                $h = $h + $h + $globals.buffer[k as u16].0 as integer;
                // while h>=hash_prime do h:=h-hash_prime;
                while $h >= hash_prime {
                    $h = $h - hash_prime;
                }
                // end
            }
        }
        #[cfg(feature = "unicode_support")]
        {
            let runestr = $globals.buffer[crate::pascal::u16_from_m_to_n::from($j as u16)
                ..=crate::pascal::u16_from_m_to_n::from(($j + $l - 1) as u16)]
                .iter()
                .map(|x| x.0)
                .collect::<runestr::RuneString>();
            let mut runestr_bytes = runestr.chars();
            // h:=buffer[j];
            $h = runestr_bytes.next().unwrap() as integer;
            // for k:=j+1 to j+l-1 do
            for v in runestr_bytes {
                // begin h:=h+h+buffer[k];
                $h = $h + $h + v as integer;
                // while h>=hash_prime do h:=h-hash_prime;
                while $h >= hash_prime {
                    $h = $h - hash_prime;
                }
                // end
            }
        }
        use crate::section_0012::hash_prime;
    };
}
