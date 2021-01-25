//! ` `

// @<Display box |p|@>=
#[allow(unused_macros)]
macro_rules! Display_box_p {
    ($globals:expr, $p:expr) => {{
        // begin if type(p)=hlist_node then print_esc("h")
        if r#type!($globals, $p) == hlist_node {
            print_esc($globals, strpool_str!("h"));
        }
        // else if type(p)=vlist_node then print_esc("v")
        else if r#type!($globals, $p) == vlist_node {
            print_esc($globals, strpool_str!("v"));
        }
        // else print_esc("unset");
        else {
            print_esc($globals, strpool_str!("unset"));
        }
        todo!("display box p");
        // print("box("); print_scaled(height(p)); print_char("+");
        // print_scaled(depth(p)); print(")x"); print_scaled(width(p));
        // if type(p)=unset_node then
        //   @<Display special fields of the unset node |p|@>
        // else  begin @<Display the value of |glue_set(p)|@>;
        //   if shift_amount(p)<>0 then
        //     begin print(", shifted "); print_scaled(shift_amount(p));
        //     end;
        //   end;
        // node_list_display(list_ptr(p)); {recursive call}
        // end
    }}
}
