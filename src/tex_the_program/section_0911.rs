//! ` `

// @<Carry out a ligature replacement, updating the cursor structure...@>=
pub(crate) macro Carry_out_a_ligature_replacement__updating_the_cursor_structure_and_possibly_advancing_j__goto_continue_if_the_cursor_doesn_t_advance__otherwise_goto_done($globals:expr, $j:expr, $n:expr, $q:expr, $bchar:expr, $lbl_continue:lifetime, $lbl_done:lifetime) {{
    // begin if cur_l=non_char then lft_hit:=true;
    if $globals.cur_l == non_char {
        $globals.lft_hit = true;
    }
    // if j=n then if lig_stack=null then rt_hit:=true;
    if $j == $n && $globals.lig_stack == null {
        $globals.rt_hit = true;
    }
    // check_interrupt; {allow a way out in case there's an infinite ligature loop}
    /// allow a way out in case there's an infinite ligature loop
    check_interrupt!($globals);
    // case op_byte(q) of
    match qo!($q.op_byte()) {
        // qi(1),qi(5):begin cur_l:=rem_byte(q); {\.{=:\?}, \.{=:\?>}}
        1 | 5 => {
            /// `=:|, =:|>`
            const _: () = ();
            $globals.cur_l = $q.rem_byte() as _;
            todo!("1 or 5");
            // ligature_present:=true;
            // end;
        }
        // qi(2),qi(6):begin cur_r:=rem_byte(q); {\.{\?=:}, \.{\?=:>}}
        2 | 6 => {
            /// `|=:, |=:>`
            const _: () = ();
            $globals.cur_r = $q.rem_byte() as _;
            // if lig_stack>null then character(lig_stack):=cur_r
            if $globals.lig_stack > null {
                character!($globals, $globals.lig_stack) =
                    ASCII_code::from($globals.cur_r as integer);
            }
            // else begin lig_stack:=new_lig_item(cur_r);
            else {
                $globals.lig_stack =
                    new_lig_item($globals, ASCII_code::from($globals.cur_r as integer))?;
                // if j=n then bchar:=non_char
                if $j == $n {
                    $bchar = non_char;
                }
                // else begin p:=get_avail; lig_ptr(lig_stack):=p;
                else {
                    /// temporary register for list manipulation
                    let p: pointer;
                    p = get_avail($globals);
                    lig_ptr!($globals, $globals.lig_stack) = p;
                    // character(p):=qi(hu[j+1]); font(p):=hf;
                    assign_font_and_character!(
                        $globals,
                        p,
                        $globals.hf,
                        ASCII_code::from($globals.hu[$j.get() as usize + 1] as integer)
                    );
                    // end;
                }
                // end;
            }
            // end;
        }
        // qi(3):begin cur_r:=rem_byte(q); {\.{\?=:\?}}
        //   p:=lig_stack; lig_stack:=new_lig_item(cur_r); link(lig_stack):=p;
        //   end;
        3 => {
            todo!("3");
        }
        // qi(7),qi(11):begin wrap_lig(false); {\.{\?=:\?>}, \.{\?=:\?>>}}
        //   cur_q:=t; cur_l:=rem_byte(q); ligature_present:=true;
        //   end;
        7 | 11 => {
            todo!("7 or 11");
        }
        // othercases begin cur_l:=rem_byte(q); ligature_present:=true; {\.{=:}}
        _ => {
            todo!("otherwise");
            // if lig_stack>null then pop_lig_stack
            // else if j=n then goto done
            // else begin append_charnode_to_t(cur_r); incr(j); set_cur_r;
            //   end;
            // end
        } // endcases;
    }
    // if op_byte(q)>qi(4) then if op_byte(q)<>qi(7) then goto done;
    if qo!($q.op_byte()) > 4 && qo!($q.op_byte()) != 7 {
        crate::goto_forward_label!($lbl_done);
    }
    // goto continue;
    crate::goto_backward_label!($lbl_continue);
    // end
    use crate::pascal::integer;
    use crate::section_0018::ASCII_code;
    use crate::section_0096::check_interrupt;
    use crate::section_0112::qo;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0120::get_avail;
    use crate::section_0134::assign_font_and_character;
    use crate::section_0134::character;
    use crate::section_0143::lig_ptr;
    use crate::section_0144::new_lig_item;
    use crate::section_0549::non_char;
}}
