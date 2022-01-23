//! ` `
// @<Construct an extensible...@>=
pub(crate) macro Construct_an_extensible_character_in_a_new_box_b__using_recipe_rem_byte_q_and_font_f($globals:expr, $b:expr, $q:expr, $v:expr, $f:expr, $c:expr) {{
    /// largest height-plus-depth so far
    let mut w;
    /// extensible pieces
    let r;
    /// the number of extensible pieces
    let mut n;
    // begin b:=new_null_box;
    $b = new_null_box($globals)?;
    // type(b):=vlist_node;
    r#type!($globals, $b) = vlist_node;
    // r:=font_info[exten_base[f]+rem_byte(q)].qqqq;@/
    r = $globals.font_info[($globals.exten_base[$f] + $q.rem_byte() as integer) as font_index_repr][MEMORY_WORD_EXTENSIBLE_RECIPE];
    // @<Compute the minimum suitable height, |w|, and the corresponding
    //   number of extension steps, |n|; also set |width(b)|@>;
    crate::section_0714::Compute_the_minimum_suitable_height__w__and_the_corresponding_number_of_extension_steps__n__also_set_width_b!($globals, $b, $q, $v, $f, $c, r, w, n);
    // c:=ext_bot(r);
    $c = ASCII_code::from(r.ext_bot() as integer);
    // if c<>min_quarterword then stack_into_box(b,f,c);
    if $c.numeric_value() != min_quarterword as _ {
        stack_into_box($globals, $b, $f, $c)?;
    }
    // c:=ext_rep(r);
    $c = ASCII_code::from(r.ext_rep() as integer);
    // for m:=1 to n do stack_into_box(b,f,c);
    for _ in 1..=n {
        stack_into_box($globals, $b, $f, $c)?;
    }
    // c:=ext_mid(r);
    $c = ASCII_code::from(r.ext_mid() as integer);
    // if c<>min_quarterword then
    if $c.numeric_value() != min_quarterword as _ {
        // begin stack_into_box(b,f,c); c:=ext_rep(r);
        stack_into_box($globals, $b, $f, $c)?;
        $c = ASCII_code::from(r.ext_rep() as integer);
        // for m:=1 to n do stack_into_box(b,f,c);
        for _ in 1..=n {
            stack_into_box($globals, $b, $f, $c)?;
        }
        // end;
    }
    // c:=ext_top(r);
    $c = ASCII_code::from(r.ext_top() as integer);
    // if c<>min_quarterword then stack_into_box(b,f,c);
    if $c.numeric_value() != min_quarterword as _ {
        stack_into_box($globals, $b, $f, $c)?;
    }
    // depth(b):=w-height(b);
    depth!($globals, $b) = w - height!($globals, $b);
    // end
    use crate::pascal::integer;
    use crate::section_0018::ASCII_code;
    use crate::section_0110::min_quarterword;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0546::MEMORY_WORD_EXTENSIBLE_RECIPE;
    use crate::section_0136::new_null_box;
    use crate::section_0137::vlist_node;
    use crate::section_0548::font_index_repr;
    use crate::section_0711::stack_into_box;
}}
