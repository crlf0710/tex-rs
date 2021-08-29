//! @* \[42] Hyphenation.
//! When a word |hc[1..hn]| has been set up to contain a candidate for hyphenation,
//! \TeX\ first looks to see if it is in the user's exception dictionary. If not,
//! hyphens are inserted based on patterns that appear within the given word,
//! using an algorithm due to Frank~M. Liang.
//! @^Liang, Franklin Mark@>
//!
//! Let's consider Liang's method first, since it is much more interesting than the
//! exception-lookup routine.  The algorithm begins by setting |hyf[j]| to zero
//! for all |j|, and invalid characters are inserted into |hc[0]|
//! and |hc[hn+1]| to serve as delimiters. Then a reasonably fast method is
//! used to see which of a given set of patterns occurs in the word
//! |hc[0..(hn+1)]|. Each pattern $p_1\ldots p_k$ of length |k| has an associated
//! sequence of |k+1| numbers $n_0\ldots n_k$; and if the pattern occurs in
//! |hc[(j+1)..(j+k)]|, \TeX\ will set |hyf[j+i]:=@tmax@>(hyf[j+i],@t$n_i$@>)| for
//! |0<=i<=k|. After this has been done for each pattern that occurs, a
//! discretionary hyphen will be inserted between |hc[j]| and |hc[j+1]| when
//! |hyf[j]| is odd, as we have already seen.
//!
//! The set of patterns $p_1\ldots p_k$ and associated numbers $n_0\ldots n_k$
//! depends, of course, on the language whose words are being hyphenated, and
//! on the degree of hyphenation that is desired. A method for finding
//! appropriate |p|'s and |n|'s, from a given dictionary of words and acceptable
//! hyphenations, is discussed in Liang's Ph.D. thesis (Stanford University,
//! 1983); \TeX\ simply starts with the patterns and works from there.
//!
