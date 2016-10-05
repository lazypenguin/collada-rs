use std::collections::{HashMap};
use std::fmt;
use xmltree::{Element};
use core::{Contributor, Extra, Location};
use error::{ColladaError};
use traits::{XmlConversion};

/// The distance units for the asset. 
#[derive(Debug)]
pub struct Unit {
    /// Name of the distance unit, does not have to be a real-world measurement
    pub name: Option<String>,

    /// How many real-world meters in one distance unit
    pub meter: Option<f32>,
}

impl Unit {
    /// Create a new unit with default values
    pub fn new() -> Unit {
        Unit {
            name: Some("meter".to_string()),
            meter: Some(1.0),
        }
    }
}

/// Which axis represents up for an asset in a right-handed coordinate system
///
/// | Value | Right Axis | Up Axis    | In Axis    |
/// | :---: | :--------: | :--------: | :--------: |
/// | X_UP  | Negative Y | Positive X | Positive Z |
/// | Y_UP  | Positive X | Positive Y | Positive Z |
/// | Z_UP  | Positive X | Positive Z | Negative Y |
#[derive(Debug)]
pub enum UpAxis {
    XUP,
    YUP,
    ZUP,
}

impl fmt::Display for UpAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &UpAxis::XUP => write!(f, "X_UP"),
            &UpAxis::YUP => write!(f, "Y_UP"),
            &UpAxis::ZUP => write!(f, "Z_UP"),
        }
    }
}

/// Asset information for parent element
#[derive(Debug)]
pub struct Asset {
    pub contributors: Vec<Contributor>,
    pub location: Option<Location>,
    pub created: String,
    pub keywords: Vec<String>,
    pub modified: String,
    pub revision: Option<String>,
    pub subject: Option<String>,
    pub title: Option<String>,
    pub unit: Option<Unit>,
    pub up_axis: Option<UpAxis>,
    pub extras: Vec<Extra>,
}

impl Asset {
    pub fn new() -> Asset {
        Asset {
            contributors: Vec::new(),
            location: None,
            created: String::from(""), 
            keywords: Vec::new(),
            modified: String::from(""),
            revision: None,
            subject: None,
            title: None,
            unit: None, 
            up_axis: None, 
            extras: Vec::new(), 
        }
    }
}

impl XmlConversion for Asset {
    fn parse(&mut self, e: &Element) -> Result<(), ColladaError> {
        for c in &e.children {
            match c.name.as_str() {
                "contributor" => {
                    let mut b = Contributor::new();
                    try!(b.parse(c));
                    self.contributors.push(b);
                    continue;
                },
                "coverage" => {
                    let mut l = Location::new();
                    try!(l.parse(c));
                    self.location = Some(l);
                    continue;
                },
                "extra" => {
                    let mut x = Extra::new();
                    try!(x.parse(c));
                    self.extras.push(x);
                    continue;
                },
                "unit" => {
                    let mut u = Unit::new();
                    u.name = match c.attributes.get("name") {
                        Some(n) => Some(n.clone()),
                        None => None,
                    };
                    u.meter = match c.attributes.get("meter") {
                        Some(m) => Some(m.parse::<f32>().unwrap()),
                        None => None,
                    };
                    self.unit = Some(u);
                    continue;
                },
                _ => {}, 
            }
            
            let t = match c.text {
                Some(ref t) => t,
                None => return Err(ColladaError::MissingData{
                    elem: c.name.clone(),
                }),
            };

            match c.name.as_str(){
                "created" => self.created = t.clone(),
                "keywords" => {
                    for kw in t.split_whitespace() {
                        self.keywords.push(kw.to_string());
                    }
                },
                "modified" => self.modified = t.clone(),
                "revision" => self.revision = Some(t.clone()),
                "subject" => self.subject = Some(t.clone()),
                "title" => self.title = Some(t.clone()),
                "up_axis" => {
                    match t.as_str() {
                        "X_UP" => self.up_axis = Some(UpAxis::XUP),
                        "Y_UP" => self.up_axis = Some(UpAxis::YUP),
                        "Z_UP" => self.up_axis = Some(UpAxis::ZUP),
                        _ => return Err(ColladaError::InvalidData{
                            elem: "up_axis".to_string(),
                            data: t.clone(),
                        }),
                    };
                },
                _ => return Err(ColladaError::InvalidChild{
                    child: c.name.clone(),
                    parent: "asset".to_string(),
                }),
            }
        }
        Ok(())
    }

    fn encode(&self) -> Element {
        let mut a = Element::new("asset");

        for con in &self.contributors {
            a.children.push(con.encode());
        }

        match self.location {
            Some(ref x) => a.children.push(x.encode()),
            None => {},
        }

        a.children.push(Element{
            name: "created".to_string(),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(self.created.clone()),
        });
        
        let mut kw_str = String::new();
        for kw in &self.keywords {
            kw_str.push_str(kw.as_str());
            kw_str.push(' ');
        }
        kw_str.pop(); // Remove extra blank

        a.children.push(Element{
            name: "keywords".to_string(),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(kw_str),
        });

        a.children.push(Element{
            name: "modified".to_string(),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(self.modified.clone()),
        });
      
        match self.revision {
            Some(ref x) => a.children.push(Element{
                name: "revision".to_string(),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: Some(x.clone()),
            }),
            None => {},
        }

        match self.subject {
            Some(ref x) => a.children.push(Element{
                name: "subject".to_string(),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: Some(x.clone()),
            }),
            None => {},
        }

        match self.title {
            Some(ref x) => a.children.push(Element{
                name: "title".to_string(),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: Some(x.clone()),
            }),
            None => {},
        }

        match self.unit {
            Some(ref x) => {
                let mut u = Element {
                    name: "unit".to_string(),
                    attributes: HashMap::new(),
                    children: Vec::new(),
                    text: None,
                };
                match x.name {
                    Some(ref n) => {
                        u.attributes.insert("name".to_string(), n.clone());
                    },
                    None => {},
                };
                match x.meter {
                    Some(ref m) => {
                        u.attributes.insert("meter".to_string(), format!("{}", m)); 
                    },
                    None => {},
                };
                a.children.push(u);
            },
            None => {},
        }

        match self.up_axis {
            Some(ref axis) => a.children.push(Element {
                name: "up_axis".to_string(),
                attributes: HashMap::new(),
                children: Vec::new(),
                text: Some(format!("{}", axis)),
            }),
            None => {},
        }
        
        for ext in &self.extras {
            a.children.push(ext.encode());
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::{Element};
    use core::{AltitudeMode, Contributor, Extra, Location};
    use traits::{XmlConversion};

    #[test]
    fn test_asset_parse() {
        let data = r#"
            <asset>
                <contributor>
                    <author>John Smith</author>
                </contributor>
                <coverage>
                    <geographic_location>
                        <longitude>50.0</longitude>
                        <latitude>50.0</latitude>
                        <altitude mode="absolute">50.0</altitude>
                    </geographic_location>
                </coverage>
                <created>2008-01-28T20:51:36Z</created>
                <keywords>foo bar baz</keywords>
                <modified>2008-01-28T20:51:36Z</modified>
                <revision>rev_v5</revision>
                <title>my sweet asset</title>
                <unit meter="1.33" name="meter" />
                <up_axis>Z_UP</up_axis>
                <extra>
                    <technique profile="foo">
                        <foo>bar</foo>
                    </technique>
                </extra>
            </asset>"#;
        let e = Element::parse(data.as_bytes()).unwrap();
        let mut a = Asset::new();
        match a.parse(&e) {
            Ok(_) => assert!(true),
            Err(e) => { println!("{}", e); assert!(false) },
        }

        assert_eq!(a.contributors.len(), 1);
        assert_eq!(a.contributors[0].author, Some("John Smith".to_string()));
        let l = a.location.unwrap();
        assert_eq!(l.longitude, 50.0);
        assert_eq!(l.latitude, 50.0);
        assert_eq!(l.altitude, 50.0);
        match l.mode {
            AltitudeMode::Absolute => assert!(true),
            _ => assert!(false),
        };
        assert_eq!(a.created.as_str(), "2008-01-28T20:51:36Z");
        assert_eq!(a.keywords.len(), 3);
        assert_eq!(a.keywords[0], "foo".to_string());
        assert_eq!(a.keywords[1], "bar".to_string());
        assert_eq!(a.keywords[2], "baz".to_string());
        assert_eq!(a.modified.as_str(), "2008-01-28T20:51:36Z");
        assert_eq!(a.revision.unwrap().as_str(), "rev_v5");
        assert_eq!(a.title.unwrap().as_str(), "my sweet asset");
        let u = a.unit.unwrap();
        assert_eq!(u.name, Some("meter".to_string()));
        assert_eq!(u.meter, Some(1.33));
        match a.up_axis.unwrap() {
            UpAxis::ZUP => assert!(true),
            _ => assert!(false),
        }
        assert_eq!(a.extras.len(), 1);
        assert_eq!(a.extras[0].techniques.len(), 1);
        assert_eq!(a.extras[0].techniques[0].profile.as_str(), "foo");
        assert_eq!(a.extras[0].techniques[0].data.name.as_str(), "technique");
    }

    #[test]
    fn test_asset_encode() {
        let mut asset = Asset::new();
        asset.contributors.push(Contributor::new());
        asset.contributors[0].author = Some("John Smith".to_string());

        let mut location = Location::new();
        location.longitude = 50.0;
        location.latitude = 50.0;
        location.altitude = 50.0;
        asset.location = Some(location);

        asset.created = "2008-01-28T20:51:36Z".to_string();
        asset.keywords = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        asset.modified = "2008-01-28T20:51:36Z".to_string();
        asset.revision = Some("rev_v5".to_string());
        asset.title = Some("my sweet asset".to_string());
        
        let mut unit = Unit::new();
        unit.name = Some("meter".to_string());
        unit.meter = Some(1.33);
        asset.unit = Some(unit);

        asset.up_axis = Some(UpAxis::ZUP);
        asset.extras = vec![Extra::new()];

       let e = asset.encode();
       assert_eq!(e.name, "asset");
       assert_eq!(e.children[0].name, "contributor");
       assert_eq!(e.children[1].name, "coverage");
       assert_eq!(e.children[2].name, "created");
       assert_eq!(e.children[2].text, Some("2008-01-28T20:51:36Z".to_string()));
       assert_eq!(e.children[3].name, "keywords");
       assert_eq!(e.children[3].text, Some("foo bar baz".to_string()));
       assert_eq!(e.children[4].name, "modified");
       assert_eq!(e.children[4].text, Some("2008-01-28T20:51:36Z".to_string()));
       assert_eq!(e.children[5].name, "revision");
       assert_eq!(e.children[5].text, Some("rev_v5".to_string()));
       assert_eq!(e.children[6].name, "title");
       assert_eq!(e.children[6].text, Some("my sweet asset".to_string()));
       assert_eq!(e.children[7].name, "unit");
       assert_eq!(e.children[7].attributes.get("name"), Some(&"meter".to_string()));
       assert_eq!(e.children[7].attributes.get("meter"), Some(&"1.33".to_string()));
       assert_eq!(e.children[8].name, "up_axis");
       assert_eq!(e.children[8].text, Some("Z_UP".to_string()));
       assert_eq!(e.children[9].name, "extra");
    }
}
