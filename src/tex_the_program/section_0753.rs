//! @ Note that a ligature between an |ord_noad| and another kind of noad
//! is replaced by an |ord_noad|, when the two noads collapse into one.
//! But we could make a parenthesis (say) change shape when it follows
//! certain letters. Presumably a font designer will define such
//! ligatures only when this convention makes sense.
//!
//! \chardef\?='174 % vertical line to indicate character retention
//
// @<If instruction |cur_i| is a kern with |cur_c|, ...@>=
pub(crate) macro If_instruction_cur_i_is_a_kern_with_cur_c__attach_the_kern_after_q__or_if_it_is_a_ligature_with_cur_c__combine_noads_q_and_p_appropriately__then_return_if_the_cursor_has_moved_past_a_noad__or_goto_restart($globals:expr, $p:expr, $q:expr, $cur_f:expr, $cur_c:expr, $cur_i:expr, $lbl_restart:lifetime) {{
    // if next_char(cur_i)=cur_c then if skip_byte(cur_i)<=stop_flag then
    if $cur_i.next_char() as integer == $cur_c.numeric_value() as integer
        && $cur_i.skip_byte() <= stop_flag
    {
        // if op_byte(cur_i)>=kern_flag then
        if $cur_i.op_byte() >= kern_flag {
            // begin p:=new_kern(char_kern(cur_f)(cur_i));
            $p = new_kern($globals, char_kern!($globals, $cur_f, $cur_i))?;
            // link(p):=link(q); link(q):=p; return;
            link!($globals, $p) = link!($globals, $q);
            link!($globals, $q) = $p;
            crate::return_nojump!();
            // end
        }
        // else  begin check_interrupt; {allow a way out of infinite ligature loop}
        else {
            /// allow a way out of infinite ligature loop
            check_interrupt!($globals);
            // case op_byte(cur_i) of
            match qo!($cur_i.op_byte()) {
                // qi(1),qi(5): character(nucleus(q)):=rem_byte(cur_i); {\.{=:\?}, \.{=:\?>}}
                1 | 5 => {
                    /// `=:|, =:|>`
                    const _: () = ();
                    let fam_nucleus_q = fam!($globals, nucleus!($q));
                    assign_fam_and_character!(
                        $globals,
                        nucleus!($q),
                        fam_nucleus_q,
                        ASCII_code::from($cur_i.rem_byte() as integer)
                    );
                }
                // qi(2),qi(6): character(nucleus(p)):=rem_byte(cur_i); {\.{\?=:}, \.{\?=:>}}
                2 | 6 => {
                    /// `|=:, |=:>`
                    const _: () = ();
                    let fam_nucleus_p = fam!($globals, nucleus!($p));
                    assign_fam_and_character!(
                        $globals,
                        nucleus!($p),
                        fam_nucleus_p,
                        ASCII_code::from($cur_i.rem_byte() as integer)
                    );
                }
                // qi(3),qi(7),qi(11):begin r:=new_noad; {\.{\?=:\?}, \.{\?=:\?>}, \.{\?=:\?>>}}
                3 | 7 | 11 => {
                    /// `|=:|, |=:|>, |=:|>>`
                    const _: () = ();
                    /// temporary registers for list manipulation
                    let r;
                    r = new_noad($globals)?;
                    // character(nucleus(r)):=rem_byte(cur_i);
                    // fam(nucleus(r)):=fam(nucleus(q));@/
                    let fam_nucleus_q = fam!($globals, nucleus!($q));
                    assign_fam_and_character!(
                        $globals,
                        nucleus!(r),
                        fam_nucleus_q,
                        ASCII_code::from($cur_i.rem_byte() as integer)
                    );
                    // link(q):=r; link(r):=p;
                    link!($globals, $q) = r;
                    link!($globals, r) = $p;
                    // if op_byte(cur_i)<qi(11) then math_type(nucleus(r)):=math_char
                    if qo!($cur_i.op_byte()) < 11 {
                        math_type!($globals, nucleus!(r)) = math_type_kind::math_char as _;
                    }
                    // else math_type(nucleus(r)):=math_text_char; {prevent combination}
                    else {
                        /// prevent combination
                        const _: () = ();
                        math_type!($globals, nucleus!(r)) = math_type_kind::math_text_char as _;
                        // end;
                    }
                } //   othercases begin link(q):=link(p);
                _ => {
                    link!($globals, $q) = link!($globals, $p);
                    // character(nucleus(q)):=rem_byte(cur_i); {\.{=:}}
                    /// `=:`
                    let fam_nucleus_q = fam!($globals, nucleus!($q));
                    assign_fam_and_character!(
                        $globals,
                        nucleus!($q),
                        fam_nucleus_q,
                        ASCII_code::from($cur_i.rem_byte() as integer)
                    );
                    // mem[subscr(q)]:=mem[subscr(p)]; mem[supscr(q)]:=mem[supscr(p)];@/
                    $globals.mem[subscr!($q)] = $globals.mem[subscr!($p)];
                    $globals.mem[supscr!($q)] = $globals.mem[supscr!($p)];
                    // free_node(p,noad_size);
                    free_node($globals, $p, noad_size as _);
                    // end
                } // endcases;
            }
            // if op_byte(cur_i)>qi(3) then return;
            if qo!($cur_i.op_byte()) > 3 {
                crate::return_nojump!();
            }
            // math_type(nucleus(q)):=math_char; goto restart;
            math_type!($globals, nucleus!($q)) = math_type_kind::math_char as _;
            crate::goto_backward_label!($lbl_restart);
            // end
        }
    }
    use crate::pascal::integer;
    use crate::section_0018::ASCII_code;
    use crate::section_0096::check_interrupt;
    use crate::section_0112::qo;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0156::new_kern;
    use crate::section_0545::kern_flag;
    use crate::section_0545::stop_flag;
    use crate::section_0557::char_kern;
    use crate::section_0681::assign_fam_and_character;
    use crate::section_0681::fam;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::noad_size;
    use crate::section_0681::nucleus;
    use crate::section_0681::subscr;
    use crate::section_0681::supscr;
    use crate::section_0686::new_noad;
}}
