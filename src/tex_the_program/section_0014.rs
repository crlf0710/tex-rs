//! @ Later on we will say `\ignorespaces|if mem_max>=max_halfword then bad:=14|',
//! or something similar. (We can't do that until |max_halfword| has been defined.)
//
// @<Check the ``constant'' values for consistency@>=
// bad:=0;
// if (half_error_line<30)or(half_error_line>error_line-15) then bad:=1;
// if max_print_line<60 then bad:=2;
// if dvi_buf_size mod 8<>0 then bad:=3;
// if mem_bot+1100>mem_top then bad:=4;
// if hash_prime>hash_size then bad:=5;
// if max_in_open>=128 then bad:=6;
// if mem_top<256+11 then bad:=7; {we will want |null_list>255|}
//
