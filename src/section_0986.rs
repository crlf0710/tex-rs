//! ` `
// @<Show the status of the current page@>=
macro_rules! Show_the_status_of_the_current_page {
    ($globals:expr) => {{
        // if page_head<>page_tail then
        if page_head != $globals.page_tail {
            todo!("show status of page");
            //   begin print_nl("### current page:");
            //   if output_active then print(" (held over for next output)");
            // @.held over for next output@>
            //   show_box(link(page_head));
            //   if page_contents>empty then
            //     begin print_nl("total height "); print_totals;
            // @:total_height}{\.{total height}@>
            //     print_nl(" goal height "); print_scaled(page_goal);
            // @.goal height@>
            //     r:=link(page_ins_head);
            //     while r<>page_ins_head do
            //       begin print_ln; print_esc("insert"); t:=qo(subtype(r));
            //       print_int(t); print(" adds ");
            //       if count(t)=1000 then t:=height(r)
            //       else t:=x_over_n(height(r),1000)*count(t);
            //       print_scaled(t);
            //       if type(r)=split_up then
            //         begin q:=page_head; t:=0;
            //         repeat q:=link(q);
            //         if (type(q)=ins_node)and(subtype(q)=subtype(r)) then incr(t);
            //         until q=broken_ins(r);
            //         print(", #"); print_int(t); print(" might split");
            //         end;
            //       r:=link(r);
            //       end;
            //     end;
            //   end
        }
        use crate::section_0162::page_head;
    }}
}