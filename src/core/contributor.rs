use std::collections::{HashMap};
use xmltree::{Element};
use error::{ColladaError};
use traits::{XmlConversion};


/// Contributor for an asset
#[derive(Debug)]
pub struct Contributor {
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub author_website: Option<String>,
    pub authoring_tool: Option<String>,
    pub comments: Option<String>,
    pub copyright: Option<String>,
    pub source_data: Option<String>,
}

impl Contributor {
    pub fn new() -> Contributor {
        Contributor {
            author: None,
            author_email: None,
            author_website: None,
            authoring_tool: None,
            comments: None,
            copyright: None,
            source_data: None
        }
    }
}

impl XmlConversion for Contributor {
    fn parse(&mut self, e: &Element) -> Result<(), ColladaError> {
        if e.name != "contributor".to_owned() {
            return Err(ColladaError::MissingElement{
                structure: "contributor".to_string(),
                elem: "contributor".to_string(),
            });
        }
        
        for c in &e.children {
            let text = match c.text.clone() {
                Some(t) => t,
                None => return Err(ColladaError::MissingData{
                    elem: c.name.clone(),
                }),
            };

            match c.name.as_str() {
                "author" => self.author = Some(text),
                "author_email" => self.author_email = Some(text),
                "author_website" => self.author_website = Some(text),
                "authoring_tool" => self.authoring_tool = Some(text),
                "comments" => self.comments = Some(text),
                "copyright" => self.copyright = Some(text),
                "source_data" => self.source_data = Some(text),
                _ => return Err(ColladaError::InvalidChild{
                    child: c.name.clone(),
                    parent: "contributor".to_string(),
                }),
            }
        }
        Ok(())
    }

    fn encode(&self) -> Element {
        let mut root = Element {
            name: String::from("contributor"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: None,
        };

        if self.author.is_some() {
            root.children.push(Element{
                name: String::from("author"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: self.author.clone(),
            });
        }

        if self.author_email.is_some() {
            root.children.push(Element{
                name: String::from("author_email"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: self.author_email.clone(),
            });
        }

        if self.author_website.is_some() {
            root.children.push(Element{
                name: String::from("author_website"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: self.author_website.clone(),
            });
        }

        if self.authoring_tool.is_some() {
            root.children.push(Element{
                name: String::from("authoring_tool"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: self.authoring_tool.clone(),
            });
        }

        if self.comments.is_some() {
            root.children.push(Element{
                name: String::from("comments"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: self.comments.clone(),
            });
        }

        if self.copyright.is_some() {
            root.children.push(Element{
                name: String::from("copyright"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: self.copyright.clone(),
            });
        }

        if self.source_data.is_some() {
            root.children.push(Element{
                name: String::from("source_data"),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: self.source_data.clone(),
            });
        }

        return root;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::{Element};
    use traits::{XmlConversion};
    
    #[test]
    fn test_contributor_parse() {
        let data = r#"
            <contributor>
                <author>Bob the artist</author>
                <author_email>bob@bobartist.com</author_email>
                <author_website>http://www.bobartist.com</author_website>
                <authoring_tool>Super3DmodelMaker3000</authoring_tool>
                <comments>This is a big Tank</comments>
                <copyright>Bob's game shack: all rights reserved</copyright>
                <source_data>c:/models/tanks.s3d</source_data>
            </contributor>"#;
        let e = Element::parse(data.as_bytes()).unwrap();
        let mut c = Contributor::new();
        match c.parse(&e) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        assert_eq!(c.author.unwrap().as_str(), "Bob the artist");
        assert_eq!(c.author_email.unwrap().as_str(), "bob@bobartist.com");
        assert_eq!(c.author_website.unwrap().as_str(), "http://www.bobartist.com");
        assert_eq!(c.authoring_tool.unwrap().as_str(), "Super3DmodelMaker3000");
        assert_eq!(c.comments.unwrap().as_str(), "This is a big Tank");
        assert_eq!(c.copyright.unwrap().as_str(), "Bob's game shack: all rights reserved");
        assert_eq!(c.source_data.unwrap().as_str(), "c:/models/tanks.s3d");
    }

    #[test]
    fn test_contributor_partial_parse() {
        let data = r#"
            <contributor>
                <author>Master of disaster</author>
                <copyright>Disaster Dungeon (c) 2016</copyright>
            </contributor>"#;
        let e = Element::parse(data.as_bytes()).unwrap();
        let mut c = Contributor::new();
        match c.parse(&e) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        assert_eq!(c.author.unwrap().as_str(), "Master of disaster");
        assert_eq!(c.author_email, None);
        assert_eq!(c.author_website, None);
        assert_eq!(c.authoring_tool, None);
        assert_eq!(c.comments, None);
        assert_eq!(c.copyright.unwrap().as_str(), "Disaster Dungeon (c) 2016");
        assert_eq!(c.source_data, None);
    }
    
    #[test]
    fn test_contributor_encode() {
        let mut c = Contributor::new();
        c.author = Some("Bob the artist".to_owned());
        c.author_email = Some("bob@bobartist.com".to_owned());
        c.author_website = Some("http://www.bobartist.com".to_owned());
        c.authoring_tool = Some("Super3DmodelMaker3000".to_owned());
        c.comments = Some("This is a big tank".to_owned());
        c.copyright = Some("Bob's game shack: all rights reserved".to_owned());
        c.source_data = Some("c:/models/tanks.s3d".to_owned());
        
        let e = c.encode();
        assert_eq!(e.name, "contributor");
        for ch in &e.children {
            match ch.name.as_str() {
                "author" => assert_eq!(ch.text, Some("Bob the artist".to_owned())),
                "author_email" => assert_eq!(ch.text, Some("bob@bobartist.com".to_owned())),
                "author_website" => assert_eq!(ch.text, Some("http://www.bobartist.com".to_owned())),
                "authoring_tool" => assert_eq!(ch.text, Some("Super3DmodelMaker3000".to_owned())),
                "comments" => assert_eq!(ch.text, Some("This is a big tank".to_owned())),
                "copyright" => assert_eq!(ch.text, Some("Bob's game shack: all rights reserved".to_owned())),
                "source_data" => assert_eq!(ch.text, Some("c:/models/tanks.s3d".to_owned())),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_contributor_partial_encode() {
        let mut c = Contributor::new();
        c.author = Some("Master of disaster".to_owned());
        c.copyright = Some("Disaster Dungeon (c) 2016".to_owned());

        let e = c.encode();
        assert_eq!(e.name, "contributor");
        
        for ch in &e.children {
            match ch.name.as_str() {
                "author" => assert_eq!(ch.text, Some("Master of disaster".to_owned())),
                "author_email" => assert_eq!(ch.text, None),
                "autor_website" => assert_eq!(ch.text, None),
                "authoring_tool" => assert_eq!(ch.text, None),
                "commments" => assert_eq!(ch.text, None),
                "copyright" => assert_eq!(ch.text, Some("Disaster Dungeon (c) 2016".to_owned())),
                "source_data" => assert_eq!(ch.text, None),
                _ => assert!(false), 
            }
        }
    }
}
