use std::error;
use std::fmt;
use std::num;
use std::string::{String};

#[derive(Debug)]
pub enum ColladaError {
    ParseError,

    /// General Validation Error
    Invalid{msg: String},

    /// Element contains an invalid child
    InvalidChild{child: String, parent: String},

    /// Element contains an invalid attribute
    InvalidAttr{elem: String, attr: String},
    
    /// Element contains invalid attribute data
    InvalidAttrData{elem: String, attr: String, data: String},
    
    /// Element contains invalid data
    InvalidData{elem: String, data: String},

    /// Required element is missing
    MissingElement{structure: String, elem: String},

    /// Required attribute is missing from element
    MissingAttr{elem: String, attr: String},

    /// Element is missing required data, e.g. <tag><!-- MISSING DATA HERE --></tag>
    MissingData{elem: String},
}

impl fmt::Display for ColladaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColladaError::ParseError => 
                write!(f, "Unable to parse data"),
            ColladaError::Invalid{ref msg} => 
                write!(f, "Invalid collada: {}", msg),
            ColladaError::InvalidChild{ref child, ref parent} =>
                write!(f, "Element <{}> has invalid child <{}>", parent, child),
            ColladaError::InvalidAttr{ref elem, ref attr} => 
                write!(f, "Element <{}> has invalid attribute '{}'", elem, attr),
            ColladaError::InvalidAttrData{ref elem, ref attr, ref data} =>
                write!(f, "Element <{}> has attribute '{}' with invalid data: {}", elem, attr, data),
            ColladaError::InvalidData{ref elem, ref data} =>
                write!(f, "Element <{}> has invalid data: {}", elem, data),
            ColladaError::MissingElement{ref structure, ref elem} =>
                write!(f, "Parsing '{}' but required <{}> element not found", structure, elem),
            ColladaError::MissingAttr{ref elem, ref attr} =>
                write!(f, "Element <{}> is missing required attribute: {}", elem, attr),
            ColladaError::MissingData{ref elem} =>
                write!(f, "Element <{}> is missing required data", elem),
        }
    }
}

impl error::Error for ColladaError {
    fn description(&self) -> &str {
        match *self {
            ColladaError::ParseError => "Parse error",
            ColladaError::Invalid{..} => "Invalid collada",
            ColladaError::InvalidChild{..} => "Invalid child",
            ColladaError::InvalidAttr{..} => "Invalid attribute",
            ColladaError::InvalidAttrData{..} => "Invalid attribute data",
            ColladaError::InvalidData{..} => "Invalid data",
            ColladaError::MissingElement{..} => "Missing required element",
            ColladaError::MissingAttr{..} => "Missing required attribute",
            ColladaError::MissingData{..} => "Missing required element data",
        }
    }
}

impl From<num::ParseFloatError> for ColladaError {
    fn from(_: num::ParseFloatError) -> ColladaError {
        ColladaError::ParseError
    }
}
