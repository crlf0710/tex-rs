//! @ @<Complain about an undefined family...@>=
//! begin print_err(""); print_size(cur_size); print_char(" ");
//! print_int(fam(a)); print(" is undefined (character ");
//! print_ASCII(qo(cur_c)); print_char(")");
//! help4("Somewhere in the math formula just ended, you used the")@/
//! ("stated character from an undefined font family. For example,")@/
//! ("plain TeX doesn't allow \it or \sl in subscripts. Proceed,")@/
//! ("and I'll try to forget that I needed that character.");
//! error; cur_i:=null_character; math_type(a):=empty;
//! end
//!
