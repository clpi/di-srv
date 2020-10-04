pub mod recorditem;
pub mod itemfield;
pub mod userrecord;
pub mod usergroup;

pub struct Link<T, U> {
    model1: T,
    model2: U,
    info: LinkInfo,
}

pub struct LinkInfo;

impl LinkInfo {
    pub fn new() -> Self { LinkInfo }
}

pub use userrecord::UserRecordLink;
pub use recorditem::RecordItemLink;
pub use itemfield::ItemFieldLink;
pub use usergroup::UserGroupLink;

pub trait LinkModel : Default + Sized {}

