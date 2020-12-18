//! @ @<Numbered cases...@>=
//! 1: print_word(mem[n]); {display |mem[n]| in all forms}
//! 2: print_int(info(n));
//! 3: print_int(link(n));
//! 4: print_word(eqtb[n]);
//! 5: print_word(font_info[n]);
//! 6: print_word(save_stack[n]);
//! 7: show_box(n);
//!   {show a box, abbreviated by |show_box_depth| and |show_box_breadth|}
//! 8: begin breadth_max:=10000; depth_threshold:=pool_size-pool_ptr-10;
//!   show_node_list(n); {show a box in its entirety}
//!   end;
//! 9: show_token_list(n,null,1000);
//! 10: slow_print(n);
//! 11: check_mem(n>0); {check wellformedness; print new busy locations if |n>0|}
//! 12: search_mem(n); {look for pointers to |n|}
//! 13: begin read(term_in,l); print_cmd_chr(n,l);
//!   end;
//! 14: for k:=0 to n do print(buffer[k]);
//! 15: begin font_in_short_display:=null_font; short_display(n);
//!   end;
//! 16: panicking:=not panicking;
//!
