//! ` `

// @<Create a new glue specification whose width is |cur_val|...@>=
macro_rules! Create_a_new_glue_specification_whose_width_is_cur_val__scan_for_its_stretch_and_shrink_components {
    ($globals:expr, $mu:expr) => {{
        /// new glue specification
        let q: pointer;
        // q:=new_spec(zero_glue); width(q):=cur_val;
        q = new_spec($globals, zero_glue)?;
        width!($globals, q) = scaled::new_from_inner($globals.cur_val);
        // if scan_keyword("plus") then
        if scan_keyword($globals, strpool_str!("plus"))? {
            // @.plus@>
            // begin scan_dimen(mu,true,false);
            scan_dimen($globals, $mu, true, false)?;
            // stretch(q):=cur_val; stretch_order(q):=cur_order;
            stretch!($globals, q) = scaled::new_from_inner($globals.cur_val);
            stretch_order!($globals, q) = $globals.cur_order as _;
            // end;
        }
        // if scan_keyword("minus") then
        if scan_keyword($globals, strpool_str!("minus"))? {
            // @.minus@>
            // begin scan_dimen(mu,true,false);
            scan_dimen($globals, $mu, true, false)?;
            // shrink(q):=cur_val; shrink_order(q):=cur_order;
            shrink!($globals, q) = scaled::new_from_inner($globals.cur_val);
            shrink_order!($globals, q) = $globals.cur_order as _;
            // end;
        }
        // cur_val:=q
        $globals.cur_val = q as _;
        use crate::section_0101::scaled;
        use crate::section_0115::pointer;
        use crate::section_0151::new_spec;
        use crate::section_0162::zero_glue;
        use crate::section_0407::scan_keyword;
    }}
}