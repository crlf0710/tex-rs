//! @ A token list is a singly linked list of one-word nodes in |mem|, where
//! each word contains a token and a link. Macro definitions, output-routine
//! definitions, marks, \.{\\write} texts, and a few other things
//! are remembered by \TeX\ in the form
//! of token lists, usually preceded by a node with a reference count in its
//! |token_ref_count| field. The token stored in location |p| is called
//! |info(p)|.
//!
//! Three special commands appear in the token lists of macro definitions.
//! When |m=match|, it means that \TeX\ should scan a parameter
//! for the current macro; when |m=end_match|, it means that parameter
//! matching should end and \TeX\ should start reading the macro text; and
//! when |m=out_param|, it means that \TeX\ should insert parameter
//! number |c| into the text at this point.
//!
//! The enclosing \.{\char'173} and \.{\char'175} characters of a macro
//! definition are omitted, but the final right brace of an output routine
//! is included at the end of its token list.
//!
//! Here is an example macro definition that illustrates these conventions.
//! After \TeX\ processes the text
//! $$\.{\\def\\mac a\#1\#2 \\b \{\#1\\-a \#\#1\#2 \#2\}}$$
//! the definition of \.{\\mac} is represented as a token list containing
//! $$\def\,{\hskip2pt}
//! \vbox{\halign{\hfil#\hfil\cr
//! (reference count), |letter|\,\.a, |match|\,\#, |match|\,\#, |spacer|\,\.\ ,
//! \.{\\b}, |end_match|,\cr
//! |out_param|\,1, \.{\\-}, |letter|\,\.a, |spacer|\,\.\ , |mac_param|\,\#,
//! |other_char|\,\.1,\cr
//! |out_param|\,2, |spacer|\,\.\ , |out_param|\,2.\cr}}$$
//! The procedure |scan_toks| builds such token lists, and |macro_call|
//! does the parameter matching.
//! @^reference counts@>
//!
//! Examples such as
//! $$\.{\\def\\m\{\\def\\m\{a\}\ b\}}$$
//! explain why reference counts would be needed even if \TeX\ had no \.{\\let}
//! operation: When the token list for \.{\\m} is being read, the redefinition of
//! \.{\\m} changes the |eqtb| entry before the token list has been fully
//! consumed, so we dare not simply destroy a token list when its
//! control sequence is being redefined.
//!
//! If the parameter-matching part of a definition ends with `\.{\#\{}',
//! the corresponding token list will have `\.\{' just before the `|end_match|'
//! and also at the very end. The first `\.\{' is used to delimit the parameter; the
//! second one keeps the first from disappearing.
//!
