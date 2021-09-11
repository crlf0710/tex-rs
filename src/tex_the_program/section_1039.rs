//! @ Even though comparatively few characters have a lig/kern program, several
//! of the instructions here count as part of \TeX's inner loop, since a
//! @^inner loop@>
//! potentially long sequential search must be performed. For example, tests with
//! Computer Modern Roman showed that about 40 per cent of all characters
//! actually encountered in practice had a lig/kern program, and that about four
//! lig/kern commands were investigated for every such character.
//!
//! At the beginning of this code we have |main_i=char_info(main_f)(cur_l)|.
//
// @<If there's a ligature/kern command...@>=
#[allow(unused_macros)]
pub(crate) macro If_there_s_a_ligature_kern_command_relevant_to_cur_l_and_cur_r__adjust_the_text_appropriately__exit_to_main_loop_wrapup
    ($globals:expr, $cur_part_idx:expr, $lbl_main_loop_cycle:lifetime, $main_loop_status:expr) {{
        crate::trace_span!("If there's a ligature/kern command...");
        crate::region_multipart_autoincr! {
            ('main_lig_loop_inner, $cur_part_idx) {
                0 =>
                {
                    crate::trace_span!("main_lig_loop");
                    // if char_tag(main_i)<>lig_tag then goto main_loop_wrapup;
                    if $globals.main_i.char_tag() != char_tag::lig_tag {
                        crate::goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_status_kind::main_loop_wrapup);
                    }
                    // if cur_r=non_char then goto main_loop_wrapup;
                    if $globals.cur_r == non_char {
                        crate::goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_status_kind::main_loop_wrapup);
                    }
                    // main_k:=lig_kern_start(main_f)(main_i); main_j:=font_info[main_k].qqqq;
                    $globals.main_k = font_index::new(lig_kern_start!($globals, $globals.main_f, $globals.main_i) as _);
                    $globals.main_j = $globals.font_info[$globals.main_k][MEMORY_WORD_LIG_KERN_CMD];
                    // if skip_byte(main_j)<=stop_flag then goto main_lig_loop+2;
                    if $globals.main_j.skip_byte() <= stop_flag {
                        crate::goto_part_label!('main_lig_loop_inner, $cur_part_idx, 2);
                    }
                    // main_k:=lig_kern_restart(main_f)(main_j);
                    $globals.main_k = font_index::new(lig_kern_restart!($globals, $globals.main_f, $globals.main_j) as _);
                },
                1 =>
                {
                    crate::trace_span!("main_lig_loop + 1");
                    // main_lig_loop+1:main_j:=font_info[main_k].qqqq;
                    $globals.main_j = $globals.font_info[$globals.main_k][MEMORY_WORD_LIG_KERN_CMD];
                },
                2 =>
                {
                    crate::trace_span!("main_lig_loop + 2");
                    // main_lig_loop+2:if next_char(main_j)=cur_r then
                    if $globals.main_j.next_char() as integer == $globals.cur_r as integer {
                        // if skip_byte(main_j)<=stop_flag then
                        if $globals.main_j.skip_byte() <= stop_flag {
                            // @<Do ligature or kern command, returning to |main_lig_loop|
                            // or |main_loop_wrapup| or |main_loop_move|@>;
                            crate::section_1040::Do_ligature_or_kern_command__returning_to_main_lig_loop_or_main_loop_wrapup_or_main_loop_move!
                                ($globals, $lbl_main_loop_cycle, $main_loop_status);
                        }
                    }
                    // if skip_byte(main_j)=qi(0) then incr(main_k)
                    if $globals.main_j.skip_byte() == qi!(0) {
                        incr!($globals.main_k);
                    }
                    // else begin if skip_byte(main_j)>=stop_flag then goto main_loop_wrapup;
                    else {
                        if $globals.main_j.skip_byte() >= stop_flag {
                            crate::goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_status_kind::main_loop_wrapup);
                        }
                        // main_k:=main_k+qo(skip_byte(main_j))+1;
                        $globals.main_k += (qo!($globals.main_j.skip_byte()) as integer + 1) as _;
                        // end;
                    }
                    // goto main_lig_loop+1
                    crate::goto_part_label!('main_lig_loop_inner, $cur_part_idx, 1);
                },
            }
        }
        use crate::pascal::integer;
        use crate::section_0016::incr;
        use crate::section_0112::qi;
        use crate::section_0112::qo;
        use crate::section_0544::char_tag;
        use crate::section_0545::MEMORY_WORD_LIG_KERN_CMD;
        use crate::section_0545::stop_flag;
        use crate::section_0548::font_index;
        use crate::section_0549::non_char;
        use crate::section_0557::lig_kern_start;
        use crate::section_0557::lig_kern_restart;
        use crate::section_1034::main_loop_status_kind;
    }}
