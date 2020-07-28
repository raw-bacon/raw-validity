pub mod truncated_group;
pub mod truncated_subgroup;
pub mod tiny_truncated_group;

pub trait Closable {
    fn close(&mut self);
}