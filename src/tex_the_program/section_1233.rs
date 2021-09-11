//! ` `
// @<Let |n| be the largest...@>=
pub(crate) macro Let_n_be_the_largest_legal_code_value__based_on_cur_chr($globals:expr, $n:expr, $cur_chr:expr) {
    // if cur_chr=cat_code_base then n:=max_char_code
    if $cur_chr.get() == cat_code_base {
        $n = max_char_code as _;

        use crate::section_0207::max_char_code;
    }
    // else if cur_chr=math_code_base then n:=@'100000
    else if $cur_chr.get() == math_code_base {
        $n = 0o100000;
    }
    // else if cur_chr=sf_code_base then n:=@'77777
    else if $cur_chr.get() == sf_code_base {
        $n = 0o77777;
    }
    // else if cur_chr=del_code_base then n:=@'77777777
    else if $cur_chr.get() == del_code_base {
        $n = 0o77777777;
    }
    // else n:=255
    else {
        $n = 255;
    }
    use crate::section_0230::cat_code_base;
    use crate::section_0230::math_code_base;
    use crate::section_0230::sf_code_base;
    use crate::section_0236::del_code_base;
}
