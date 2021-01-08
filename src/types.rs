use fantoccini::{Locator, Element, Form};

pub type ElementResult = Result<Option<Element>, fantoccini::error::CmdError>;

pub enum Action {
	Stay,
  Click(Element),
  Wait,
	End,
	Submit(Form)
}
