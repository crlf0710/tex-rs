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
macro_rules! If_there_s_a_ligature_kern_command_relevant_to_cur_l_and_cur_r__adjust_the_text_appropriately__exit_to_main_loop_wrapup {
    ($globals:expr, $cur_part_idx:expr) => {{
        region_multipart_autoincr! {
            ('main_lig_loop_inner, $cur_part_idx) {
                0 =>
                {
                    // if char_tag(main_i)<>lig_tag then goto main_loop_wrapup;
                    // if cur_r=non_char then goto main_loop_wrapup;
                    // main_k:=lig_kern_start(main_f)(main_i); main_j:=font_info[main_k].qqqq;
                    // if skip_byte(main_j)<=stop_flag then goto main_lig_loop+2;
                    // main_k:=lig_kern_restart(main_f)(main_j);
                },
                1 =>
                {
                    // main_lig_loop+1:main_j:=font_info[main_k].qqqq;
                },
                2 =>
                {
                    // main_lig_loop+2:if next_char(main_j)=cur_r then
                    //  if skip_byte(main_j)<=stop_flag then
                    //   @<Do ligature or kern command, returning to |main_lig_loop|
                    //   or |main_loop_wrapup| or |main_loop_move|@>;
                    // if skip_byte(main_j)=qi(0) then incr(main_k)
                    // else begin if skip_byte(main_j)>=stop_flag then goto main_loop_wrapup;
                    //   main_k:=main_k+qo(skip_byte(main_j))+1;
                    //   end;
                    // goto main_lig_loop+1
                    goto_part_label!('main_lig_loop_inner, $cur_part_idx, 1);
                },
            }
        }
    }}
}