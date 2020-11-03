//! @* \[24] Getting the next token.
//! The heart of \TeX's input mechanism is the |get_next| procedure, which
//! we shall develop in the next few sections of the program. Perhaps we
//! shouldn't actually call it the ``heart,'' however, because it really acts
//! as \TeX's eyes and mouth, reading the source files and gobbling them up.
//! And it also helps \TeX\ to regurgitate stored token lists that are to be
//! processed again.
//! @^eyes and mouth@>
//!
//! The main duty of |get_next| is to input one token and to set |cur_cmd|
//! and |cur_chr| to that token's command code and modifier. Furthermore, if
//! the input token is a control sequence, the |eqtb| location of that control
//! sequence is stored in |cur_cs|; otherwise |cur_cs| is set to zero.
//!
//! Underlying this simple description is a certain amount of complexity
//! because of all the cases that need to be handled.
//! However, the inner loop of |get_next| is reasonably short and fast.
//!
//! When |get_next| is asked to get the next token of a \.{\\read} line,
//! it sets |cur_cmd=cur_chr=cur_cs=0| in the case that no more tokens
//! appear on that line. (There might not be any tokens at all, if the
//! |end_line_char| has |ignore| as its catcode.)
//!
