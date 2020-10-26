//! @ Here is a list of cases where the user has probably gotten into or out of math
//! mode by mistake. \TeX\ will insert a dollar sign and rescan the current token.
//!
//! @d non_math(#)==vmode+#,hmode+#
//!
//! @<Math-only cases in non-math modes...@>=
//! non_math(sup_mark), non_math(sub_mark), non_math(math_char_num),
//! non_math(math_given), non_math(math_comp), non_math(delim_num),
//! non_math(left_right), non_math(above), non_math(radical),
//! non_math(math_style), non_math(math_choice), non_math(vcenter),
//! non_math(non_script), non_math(mkern), non_math(limit_switch),
//! non_math(mskip), non_math(math_accent),
//! mmode+endv, mmode+par_end, mmode+stop, mmode+vskip, mmode+un_vbox,
//! mmode+valign, mmode+hrule
//!
//! @ @<Declare action...@>=
//! procedure insert_dollar_sign;
//! begin back_input; cur_tok:=math_shift_token+"$";
//! print_err("Missing $ inserted");
//! @.Missing \$ inserted@>
//! help2("I've inserted a begin-math/end-math symbol since I think")@/
//! ("you left one out. Proceed, with fingers crossed."); ins_error;
//! end;
//!
//! @ When erroneous situations arise, \TeX\ usually issues an error message
//! specific to the particular error. For example, `\.{\\noalign}' should
//! not appear in any mode, since it is recognized by the |align_peek| routine
//! in all of its legitimate appearances; a special error message is given
//! when `\.{\\noalign}' occurs elsewhere. But sometimes the most appropriate
//! error message is simply that the user is not allowed to do what he or she
//! has attempted. For example, `\.{\\moveleft}' is allowed only in vertical mode,
//! and `\.{\\lower}' only in non-vertical modes.  Such cases are enumerated
//! here and in the other sections referred to under `See also \dots.'
//!
//! @<Forbidden cases...@>=
//! vmode+vmove,hmode+hmove,mmode+hmove,any_mode(last_item),
//!
//! @ The `|you_cant|' procedure prints a line saying that the current command
//! is illegal in the current mode; it identifies these things symbolically.
//!
//! @<Declare action...@>=
//! procedure you_cant;
//! begin print_err("You can't use `");
//! @.You can't use x in y mode@>
//! print_cmd_chr(cur_cmd,cur_chr);
//! print("' in "); print_mode(mode);
//! end;
//!
//! @ @<Declare act...@>=
//! procedure report_illegal_case;
//! begin you_cant;
//! help4("Sorry, but I'm not programmed to handle this case;")@/
//! ("I'll just pretend that you didn't ask for it.")@/
//! ("If you're in the wrong mode, you might be able to")@/
//! ("return to the right one by typing `I}' or `I$' or `I\par'.");@/
//! error;
//! end;
//!
//! @ Some operations are allowed only in privileged modes, i.e., in cases
//! that |mode>0|. The |privileged| function is used to detect violations
//! of this rule; it issues an error message and returns |false| if the
//! current |mode| is negative.
//!
//! @<Declare act...@>=
//! function privileged:boolean;
//! begin if mode>0 then privileged:=true
//! else  begin report_illegal_case; privileged:=false;
//!   end;
//! end;
//!
