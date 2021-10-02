//! ` `
//! We may want to look at the lig/kern program twice, once for a hyphen
//! and once for a normal letter. (The hyphen might appear after the letter
//! in the program, so we'd better not try to look for both at once.)
//
// @<If there's a ligature or kern at the cursor position, update...@>=
pub(crate) macro If_there_s_a_ligature_or_kern_at_the_cursor_position__update_the_data_structures__possibly_advancing_j__continue_until_the_cursor_moves($globals:expr, $j:expr, $n:expr, $bchar:expr, $hchar:expr, $cur_rh:expr, $test_char:expr, $w:expr, $lbl_continue:lifetime) {{
    crate::region_forward_label! {
        |'done|
        {
            /// position of current lig/kern instruction
            let mut k;

            /// lig/kern instruction
            let mut q_lig_kern_cmd;

            // if cur_l=non_char then
            if $globals.cur_l == non_char {
                // begin k:=bchar_label[hf];
                k = $globals.bchar_label[$globals.hf];
                // if k=non_address then goto done@+else q:=font_info[k].qqqq;
                if k == non_address {
                    crate::goto_forward_label!('done);
                } else {
                    q_lig_kern_cmd = $globals.font_info[k][MEMORY_WORD_LIG_KERN_CMD];
                }
                // end
            }
            // else begin q:=char_info(hf)(cur_l);
            else {
                /// character information
                let q_charinfo;

                q_charinfo = char_info!($globals, $globals.hf, $globals.cur_l);
                // if char_tag(q)<>lig_tag then goto done;
                if q_charinfo.char_tag() != char_tag::lig_tag {
                    crate::goto_forward_label!('done);
                }
                // k:=lig_kern_start(hf)(q); q:=font_info[k].qqqq;
                k = font_index::new(lig_kern_start!($globals, $globals.hf, q_charinfo) as _);
                q_lig_kern_cmd = $globals.font_info[k][MEMORY_WORD_LIG_KERN_CMD];
                // if skip_byte(q)>stop_flag then
                if q_lig_kern_cmd.skip_byte() > stop_flag {
                    // begin k:=lig_kern_restart(hf)(q); q:=font_info[k].qqqq;
                    k = font_index::new(lig_kern_restart!($globals, $globals.hf, q_lig_kern_cmd) as _);
                    q_lig_kern_cmd = $globals.font_info[k][MEMORY_WORD_LIG_KERN_CMD];
                    // end;
                }
                // end; {now |k| is the starting address of the lig/kern program}
            }
            /// now `k` is the starting address of the lig/kern program
            const _ : () = ();
            // if cur_rh<non_char then test_char:=cur_rh@+else test_char:=cur_r;
            if $cur_rh < non_char {
                $test_char = $cur_rh;
            } else {
                $test_char = $globals.cur_r;
            }
            // loop@+begin if next_char(q)=test_char then if skip_byte(q)<=stop_flag then
            loop {
                if q_lig_kern_cmd.next_char() as ASCII_code_or_non_char == $test_char &&
                    q_lig_kern_cmd.skip_byte() <= stop_flag {
                    // if cur_rh<non_char then
                    if $cur_rh < non_char {
                        // begin hyphen_passed:=j; hchar:=non_char; cur_rh:=non_char;
                        // goto continue;
                        // end
                        todo!("cur_rh < non_char");
                    }
                    // else begin if hchar<non_char then if odd(hyf[j]) then
                    else {
                        if $hchar < non_char && $globals.hyf[$j.get() as usize].is_odd() {
                            // begin hyphen_passed:=j; hchar:=non_char;
                            $globals.hyphen_passed = $j;
                            $hchar = non_char;
                            // end;
                        }
                        //  if op_byte(q)<kern_flag then
                        if q_lig_kern_cmd.op_byte() < kern_flag {
                            // @<Carry out a ligature replacement, updating the cursor structure
                            //   and possibly advancing~|j|; |goto continue| if the cursor doesn't
                            //   advance, otherwise |goto done|@>;
                            crate::section_0911::Carry_out_a_ligature_replacement__updating_the_cursor_structure_and_possibly_advancing_j__goto_continue_if_the_cursor_doesn_t_advance__otherwise_goto_done!
                                ($globals, $j, $n, q_lig_kern_cmd, $bchar, $lbl_continue, 'done);
                        }
                        // w:=char_kern(hf)(q); goto done; {this kern will be inserted below}
                        $w = char_kern!($globals, $globals.hf, q_lig_kern_cmd);
                        crate::goto_forward_label!('done);
                        /// this kern will be inserted below
                        const _ : () = ();
                        // end;
                    }
                }
                // if skip_byte(q)>=stop_flag then
                if q_lig_kern_cmd.skip_byte() >= stop_flag {
                    // if cur_rh=non_char then goto done
                    if $cur_rh == non_char {
                        crate::goto_forward_label!('done);
                    }
                    // else begin cur_rh:=non_char; goto continue;
                    else {
                        $cur_rh = non_char;
                        crate::goto_backward_label!($lbl_continue);
                        // end;
                    }
                }
                // k:=k+qo(skip_byte(q))+1; q:=font_info[k].qqqq;
                k = font_index::new(k.get() + q_lig_kern_cmd.skip_byte() as u16 + 1);
                q_lig_kern_cmd = $globals.font_info[k][MEMORY_WORD_LIG_KERN_CMD];
                // end;
            }
        }
        // done:
        'done <-
    };
    use crate::pascal::IsOddOrEven;
    use crate::section_0544::char_tag;
    use crate::section_0545::stop_flag;
    use crate::section_0545::MEMORY_WORD_LIG_KERN_CMD;
    use crate::section_0545::kern_flag;
    use crate::section_0548::font_index;
    use crate::section_0549::non_address;
    use crate::section_0549::non_char;
    use crate::section_0554::char_info;
    use crate::section_0557::char_kern;
    use crate::section_0557::lig_kern_restart;
    use crate::section_0557::lig_kern_start;
    use crate::section_0907::ASCII_code_or_non_char;
}}
