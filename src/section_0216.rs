//! @ When \TeX's work on one level is interrupted, the state is saved by
//! calling |push_nest|. This routine changes |head| and |tail| so that
//! a new (empty) list is begun; it does not change |mode| or |aux|.
//!
//! @p procedure push_nest; {enter a new semantic level, save the old}
//! begin if nest_ptr>max_nest_stack then
//!   begin max_nest_stack:=nest_ptr;
//!   if nest_ptr=nest_size then overflow("semantic nest size",nest_size);
//! @:TeX capacity exceeded semantic nest size}{\quad semantic nest size@>
//!   end;
//! nest[nest_ptr]:=cur_list; {stack the record}
//! incr(nest_ptr); head:=get_avail; tail:=head; prev_graf:=0; mode_line:=line;
//! end;
//!
