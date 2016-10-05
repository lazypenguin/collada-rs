use xmltree::{Element};
use error::{ColladaError};

pub trait XmlConversion {
    /// Parse data from an xml element into struct.
    fn parse(&mut self, e: &Element) -> Result<(), ColladaError>;

    /// Encode struct data as an xml element 
    fn encode(&self) -> Element;
}


