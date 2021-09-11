//! @ The active node that represents the starting point does not need a
//! corresponding passive node.
//
// @d store_background(#)==active_width[#]:=background[#]
pub(crate) macro store_background($globals:expr, $idx:expr) {{
    $globals.active_width[$idx] = $globals.background[$idx]
}}

// @<Create an active breakpoint representing the beginning of the paragraph@>=
pub(crate) macro Create_an_active_breakpoint_representing_the_beginning_of_the_paragraph
    ($globals:expr) {{
        /// miscellaneous nodes of temporary interest
        let q: pointer;
        // q:=get_node(active_node_size);
        q = get_node($globals, active_node_size as _)?;
        // type(q):=unhyphenated; fitness(q):=decent_fit;
        r#type!($globals, q) = unhyphenated;
        fitness!($globals, q) = fit_class_kind::decent_fit as _;
        // link(q):=last_active; break_node(q):=null;
        link!($globals, q) = last_active!();
        break_node!($globals, q) = null;
        // line_number(q):=prev_graf+1; total_demerits(q):=0; link(active):=q;
        line_number!($globals, q) = (prev_graf!($globals) + 1) as _;
        total_demerits!($globals, q) = 0;
        link!($globals, active) = q;
        // do_all_six(store_background);@/
        do_all_six!(store_background !; @globals = $globals);
        // passive:=null; printed_node:=temp_head; pass_number:=0;
        $globals.passive = null;
        $globals.printed_node = temp_head;
        $globals.pass_number = 0;
        // font_in_short_display:=null_font
        $globals.font_in_short_display = null_font.get() as _;
        use crate::section_0115::null;
        use crate::section_0115::pointer;
        use crate::section_0118::link;
        use crate::section_0125::get_node;
        use crate::section_0133::r#type;
        use crate::section_0162::active;
        use crate::section_0162::temp_head;
        use crate::section_0213::prev_graf;
        use crate::section_0232::null_font;
        use crate::section_0817::fit_class_kind;
        use crate::section_0819::active_node_size;
        use crate::section_0819::break_node;
        use crate::section_0819::fitness;
        use crate::section_0819::unhyphenated;
        use crate::section_0819::line_number;
        use crate::section_0819::last_active;
        use crate::section_0819::total_demerits;
        use crate::section_0823::do_all_six;
    }}
