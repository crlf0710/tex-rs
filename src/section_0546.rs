//! @ Extensible characters are specified by an |@!extensible_recipe|, which
//! consists of four bytes called |@!top|, |@!mid|, |@!bot|, and |@!rep| (in this
//! order). These bytes are the character codes of individual pieces used to
//! build up a large symbol.  If |top|, |mid|, or |bot| are zero, they are not
//! present in the built-up result. For example, an extensible vertical line is
//! like an extensible bracket, except that the top and bottom pieces are missing.
//!
//! Let $T$, $M$, $B$, and $R$ denote the respective pieces, or an empty box
//! if the piece isn't present. Then the extensible characters have the form
//! $TR^kMR^kB$ from top to bottom, for some |k>=0|, unless $M$ is absent;
//! in the latter case we can have $TR^kB$ for both even and odd values of~|k|.
//! The width of the extensible character is the width of $R$; and the
//! height-plus-depth is the sum of the individual height-plus-depths of the
//! components used, since the pieces are butted together in a vertical list.
//!
//! @d ext_top(#)==#.b0 {|top| piece in a recipe}
//! @d ext_mid(#)==#.b1 {|mid| piece in a recipe}
//! @d ext_bot(#)==#.b2 {|bot| piece in a recipe}
//! @d ext_rep(#)==#.b3 {|rep| piece in a recipe}
//!
