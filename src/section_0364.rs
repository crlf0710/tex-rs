//! @ Since |get_next| is used so frequently in \TeX, it is convenient
//! to define three related procedures that do a little more:
//!
//! \yskip\hang|get_token| not only sets |cur_cmd| and |cur_chr|, it
//! also sets |cur_tok|, a packed halfword version of the current token.
//!
//! \yskip\hang|get_x_token|, meaning ``get an expanded token,'' is like
//! |get_token|, but if the current token turns out to be a user-defined
//! control sequence (i.e., a macro call), or a conditional,
//! or something like \.{\\topmark} or \.{\\expandafter} or \.{\\csname},
//! it is eliminated from the input by beginning the expansion of the macro
//! or the evaluation of the conditional.
//!
//! \yskip\hang|x_token| is like |get_x_token| except that it assumes that
//! |get_next| has already been called.
//!
//! \yskip\noindent
//! In fact, these three procedures account for almost every use of |get_next|.
//!
