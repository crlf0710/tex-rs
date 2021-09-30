//! @ We ought to give special care to the efficiency of one part of |hlist_out|,
//! since it belongs to \TeX's inner loop. When a |char_node| is encountered,
//! we save a little time by processing several nodes in succession until
//! reaching a non-|char_node|. The program uses the fact that |set_char_0=0|.
//! @^inner loop@>
//
// @<Output node |p| for |hlist_out|...@>=
pub(crate) macro Output_node_p_for_hlist_out_and_move_to_the_next_node__maintaining_the_condition_cur_v_eq_base_line($globals:expr, $p:expr, $this_box:expr, $base_line:expr, $cur_glue:expr, $cur_g:expr, $g_sign:expr, $g_order:expr) {{
    crate::region_backward_label!{
    'reswitch <-
    {
        // reswitch: if is_char_node(p) then
        if is_char_node!($globals, $p) {
            // begin synch_h; synch_v;
            synch_h!($globals);
            synch_v!($globals);
            // repeat f:=font(p); c:=character(p);
            loop {
                $globals.ship_out_f = font!($globals, $p);
                $globals.ship_out_c = character!($globals, $p);
                // if f<>dvi_f then @<Change font |dvi_f| to |f|@>;
                if $globals.ship_out_f != $globals.dvi_f {
                    crate::section_0621::Change_font_dvi_f_to_f!($globals);
                }
                #[cfg(not(feature = "unicode_support"))]
                {
                    // if c>=qi(128) then dvi_out(set1);
                    if $globals.ship_out_c.numeric_value() >= 128 {
                        dvi_out!($globals, set1.byte());
                    }
                    // dvi_out(qo(c));@/
                    dvi_out!($globals, $globals.ship_out_c.numeric_value());
                }
                #[cfg(feature = "unicode_support")]
                {
                    if $globals.ship_out_c.numeric_value() >= 256 {
                        dvi_out!($globals, set4.byte());
                        dvi_four($globals, $globals.ship_out_c.numeric_value() as _);
                        use crate::section_0586::set4;
                        use crate::section_0600::dvi_four;
                    } else {
                        if $globals.ship_out_c.numeric_value() >= 128 {
                            dvi_out!($globals, set1.byte());
                        }
                        dvi_out!($globals, $globals.ship_out_c.numeric_value());
                    }
                }
                // cur_h:=cur_h+char_width(f)(char_info(f)(c));
                $globals.cur_h += char_width!(
                    $globals,
                    $globals.ship_out_f,
                    char_info!(
                        $globals,
                        $globals.ship_out_f,
                        $globals.ship_out_c.numeric_value()
                    )
                );
                // p:=link(p);
                $p = link!($globals, $p);
                // until not is_char_node(p);
                if !is_char_node!($globals, $p) {
                    break;
                }
            }
            // dvi_h:=cur_h;
            $globals.dvi_h = $globals.cur_h;
            // end
        }
        // else @<Output the non-|char_node| |p| for |hlist_out|
        //     and move to the next node@>
        else {
            crate::section_0622::Output_the_non_char_node_p_for_hlist_out_and_move_to_the_next_node!(
                $globals, $p, $this_box, $base_line, $cur_glue, $cur_g, $g_sign, $g_order, 'reswitch
            );
        }
    }
    |'reswitch|
    };
    use crate::section_0118::link;
    use crate::section_0134::character;
    use crate::section_0134::font;
    use crate::section_0134::is_char_node;
    use crate::section_0554::char_info;
    use crate::section_0554::char_width;
    use crate::section_0586::set1;
    use crate::section_0598::dvi_out;
    use crate::section_0616::synch_h;
    use crate::section_0616::synch_v;
}}
