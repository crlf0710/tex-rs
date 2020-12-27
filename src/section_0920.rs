//! @ The patterns are stored in a compact table that is also efficient for
//! retrieval, using a variant of ``trie memory'' [cf.\ {\sl The Art of
//! Computer Programming \bf3} (1973), 481--505]. We can find each pattern
//! $p_1\ldots p_k$ by letting $z_0$ be one greater than the relevant language
//! index and then, for |1<=i<=k|,
//! setting |@t$z_i$@>:=trie_link@t$(z_{i-1})+p_i$@>|; the pattern will be
//! identified by the number $z_k$. Since all the pattern information is
//! packed together into a single |trie_link| array, it is necessary to
//! prevent confusion between the data from inequivalent patterns, so another
//! table is provided such that |trie_char@t$(z_i)=p_i$@>| for all |i|. There
//! is also a table |trie_op|$(z_k)$ to identify the numbers $n_0\ldots n_k$
//! associated with $p_1\ldots p_k$.
//!
//! Comparatively few different number sequences $n_0\ldots n_k$ actually occur,
//! since most of the |n|'s are generally zero. Therefore the number sequences
//! are encoded in such a way that |trie_op|$(z_k)$ is only one byte long.
//! If |trie_op(@t$z_k$@>)<>min_quarterword|, when $p_1\ldots p_k$ has matched
//! the letters in |hc[(l-k+1)..l@,]| of language |t|,
//! we perform all of the required operations
//! for this pattern by carrying out the following little program: Set
//! |v:=trie_op(@t$z_k$@>)|. Then set |v:=v+op_start[t]|,
//! |hyf[l-hyf_distance[v]]:=@tmax@>(hyf[l-hyf_distance[v]], hyf_num[v])|,
//! and |v:=hyf_next[v]|; repeat, if necessary, until |v=min_quarterword|.
//
// @<Types...@>=
// @!trie_pointer=0..trie_size; {an index into |trie|}
/// an index into `trie`
pub(crate) type trie_pointer = u16_from_0_to_n<trie_size_TYPENUM>;

use crate::pascal::u16_from_0_to_n;
use crate::section_0011::trie_size_TYPENUM;