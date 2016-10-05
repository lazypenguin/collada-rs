use xmltree::{Element};
use core::{Asset, Technique};
use error::{ColladaError};
use traits::{XmlConversion};

/// Provides arbitrary additional information about or related to its parent
#[derive(Debug)]
pub struct Extra {
    pub id: Option<String>,
    pub name: Option<String>,
    pub typ: Option<String>,
    pub asset: Option<Asset>,
    pub techniques: Vec<Technique>,
}

impl Extra {
    pub fn new() -> Extra {
        Extra {
            id: None,
            name: None,
            typ: None,
            asset: None,
            techniques: Vec::new(),
        }
    }
}

impl XmlConversion for Extra {
    fn parse(&mut self, e: &Element) -> Result<(), ColladaError> {
        self.id = match e.attributes.get("id") {
            Some(v) => Some(v.clone()),
            None => None,
        };
        
        self.name = match e.attributes.get("name") {
            Some(v) => Some(v.clone()),
            None => None,
        };

        self.typ = match e.attributes.get("type") {
            Some(v) => Some(v.clone()),
            None => None,
        };
        
        match e.get_child("asset") {
            Some(a_node) => {
                let mut a = Asset::new();
                try!(a.parse(a_node));
                self.asset = Some(a);
            },
            None => {},
        }
    
        if e.get_child("technique").is_none() {
            return Err(ColladaError::MissingElement{
                structure: "extra".to_string(),
                elem: "technique".to_string(),
            });
        }
        
        for c in &e.children {
            match c.name.as_str() {
                "technique" => {
                    let mut t = Technique::new();
                    try!(t.parse(c));
                    self.techniques.push(t);
                },
                "asset" => {}, // Skip if asset, this is just to avoid validation error
                _ => return Err(ColladaError::InvalidChild{
                    child: c.name.clone(),
                    parent: "extra".to_string(),
                }),
            }
        }

        Ok(())
    }

    fn encode(&self) -> Element {
        let mut ext = Element::new("extra");

        if self.id.is_some() {
            ext.attributes.insert("id".to_string(), self.id.clone().unwrap());
        }
        if self.name.is_some() {
            ext.attributes.insert("name".to_string(), self.name.clone().unwrap());
        }
        if self.typ.is_some() {
            ext.attributes.insert("type".to_string(), self.typ.clone().unwrap());
        }
        
        match self.asset {
            Some(ref x) => ext.children.push(x.encode()),
            None => {},
        }

        for t in &self.techniques {
            ext.children.push(t.encode());
        }

        ext
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::{Element};
    use core::{Asset, Technique};
    use traits::{XmlConversion};

    #[test]
    fn test_extra_parse() {
        let data = r#"
            <extra id='some-id' name='some-name' type='basic'>
                <asset>
                    <created>2005-06-27T21:00:00z</created>
                    <modified>2005-06-27T21:00:00z</modified>
                </asset>
                <technique profile='lolo'>
                    <oh>fo fo fo</oh>
                </technique>
                <technique profile='lala'>
                    <ah>fa fa fa</ah>
                </technique>
            </extra>"#;
        let e = Element::parse(data.as_bytes()).unwrap();
        let mut x = Extra::new();
        match x.parse(&e) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        assert_eq!(x.id.unwrap().as_str(), "some-id");
        assert_eq!(x.name.unwrap().as_str(), "some-name");
        assert_eq!(x.typ.unwrap().as_str(), "basic");
        assert_eq!(x.asset.is_some(), true);
        let asset = x.asset.unwrap();
        assert_eq!(asset.created, "2005-06-27T21:00:00z");
        assert_eq!(asset.modified, "2005-06-27T21:00:00z");
        assert_eq!(x.techniques.len(), 2);
        assert_eq!(x.techniques[0].profile, "lolo");
        assert_eq!(x.techniques[1].profile, "lala");
    }

    #[test]
    fn test_extra_encode() {
        let mut x = Extra::new();
        x.id = Some(String::from("some-id"));
        x.name = Some(String::from("some-name"));
        x.typ = Some(String::from("basic"));

        let mut a = Asset::new();
        a.created = "2005-06-27T21:00:00z".to_owned();
        a.modified = "2005-06-27T21:00:00z".to_owned();
        x.asset = Some(a);
        
        let mut t = Technique::new();
        t.profile = "foo".to_owned();
        x.techniques.push(t);

        let e = x.encode();
        assert_eq!(e.name, "extra");
        assert_eq!(e.attributes.get("id"), Some(&String::from("some-id")));
        assert_eq!(e.attributes.get("name"), Some(&String::from("some-name")));
        assert_eq!(e.attributes.get("type"), Some(&String::from("basic")));
        assert_eq!(e.children.len(), 2);
        assert_eq!(e.children[0].name, "asset");
        assert_eq!(e.children[0].children.len(), 3);
        assert_eq!(e.children[1].name, "technique");
    }
}

