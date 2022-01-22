//! ` `
// @<Insert a dummy...@>=
pub(crate) macro Insert_a_dummy_noad_to_be_sub_superscripted($globals:expr, $p:expr, $t:expr) {{
    // begin tail_append(new_noad);
    tail_append!($globals, new_noad($globals)?);
    // p:=supscr(tail)+cur_cmd-sup_mark; {|supscr| or |subscr|}
    /// `supscr` or `subscr`
    const _: () = ();
    $p = supscr!(tail!($globals)) + ($globals.cur_cmd - sup_mark) as pointer;
    // if t<>empty then
    if $t != math_type_kind::empty as _ {
        // begin if cur_cmd=sup_mark then
        //   begin print_err("Double superscript");
        // @.Double superscript@>
        //   help1("I treat `x^1^2' essentially like `x^1{}^2'.");
        //   end
        // else  begin print_err("Double subscript");
        // @.Double subscript@>
        //   help1("I treat `x_1_2' essentially like `x_1{}_2'.");
        //   end;
        // error;
        // end;
        todo!("t != empty");
    }
    // end
    use crate::section_0115::pointer;
    use crate::section_0207::sup_mark;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::supscr;
    use crate::section_0686::new_noad;
}}
