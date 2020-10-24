//! @ No new control sequences will be defined except during a call of
//! |get_token|, or when \.{\\csname} compresses a token list, because
//! |no_new_control_sequence| is always |true| at other times.
//!
//! @p procedure get_token; {sets |cur_cmd|, |cur_chr|, |cur_tok|}
//! begin no_new_control_sequence:=false; get_next; no_new_control_sequence:=true;
//! @^inner loop@>
//! if cur_cs=0 then cur_tok:=(cur_cmd*@'400)+cur_chr
//! else cur_tok:=cs_token_flag+cur_cs;
//! end;
//!
