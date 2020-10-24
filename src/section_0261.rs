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
            // for k:=j+1 to j+l-1 do
            //   begin h:=h+h+buffer[k];
            //   while h>=hash_prime do h:=h-hash_prime;
            //   end
        }
        #[cfg(feature = "unicode_support")]
        {
            $h = 0;
            todo!();
        }
    }
}
