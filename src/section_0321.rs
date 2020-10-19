//! @* \[23] Maintaining the input stacks.
//! The following subroutines change the input status in commonly needed ways.
//!
//! First comes |push_input|, which stores the current state and creates a
//! new level (having, initially, the same properties as the old).
//!
//! @d push_input==@t@> {enter a new input level, save the old}
//!   begin if input_ptr>max_in_stack then
//!     begin max_in_stack:=input_ptr;
//!     if input_ptr=stack_size then overflow("input stack size",stack_size);
//! @:TeX capacity exceeded input stack size}{\quad input stack size@>
//!     end;
//!   input_stack[input_ptr]:=cur_input; {stack the record}
//!   incr(input_ptr);
//!   end
//!
