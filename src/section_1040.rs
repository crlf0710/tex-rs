//! @ When a ligature or kern instruction matches a character, we know from
//! |read_font_info| that the character exists in the font, even though we
//! haven't verified its existence in the normal way.
//!
//! This section could be made into a subroutine, if the code inside
//! |main_control| needs to be shortened.
//!
//! \chardef\?='174 % vertical line to indicate character retention
//
// @<Do ligature or kern command...@>=
macro_rules! Do_ligature_or_kern_command__returning_to_main_lig_loop_or_main_loop_wrapup_or_main_loop_move {
    ($globals:expr, $lbl_main_loop_cycle:lifetime, $main_loop_status:expr) => {{
        // begin if op_byte(main_j)>=kern_flag then
        if $globals.main_j.op_byte() >= kern_flag {
            // begin wrapup(rt_hit);
            wrapup!($globals, $globals.rt_hit);
            // tail_append(new_kern(char_kern(main_f)(main_j))); goto main_loop_move;
            tail_append!(
                $globals,
                new_kern(
                    $globals,
                    char_kern!($globals, $globals.main_f, $globals.main_j)
                )?
            );
            goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_move(0));
            // end;
        }
        // if cur_l=non_char then lft_hit:=true
        if $globals.cur_l == non_char {
            $globals.lft_hit = true;
        }
        // else if lig_stack=null then rt_hit:=true;
        else if $globals.lig_stack == null {
            $globals.rt_hit = true;
        }
        // check_interrupt; {allow a way out in case there's an infinite ligature loop}
        /// allow a way out in case there's an infinite ligature loop
        check_interrupt!($globals);
        // case op_byte(main_j) of
        match qo!($globals.main_j.op_byte()) {
            // qi(1),qi(5):begin cur_l:=rem_byte(main_j); {\.{=:\?}, \.{=:\?>}}
            1 | 5 => {
                todo!("op1&5");
                // main_i:=char_info(main_f)(cur_l); ligature_present:=true;
                // end;
            }
            // qi(2),qi(6):begin cur_r:=rem_byte(main_j); {\.{\?=:}, \.{\?=:>}}
            2 | 6 => {
                /// `|=:, |=:>`
                const _ : () = ();
                $globals.cur_r = $globals.main_j.rem_byte() as _;
                // if lig_stack=null then {right boundary character is being consumed}
                if $globals.lig_stack == null {
                    /// right boundary character is being consumed
                    const _ : () = ();
                    // begin lig_stack:=new_lig_item(cur_r); bchar:=non_char;
                    $globals.lig_stack = new_lig_item($globals, ASCII_code::from($globals.cur_r as integer))?;
                    $globals.bchar = non_char;
                    // end
                }
                // else if is_char_node(lig_stack) then {|link(lig_stack)=null|}
                else if is_char_node!($globals, $globals.lig_stack) {
                    /// `link(lig_stack)=null`
                    const _ : () = ();
                    // begin main_p:=lig_stack; lig_stack:=new_lig_item(cur_r);
                    $globals.main_p = $globals.lig_stack;
                    $globals.lig_stack = new_lig_item($globals, ASCII_code::from($globals.cur_r as integer))?;
                    // lig_ptr(lig_stack):=main_p;
                    lig_ptr!($globals, $globals.lig_stack) = $globals.main_p;
                    // end
                }
                // else character(lig_stack):=cur_r;
                else {
                    let f = null_font;
                    let c = ASCII_code::from($globals.cur_r as integer);
                    assign_font_and_character!($globals, $globals.lig_stack, f, c);
                }
                // end;
            }
            // qi(3):begin cur_r:=rem_byte(main_j); {\.{\?=:\?}}
            3 => {
                /// `|=:|`
                const _ : () = ();
                $globals.cur_r = $globals.main_j.rem_byte() as _;
                // main_p:=lig_stack; lig_stack:=new_lig_item(cur_r);
                $globals.main_p = $globals.lig_stack;
                $globals.lig_stack = new_lig_item($globals, ASCII_code::from($globals.cur_r as integer))?;
                // link(lig_stack):=main_p;
                link!($globals, $globals.lig_stack) = $globals.main_p;
                // end;
            }
            // qi(7),qi(11):begin wrapup(false); {\.{\?=:\?>}, \.{\?=:\?>>}}
            7 | 11 => {
                todo!("op7&11");
                // cur_q:=tail; cur_l:=rem_byte(main_j);
                // main_i:=char_info(main_f)(cur_l); ligature_present:=true;
                // end;
            }
            // othercases begin cur_l:=rem_byte(main_j); ligature_present:=true; {\.{=:}}
            _ => {
                todo!("othercases");
                // if lig_stack=null then goto main_loop_wrapup
                // else goto main_loop_move+1;
            }
            // end
        }
        // endcases;
        // if op_byte(main_j)>qi(4) then
        //   if op_byte(main_j)<>qi(7) then goto main_loop_wrapup;
        if qo!($globals.main_j.op_byte()) > 4 && qo!($globals.main_j.op_byte()) != 7 {
            goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_loop_wrapup);
        }
        // if cur_l<non_char then goto main_lig_loop;
        if $globals.cur_l < non_char {
            goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_lig_loop(0));
        }
        // main_k:=bchar_label[main_f]; goto main_lig_loop+1;
        $globals.main_k = $globals.bchar_label[$globals.main_f];
        goto_part_label!($lbl_main_loop_cycle, $main_loop_status, main_lig_loop(1));
        // end
        use crate::section_0144::new_lig_item;
        use crate::section_0156::new_kern;
        use crate::section_0232::null_font;
        use crate::section_0545::kern_flag;
    }};
}
