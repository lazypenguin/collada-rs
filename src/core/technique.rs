use std::collections::{HashMap};
use xmltree::{Element};
use error::{ColladaError};
use traits::{XmlConversion};

/// A technique describes information needed by a specific platform or program. 
#[derive(Debug)]
pub struct Technique {
    /// Vendor-defined string that indicates the platform/target for this technique
    pub profile: String,

    /// XML Schema namespace for validation 
    pub xmlns: Option<String>,

    /// Any well-formed XML data stored as an XML Element
    pub data: Element,
}

impl Technique {
    pub fn new() -> Technique{
        Technique {
            profile: String::from(""),
            xmlns: None,
            data: Element {
                name: String::from("technique"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: None,
            }
        }
    }
}

impl XmlConversion for Technique {
    fn parse(&mut self, e: &Element) -> Result<(), ColladaError> {
        if e.name != "technique".to_owned() {
            return Err(ColladaError::MissingElement{
                structure: "technique".to_string(),
                elem: "technique".to_string(),
            });
        }
        
        self.profile = match e.attributes.get("profile") {
            Some(p) => p.clone(),
            None => return Err(ColladaError::MissingAttr{
                elem: "technique".to_string(),
                attr: "profile".to_string(),
            }),
        };

        // TODO: Parse xmlns attribute once parser is fixed in xmltree-rs
        self.data = e.clone();
        Ok(())
    }

    fn encode(&self) -> Element {
        // We only need to clone the data because it already contains the 
        // entire <technique> tag (including attributes profile/xmlns)
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap};
    use xmltree::{Element};
    use traits::{XmlConversion};

    #[test]
    fn test_technique_parse() {
        let data = r#"
            <technique profile="Max" xmlns:max="some/max/schema">
                <param name="wow" sid="animated" type="string">
                    A validated string parameter from the COLLADA scheme.
                </param>
                <max:someElement>defined in the Max schema and validated.</max:someElement>
                <uhoh>something well-formed and level, but that can't be validated
                because there is no schema for it!</uhoh>
            </technique>"#;
        let e = Element::parse(data.as_bytes()).unwrap();
        let mut t = Technique::new();
        match t.parse(&e) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        assert_eq!(t.profile.as_str(), "Max");
        // TODO: Enable this test once namespace parsing is fixed in xmltree
        //assert_eq!(t.xmlns.unwrap().as_str(), "some/max/schema");
        let d = &t.data;
        assert_eq!(d.name, "technique");
        assert_eq!(d.children.len(), 3);
        assert_eq!(d.children[0].name.as_str(), "param");
        // TODO: Enable this test once namespace parsing is fixed in xmltree
        //assert_eq!(data.children[1].name.as_str(), "max:someElement");
        assert_eq!(d.children[2].name.as_str(), "uhoh");
    }

    #[test]
    fn test_technique_encode(){
        let mut data = Element {
            name: String::from("technique"),
            attributes: HashMap::new(),
            children: vec![
                Element {
                     name: String::from("max:SomeElement"),
                     attributes: HashMap::new(),
                     children: Vec::new(),
                     text: Some(String::from("defined in the Max schema and validated.")),
                },
                Element {
                    name: String::from("uhoh"),
                    attributes: HashMap::new(),
                    children: Vec::new(),
                    text: Some(String::from("some string")),
                },
            ],
            text: None,
        };
        data.attributes.insert("profile".to_owned(), "max".to_owned());
        data.attributes.insert("xmlns:max".to_owned(), "some/max/schema".to_owned());

        let mut t = Technique::new();
        match t.parse(&data) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
         
        let e = t.encode();
        assert_eq!(e.name, "technique");
        assert_eq!(e.children.len(), 2);
        assert_eq!(e.attributes.get("profile"), Some(&String::from("max")));
        // TODO: Write test for namespace attribute
    }
}
