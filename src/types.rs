use fantoccini::{Locator, Element};

pub type ElementResult = Result<Option<Element>, fantoccini::error::CmdError>;

pub enum Action {
	Stay,
  Click(Element),
  Wait,
	End 
}
