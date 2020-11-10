//! @ Before we put a new token list on the input stack, it is wise to clean off
//! all token lists that have recently been depleted. Then a user macro that ends
//! with a call to itself will not require unbounded stack space.
//
// @<Feed the macro body and its parameters to the scanner@>=
macro_rules! Feed_the_macro_body_and_its_parameters_to_the_scanner {
    ($globals:expr, $ref_count:expr, $r:expr, $n:expr) => {{
        // while (state=token_list)and(loc=null)and(token_type<>v_template) do
        //   end_token_list; {conserve stack space}
        // begin_token_list(ref_count,macro); name:=warning_index; loc:=link(r);
        begin_token_list($globals, $ref_count, r#macro);
        name!($globals) = $globals.warning_index;
        loc!($globals) = link!($globals, $r);
        // if n>0 then
        if $n > 0 {
            // begin if param_ptr+n>max_param_stack then
            //   begin max_param_stack:=param_ptr+n;
            //   if max_param_stack>param_size then
            //     overflow("parameter stack size",param_size);
            // @:TeX capacity exceeded parameter stack size}{\quad parameter stack size@>
            //     end;
            //   for m:=0 to n-1 do param_stack[param_ptr+m]:=pstack[m];
            //   param_ptr:=param_ptr+n;
            //   end
        }
        use crate::section_0307::r#macro;
        use crate::section_0323::begin_token_list;
    }}
}
