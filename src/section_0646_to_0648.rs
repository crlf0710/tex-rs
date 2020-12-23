//! @ To figure out the glue setting, |hpack| and |vpack| determine how much
//! stretchability and shrinkability are present, considering all four orders
//! of infinity. The highest order of infinity that has a nonzero coefficient
//! is then used as if no other orders were present.
//!
//! For example, suppose that the given list contains six glue nodes with
//! the respective stretchabilities 3pt, 8fill, 5fil, 6pt, $-3$fil, $-8$fill.
//! Then the total is essentially 2fil; and if a total additional space of 6pt
//! is to be achieved by stretching, the actual amounts of stretch will be
//! 0pt, 0pt, 15pt, 0pt, $-9$pt, and 0pt, since only `fil' glue will be
//! considered. (The `fill' glue is therefore not really stretching infinitely
//! with respect to `fil'; nobody would actually want that to happen.)
//!
//! The arrays |total_stretch| and |total_shrink| are used to determine how much
//! glue of each kind is present. A global variable |last_badness| is used
//! to implement \.{\\badness}.
//!
//! @<Glob...@>=
//! @!total_stretch, @!total_shrink: array[glue_ord] of scaled;
//!   {glue found by |hpack| or |vpack|}
//! @!last_badness:integer; {badness of the most recently packaged box}
//!
//! @ If the global variable |adjust_tail| is non-null, the |hpack| routine
//! also removes all occurrences of |ins_node|, |mark_node|, and |adjust_node|
//! items and appends the resulting material onto the list that ends at
//! location |adjust_tail|.
//!
//! @< Glob...@>=
//! @!adjust_tail:pointer; {tail of adjustment list}
//!
//! @ @<Set init...@>=adjust_tail:=null; last_badness:=0;
//!
