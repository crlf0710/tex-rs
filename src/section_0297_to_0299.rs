//! @* \[21] Introduction to the syntactic routines.
//! Let's pause a moment now and try to look at the Big Picture.
//! The \TeX\ program consists of three main parts: syntactic routines,
//! semantic routines, and output routines. The chief purpose of the
//! syntactic routines is to deliver the user's input to the semantic routines,
//! one token at a time. The semantic routines act as an interpreter
//! responding to these tokens, which may be regarded as commands. And the
//! output routines are periodically called on to convert box-and-glue
//! lists into a compact set of instructions that will be sent
//! to a typesetter. We have discussed the basic data structures and utility
//! routines of \TeX, so we are good and ready to plunge into the real activity by
//! considering the syntactic routines.
//!
//! Our current goal is to come to grips with the |get_next| procedure,
//! which is the keystone of \TeX's input mechanism. Each call of |get_next|
//! sets the value of three variables |cur_cmd|, |cur_chr|, and |cur_cs|,
//! representing the next input token.
//! $$\vbox{\halign{#\hfil\cr
//!   \hbox{|cur_cmd| denotes a command code from the long list of codes
//!    given above;}\cr
//!   \hbox{|cur_chr| denotes a character code or other modifier of the command
//!    code;}\cr
//!   \hbox{|cur_cs| is the |eqtb| location of the current control sequence,}\cr
//!   \hbox{\qquad if the current token was a control sequence,
//!    otherwise it's zero.}\cr}}$$
//! Underlying this external behavior of |get_next| is all the machinery
//! necessary to convert from character files to tokens. At a given time we
//! may be only partially finished with the reading of several files (for
//! which \.{\\input} was specified), and partially finished with the expansion
//! of some user-defined macros and/or some macro parameters, and partially
//! finished with the generation of some text in a template for \.{\\halign},
//! and so on. When reading a character file, special characters must be
//! classified as math delimiters, etc.; comments and extra blank spaces must
//! be removed, paragraphs must be recognized, and control sequences must be
//! found in the hash table. Furthermore there are occasions in which the
//! scanning routines have looked ahead for a word like `\.{plus}' but only
//! part of that word was found, hence a few characters must be put back
//! into the input and scanned again.
//!
//! To handle these situations, which might all be present simultaneously,
//! \TeX\ uses various stacks that hold information about the incomplete
//! activities, and there is a finite state control for each level of the
//! input mechanism. These stacks record the current state of an implicitly
//! recursive process, but the |get_next| procedure is not recursive.
//! Therefore it will not be difficult to translate these algorithms into
//! low-level languages that do not support recursion.
//!
//! @<Glob...@>=
//! @!cur_cmd: eight_bits; {current command set by |get_next|}
//! @!cur_chr: halfword; {operand of current command}
//! @!cur_cs: pointer; {control sequence found here, zero if none found}
//! @!cur_tok: halfword; {packed representative of |cur_cmd| and |cur_chr|}
//!
//! @ The |print_cmd_chr| routine prints a symbolic interpretation of a
//! command code and its modifier. This is used in certain `\.{You can\'t}'
//! error messages, and in the implementation of diagnostic routines like
//! \.{\\show}.
//!
//! The body of |print_cmd_chr| is a rather tedious listing of print
//! commands, and most of it is essentially an inverse to the |primitive|
//! routine that enters a \TeX\ primitive into |eqtb|. Therefore much of
//! this procedure appears elsewhere in the program,
//! together with the corresponding |primitive| calls.
//!
//! @d chr_cmd(#)==begin print(#); print_ASCII(chr_code);
//!   end
//!
//! @<Declare the procedure called |print_cmd_chr|@>=
//! procedure print_cmd_chr(@!cmd:quarterword;@!chr_code:halfword);
//! begin case cmd of
//! left_brace: chr_cmd("begin-group character ");
//! right_brace: chr_cmd("end-group character ");
//! math_shift: chr_cmd("math shift character ");
//! mac_param: chr_cmd("macro parameter character ");
//! sup_mark: chr_cmd("superscript character ");
//! sub_mark: chr_cmd("subscript character ");
//! endv: print("end of alignment template");
//! spacer: chr_cmd("blank space ");
//! letter: chr_cmd("the letter ");
//! other_char: chr_cmd("the character ");
//! @t\4@>@<Cases of |print_cmd_chr| for symbolic printing of primitives@>@/
//! othercases print("[unknown command code!]")
//! endcases;
//! end;
//!
//! @ Here is a procedure that displays the current command.
//!
//! @p procedure show_cur_cmd_chr;
//! begin begin_diagnostic; print_nl("{");
//! if mode<>shown_mode then
//!   begin print_mode(mode); print(": "); shown_mode:=mode;
//!   end;
//! print_cmd_chr(cur_cmd,cur_chr); print_char("}");
//! end_diagnostic(false);
//! end;
