//! ` `
// @<Swap the subscript and superscript into box |x|@>=
pub(crate) macro Swap_the_subscript_and_superscript_into_box_x($globals:expr, $q:expr, $x:expr, $h:expr, $delta:expr) {{
    // begin flush_node_list(x); x:=new_noad;
    flush_node_list($globals, $x)?;
    $x = new_noad($globals)?;
    // mem[nucleus(x)]:=mem[nucleus(q)];
    $globals.mem[nucleus!($x)] = $globals.mem[nucleus!($q)];
    // mem[supscr(x)]:=mem[supscr(q)];
    $globals.mem[supscr!($x)] = $globals.mem[supscr!($q)];
    // mem[subscr(x)]:=mem[subscr(q)];@/
    $globals.mem[subscr!($x)] = $globals.mem[subscr!($q)];
    // mem[supscr(q)].hh:=empty_field;
    $globals.mem[supscr!($q)][MEMORY_WORD_HH] = empty_field;
    // mem[subscr(q)].hh:=empty_field;@/
    $globals.mem[subscr!($q)][MEMORY_WORD_HH] = empty_field;
    // math_type(nucleus(q)):=sub_mlist; info(nucleus(q)):=x;
    math_type!($globals, nucleus!($q)) = math_type_kind::sub_mlist as _;
    info_inner!($globals, nucleus!($q)) = $x;
    // x:=clean_box(nucleus(q),cur_style); delta:=delta+height(x)-h; h:=height(x);
    $x = clean_box($globals, nucleus!($q), $globals.cur_style)?;
    $delta = $delta + height!($globals, $x) - $h;
    $h = height!($globals, $x);
    // end
    use crate::section_0113::MEMORY_WORD_HH;
    use crate::section_0118::info_inner;
    use crate::section_0135::height;
    use crate::section_0202::flush_node_list;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::nucleus;
    use crate::section_0681::subscr;
    use crate::section_0681::supscr;
    use crate::section_0684::empty_field;
    use crate::section_0686::new_noad;
    use crate::section_0720::clean_box;
}}
