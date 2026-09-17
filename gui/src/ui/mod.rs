pub mod dashboard;
pub mod form;
pub mod list;

#[derive(PartialEq, Eq)]
pub enum Tab {
    Entries,
    Dashboard,
}
