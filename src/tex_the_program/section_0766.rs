//! ` `

// @<Append inter-element spacing based on |r_type| and |t|@>=
pub(crate) macro Append_inter_element_spacing_based_on_r_type_and_t($globals:expr, $p:expr, $r_type:expr, $t:expr) {{
    // if r_type>0 then {not the first noad}
    if $r_type > 0 {
        /// not the first noad
        const _: () = ();
        /// temporary registers for list construction
        let x;
        // begin case so(str_pool[r_type*8+t+magic_offset]) of
        let spacing = crate::section_0764::MATH_SPACING[($r_type as isize * 8
            + $t as isize
            + crate::section_0765::MATH_SPACING_OFFSET)
            as usize];
        // "0": x:=0;
        if spacing == b'0' {
            x = 0;
        }
        // "1": if cur_style<script_style then x:=thin_mu_skip_code@+else x:=0;
        else if spacing == b'1' {
            if $globals.cur_style.get() < style_node_subtype::script_style as _ {
                x = thin_mu_skip_code;
            } else {
                x = 0;
            }
        }
        // "2": x:=thin_mu_skip_code;
        else if spacing == b'2' {
            x = thin_mu_skip_code;
        }
        // "3": if cur_style<script_style then x:=med_mu_skip_code@+else x:=0;
        else if spacing == b'3' {
            if $globals.cur_style.get() < style_node_subtype::script_style as _ {
                x = med_mu_skip_code;
            } else {
                x = 0;
            }
        }
        // "4": if cur_style<script_style then x:=thick_mu_skip_code@+else x:=0;
        else if spacing == b'4' {
            if $globals.cur_style.get() < style_node_subtype::script_style as _ {
                x = thick_mu_skip_code;
            } else {
                x = 0;
            }
        }
        // othercases confusion("mlist4")
        else {
            crate::trace_error_expr!("r_type = {}, t = {}, spacing = {}", $r_type, $t, spacing);
            confusion($globals, crate::strpool_str!("mlist4"))?;
            // @:this can't happen mlist4}{\quad mlist4@>
        }
        // endcases;
        // if x<>0 then
        if x != 0 {
            /// temporary registers for list construction
            let (y, z);
            // begin y:=math_glue(glue_par(x),cur_mu);
            y = math_glue($globals, glue_par!($globals, x), $globals.cur_mu)?;
            // z:=new_glue(y); glue_ref_count(y):=null; link(p):=z; p:=z;@/
            z = new_glue($globals, y)?;
            glue_ref_count!($globals, y) = null;
            link!($globals, $p) = z;
            $p = z;
            // subtype(z):=x+1; {store a symbolic subtype}
            /// store a symbolic subtype
            const _: () = ();
            subtype!($globals, z) = x + 1;
            // end;
        }
        // end
    }
    use crate::section_0095::confusion;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0150::glue_ref_count;
    use crate::section_0153::new_glue;
    use crate::section_0224::glue_par;
    use crate::section_0224::med_mu_skip_code;
    use crate::section_0224::thick_mu_skip_code;
    use crate::section_0224::thin_mu_skip_code;
    use crate::section_0688::style_node_subtype;
    use crate::section_0716::math_glue;
}}
