//! ` `
// @<Ship box |p| out@>=
pub(crate) macro Ship_box_p_out($globals:expr, $p:expr) {{
    crate::trace_span!("Ship box `p` out");
    /// location of the current `bop`
    let page_loc: integer;
    // @<Update the values of |max_h| and |max_v|; but if the page is too large,
    //   |goto done|@>;
    crate::section_0641::Update_the_values_of_max_h_and_max_v__but_if_the_page_is_too_large__goto_done!($globals, $p);
    // @<Initialize variables as |ship_out| begins@>;
    crate::section_0617::Initialize_variables_as_ship_out_begins!($globals);
    // page_loc:=dvi_offset+dvi_ptr;
    page_loc = $globals.dvi_offset + $globals.dvi_ptr.get() as integer;
    // dvi_out(bop);
    dvi_out!($globals, bop.byte());
    // for k:=0 to 9 do dvi_four(count(k));
    for k in 0..=9 {
        dvi_four($globals, count!($globals, k));
    }
    // dvi_four(last_bop); last_bop:=page_loc;
    dvi_four($globals, $globals.last_bop);
    $globals.last_bop = page_loc;
    // cur_v:=height(p)+v_offset; temp_ptr:=p;
    $globals.cur_v = height!($globals, $p) + v_offset!($globals);
    $globals.temp_ptr = $p;
    // if type(p)=vlist_node then vlist_out@+else hlist_out;
    if r#type!($globals, $p) == vlist_node {
        vlist_out($globals)?;
    } else {
        hlist_out($globals)?;
    }
    // dvi_out(eop); incr(total_pages); cur_s:=-1;
    dvi_out!($globals, eop.byte());
    incr!($globals.total_pages);
    $globals.cur_s = -1;
    // done:
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0133::r#type;
    use crate::section_0135::height;
    use crate::section_0137::vlist_node;
    use crate::section_0236::count;
    use crate::section_0247::v_offset;
    use crate::section_0586::bop;
    use crate::section_0586::eop;
    use crate::section_0598::dvi_out;
    use crate::section_0600::dvi_four;
    use crate::section_0619::hlist_out;
    use crate::section_0629::vlist_out;
}}
