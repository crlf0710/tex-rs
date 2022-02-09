//! ` `
// @<Compute the amount of skew@>=
pub(crate) macro Compute_the_amount_of_skew($globals:expr, $q:expr, $s:expr) {{
    // s:=0;
    $s = scaled::zero();
    crate::region_forward_label! {
        |'done1|
        {
            // if math_type(nucleus(q))=math_char then
            if math_type!($globals, nucleus!($q)) == math_type_kind::math_char as _ {
                // begin fetch(nucleus(q));
                let fetched = fetch($globals, nucleus!($q));
                // if char_tag(cur_i)=lig_tag then
                if fetched.cur_i.char_tag() == char_tag::lig_tag {
                    /// address of lig/kern instruction
                    let mut a;
                    // begin a:=lig_kern_start(cur_f)(cur_i);
                    a = lig_kern_start!($globals, fetched.cur_f, fetched.cur_i);
                    // cur_i:=font_info[a].qqqq;
                    let mut cur_i = $globals.font_info[a as font_index_repr][MEMORY_WORD_LIG_KERN_CMD];
                    // if skip_byte(cur_i)>stop_flag then
                    if cur_i.skip_byte() > stop_flag {
                        // begin a:=lig_kern_restart(cur_f)(cur_i);
                        a = lig_kern_restart!($globals, fetched.cur_f, cur_i);
                        // cur_i:=font_info[a].qqqq;
                        cur_i = $globals.font_info[a as font_index_repr][MEMORY_WORD_LIG_KERN_CMD];
                        // end;
                    }
                    // loop@+ begin if qo(next_char(cur_i))=skew_char[cur_f] then
                    loop {
                        if qo!(cur_i.next_char()) as integer == $globals.skew_char[fetched.cur_f] {
                            // begin if op_byte(cur_i)>=kern_flag then
                            if cur_i.op_byte() >= kern_flag {
                                // if skip_byte(cur_i)<=stop_flag then s:=char_kern(cur_f)(cur_i);
                                if cur_i.skip_byte() <= stop_flag {
                                    $s = char_kern!($globals, fetched.cur_f, cur_i);
                                }
                            }
                            // goto done1;
                            crate::goto_forward_label!('done1);
                            // end;
                        }
                        // if skip_byte(cur_i)>=stop_flag then goto done1;
                        if cur_i.skip_byte() >= stop_flag {
                            crate::goto_forward_label!('done1);
                        }
                        // a:=a+qo(skip_byte(cur_i))+1;
                        a = a + qo!(cur_i.skip_byte()) as integer + 1;
                        // cur_i:=font_info[a].qqqq;
                        cur_i = $globals.font_info[a as font_index_repr][MEMORY_WORD_LIG_KERN_CMD];
                        // end;
                    }
                    // end;
                }
                // end;
            }
        }
        // done1:
        'done1 <-
    }
    use crate::pascal::integer;
    use crate::section_0101::scaled;
    use crate::section_0112::qo;
    use crate::section_0544::char_tag;
    use crate::section_0545::kern_flag;
    use crate::section_0545::stop_flag;
    use crate::section_0545::MEMORY_WORD_LIG_KERN_CMD;
    use crate::section_0548::font_index;
    use crate::section_0548::font_index_repr;
    use crate::section_0557::char_kern;
    use crate::section_0557::lig_kern_restart;
    use crate::section_0557::lig_kern_start;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::nucleus;
    use crate::section_0722::fetch;
}}
