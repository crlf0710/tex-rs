//! @ At this time |p| points to the mlist for the formula; |a| is either
//! |null| or it points to a box containing the equation number; and we are in
//! vertical mode (or internal vertical mode).
//
// @<Finish displayed math@>=
pub(crate) macro Finish_displayed_math($globals:expr, $p:expr, $a:expr, $l:expr, $danger:expr) {{
    /// box containing the equation
    let b;
    /// width of the equation
    let w;
    /// width of the line
    let z;
    /// width of equation number
    let e;
    /// width of equation number plus space to separate from equation
    let q;
    /// displacement of equation in the line
    let d: scaled;
    /// move the line right this much
    let s;
    /// tail of adjustment list
    let t;
    /// glue parameter codes for before and after
    let (g1, mut g2);

    // cur_mlist:=p; cur_style:=display_style; mlist_penalties:=false;
    $globals.cur_mlist = $p;
    $globals.cur_style = (style_node_subtype::display_style as u8).into();
    $globals.mlist_penalties = false;
    // mlist_to_hlist; p:=link(temp_head);@/
    mlist_to_hlist($globals)?;
    $p = link!($globals, temp_head);
    // adjust_tail:=adjust_head; b:=hpack(p,natural); p:=list_ptr(b);
    $globals.adjust_tail = adjust_head;
    b = hpack($globals, $p, natural0!(), natural1!())?;
    $p = list_ptr!($globals, b);
    // t:=adjust_tail; adjust_tail:=null;@/
    t = $globals.adjust_tail;
    $globals.adjust_tail = null;
    // w:=width(b); z:=display_width; s:=display_indent;
    w = width!($globals, b);
    z = display_width!($globals);
    s = display_indent!($globals);
    // if (a=null)or danger then
    if $a == null || $danger {
        // begin e:=0; q:=0;
        e = scaled::zero();
        q = scaled::zero();
        // end
    }
    // else  begin e:=width(a); q:=e+math_quad(text_size);
    else {
        e = width!($globals, $a);
        q = e + math_quad!($globals, text_size);
        // end;
    }
    // if w+q>z then
    if w + q > z {
        // @<Squeeze the equation as much as possible; if there is an equation
        //   number that should go on a separate line by itself,
        //   set~|e:=0|@>;
        todo!("Squeeze");
    }

    // @<Determine the displacement, |d|, of the left edge of the equation, with
    //   respect to the line size |z|, assuming that |l=false|@>;
    crate::section_1202::Determine_the_displacement__d__of_the_left_edge_of_the_equation__with_respect_to_the_line_size_z__assuming_that_l_is_false!($globals, d, z, w, e);
    // @<Append the glue or equation number preceding the display@>;
    crate::section_1203::Append_the_glue_or_equation_number_preceding_the_display!($globals, d, s, $l, e, g1, g2);
    // @<Append the display and perhaps also the equation number@>;
    crate::section_1204::Append_the_display_and_perhaps_also_the_equation_number!($globals, e, s, d, b);
    // @<Append the glue or equation number following the display@>;
    crate::section_1205::Append_the_glue_or_equation_number_following_the_display!($globals, $a, e, $l, t, g2);
    // resume_after_display
    resume_after_display($globals)?;

    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0135::list_ptr;
    use crate::section_0135::width;
    use crate::section_0162::adjust_head;
    use crate::section_0162::temp_head;
    use crate::section_0247::display_width;
    use crate::section_0247::display_indent;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0649::hpack;
    use crate::section_0688::style_node_subtype;
    use crate::section_0699::text_size;
    use crate::section_0700::math_quad;
    use crate::section_0726::mlist_to_hlist;
    use crate::section_1200::resume_after_display;
}}
