//! @ The |error| routine calls on |give_err_help| if help is requested from
//! the |err_help| parameter.
//!
//! @p procedure give_err_help;
//! begin token_show(err_help);
//! end;
//!
//! @ The \.{\\uppercase} and \.{\\lowercase} commands are implemented by
//! building a token list and then changing the cases of the letters in it.
//!
//! @<Cases of |main_control| that don't...@>=
//! any_mode(case_shift):shift_case;
//!
//! @ @<Put each...@>=
//! primitive("lowercase",case_shift,lc_code_base);
//! @!@:lowercase_}{\.{\\lowercase} primitive@>
//! primitive("uppercase",case_shift,uc_code_base);
//! @!@:uppercase_}{\.{\\uppercase} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! case_shift:if chr_code=lc_code_base then print_esc("lowercase")
//!   else print_esc("uppercase");
//!
//! @ @<Declare act...@>=
//! procedure shift_case;
//! var b:pointer; {|lc_code_base| or |uc_code_base|}
//! @!p:pointer; {runs through the token list}
//! @!t:halfword; {token}
//! @!c:eight_bits; {character code}
//! begin b:=cur_chr; p:=scan_toks(false,false); p:=link(def_ref);
//! while p<>null do
//!   begin @<Change the case of the token in |p|, if a change is appropriate@>;
//!   p:=link(p);
//!   end;
//! back_list(link(def_ref)); free_avail(def_ref); {omit reference count}
//! end;
//!
//! @ When the case of a |chr_code| changes, we don't change the |cmd|.
//! We also change active characters, using the fact that
//! |cs_token_flag+active_base| is a multiple of~256.
//! @^data structure assumptions@>
//!
//! @<Change the case of the token in |p|, if a change is appropriate@>=
//! t:=info(p);
//! if t<cs_token_flag+single_base then
//!   begin c:=t mod 256;
//!   if equiv(b+c)<>0 then info(p):=t-c+equiv(b+c);
//!   end
//!
