//! Element attributes are a powerful means of controlling the built-in settings
//! of individual block and inline elements in the AsciiDoc syntax. They can
//! also be used to add supplemental information, such as citation metadata and
//! fallback content, to certain elements.

mod element_attribute;
pub use element_attribute::ElementAttribute;

mod attrlist;
pub use attrlist::Attrlist;
