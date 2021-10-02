//! @ Okay, we're ready to insert the potential hyphenations that were found.
//! When the following program is executed, we want to append the word
//! |hu[1..hn]| after node |ha|, and node |q| should be appended to the result.
//! During this process, the variable |i| will be a temporary
//! index into |hu|; the variable |j| will be an index to our current position
//! in |hu|; the variable |l| will be the counterpart of |j|, in a discretionary
//! branch; the variable |r| will point to new nodes being created; and
//! we need a few new local variables:
//!
//! @<Local variables for hyph...@>=
//! @!major_tail,@!minor_tail:pointer; {the end of lists in the main and
//!   discretionary branches being reconstructed}
//! @!c:ASCII_code; {character temporarily replaced by a hyphen}
//! @!c_loc:0..63; {where that character came from}
//! @!r_count:integer; {replacement count for discretionary}
//! @!hyf_node:pointer; {the hyphen, if it exists}
//!
