//! @ We must now face the fact that the battle is not over, even though the
//! {\def\!{\kern-1pt}%
//! hyphens have been found: The process of reconstituting a word can be nontrivial
//! because ligatures might change when a hyphen is present. {\sl The \TeX book\/}
//! discusses the difficulties of the word ``difficult'', and
//! the discretionary material surrounding a
//! hyphen can be considerably more complex than that. Suppose
//! \.{abcdef} is a word in a font for which the only ligatures are \.{b\!c},
//! \.{c\!d}, \.{d\!e}, and \.{e\!f}. If this word permits hyphenation
//! between \.b and \.c, the two patterns with and without hyphenation are
//! $\.a\,\.b\,\.-\,\.{c\!d}\,\.{e\!f}$ and $\.a\,\.{b\!c}\,\.{d\!e}\,\.f$.
//! Thus the insertion of a hyphen might cause effects to ripple arbitrarily
//! far into the rest of the word. A further complication arises if additional
//! hyphens appear together with such rippling, e.g., if the word in the
//! example just given could also be hyphenated between \.c and \.d; \TeX\
//! avoids this by simply ignoring the additional hyphens in such weird cases.}
//!
//! Still further complications arise in the presence of ligatures that do not
//! delete the original characters. When punctuation precedes the word being
//! hyphenated, \TeX's method is not perfect under all possible scenarios,
//! because punctuation marks and letters can propagate information back and forth.
//! For example, suppose the original pre-hyphenation pair
//! \.{*a} changes to \.{*y} via a \.{\?=:} ligature, which changes to \.{xy}
//! via a \.{=:\?} ligature; if $p_{a-1}=\.x$ and $p_a=\.y$, the reconstitution
//! procedure isn't smart enough to obtain \.{xy} again. In such cases the
//! font designer should include a ligature that goes from \.{xa} to \.{xy}.
//!
