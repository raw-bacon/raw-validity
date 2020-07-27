pub mod truncated_group;
pub mod truncated_subgroup;

pub trait Closable {
    fn close(&mut self);
}