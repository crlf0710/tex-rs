//! @ All glue parameters and registers are initially `\.{0pt plus0pt minus0pt}'.
//!
//! @<Initialize table entries...@>=
//! equiv(glue_base):=zero_glue; eq_level(glue_base):=level_one;
//! eq_type(glue_base):=glue_ref;
//! for k:=glue_base+1 to local_base-1 do eqtb[k]:=eqtb[glue_base];
//! glue_ref_count(zero_glue):=glue_ref_count(zero_glue)+local_base-glue_base;
//!
