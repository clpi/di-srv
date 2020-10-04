pub mod person;
pub mod event;
pub mod place;

pub use person::Person;
pub use place::Place;
pub use event::Event;

pub trait Entity {}
