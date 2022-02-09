//! @ We use |shift_up| and |shift_down| in the following program for the
//! amount of glue between the displayed operator |y| and its limits |x| and
//! |z|. The vlist inside box |v| will consist of |x| followed by |y| followed
//! by |z|, with kern nodes for the spaces between and around them.
//
// @<Attach the limits to |y| and adjust |height(v)|, |depth(v)|...@>=
pub(crate) macro Attach_the_limits_to_y_and_adjust_height_v__depth_v_to_account_for_their_presence($globals:expr, $q:expr, $x:expr, $y:expr, $z:expr, $v:expr) {{
    // if math_type(supscr(q))=empty then
    if math_type!($globals, supscr!($q)) == math_type_kind::empty as _ {
        // begin free_node(x,box_node_size); list_ptr(v):=y;
        free_node($globals, $x, box_node_size as _);
        list_ptr!($globals, $v) = $y;
        // end
    }
    // else  begin shift_up:=big_op_spacing3-depth(x);
    else {
        /// temporary registers for box construction
        let mut p;
        /// dimensions for box calculation
        let mut shift_up;
        shift_up = big_op_spacing3!($globals) - depth!($globals, $x);
        // if shift_up<big_op_spacing1 then shift_up:=big_op_spacing1;
        if shift_up < big_op_spacing1!($globals) {
            shift_up = big_op_spacing1!($globals);
        }
        // p:=new_kern(shift_up); link(p):=y; link(x):=p;@/
        p = new_kern($globals, shift_up)?;
        link!($globals, p) = $y;
        link!($globals, $x) = p;
        // p:=new_kern(big_op_spacing5); link(p):=x; list_ptr(v):=p;
        p = new_kern($globals, big_op_spacing5!($globals))?;
        link!($globals, p) = $x;
        list_ptr!($globals, $v) = p;
        // height(v):=height(v)+big_op_spacing5+height(x)+depth(x)+shift_up;
        height!($globals, $v) = height!($globals, $v)
            + big_op_spacing5!($globals)
            + height!($globals, $x)
            + depth!($globals, $x)
            + shift_up;
        // end;
    }
    // if math_type(subscr(q))=empty then free_node(z,box_node_size)
    if math_type!($globals, subscr!($q)) == math_type_kind::empty as _ {
        free_node($globals, $z, box_node_size as _);
    }
    // else  begin shift_down:=big_op_spacing4-height(z);
    else {
        /// temporary registers for box construction
        let mut p;
        /// dimensions for box calculation
        let mut shift_down;
        shift_down = big_op_spacing4!($globals) - height!($globals, $z);
        // if shift_down<big_op_spacing2 then shift_down:=big_op_spacing2;
        if shift_down < big_op_spacing2!($globals) {
            shift_down = big_op_spacing2!($globals);
        }
        // p:=new_kern(shift_down); link(y):=p; link(p):=z;@/
        p = new_kern($globals, shift_down)?;
        link!($globals, $y) = p;
        link!($globals, p) = $z;
        // p:=new_kern(big_op_spacing5); link(z):=p;
        p = new_kern($globals, big_op_spacing5!($globals))?;
        link!($globals, $z) = p;
        // depth(v):=depth(v)+big_op_spacing5+height(z)+depth(z)+shift_down;
        depth!($globals, $v) = depth!($globals, $v)
            + big_op_spacing5!($globals)
            + height!($globals, $z)
            + depth!($globals, $z)
            + shift_down;
        // end
    }
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0135::box_node_size;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::list_ptr;
    use crate::section_0156::new_kern;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::subscr;
    use crate::section_0681::supscr;
    use crate::section_0701::big_op_spacing1;
    use crate::section_0701::big_op_spacing2;
    use crate::section_0701::big_op_spacing3;
    use crate::section_0701::big_op_spacing4;
    use crate::section_0701::big_op_spacing5;
}}
