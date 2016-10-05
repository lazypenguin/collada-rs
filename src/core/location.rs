use std::fmt;
use std::collections::{HashMap};
use std::string::{String};
use xmltree::{Element};
use error::{ColladaError};
use traits::{XmlConversion};


/// Specifies wether the altitude is distance from sea level or distance 
/// relative to terrain height at that latitude+longitude 
#[derive(Copy, Clone, Debug)]
pub enum AltitudeMode {
    /// Altitude value should be interpreted as distance in meters relative to 
    /// ground level
    RelativeToGround,

    /// Altitude value should be interepreted as distance in meters relative 
    /// to sea level
    Absolute
}

impl fmt::Display for AltitudeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AltitudeMode::RelativeToGround => {write!(f, "relativeToGround")},
            &AltitudeMode::Absolute => {write!(f, "absolute")},
        }
    }
}

/// Geographic location for an asset
#[derive(Debug)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
    pub altitude: f32,
    pub mode: AltitudeMode,
}

impl Location {
    /// Create a new Location node
    pub fn new() -> Location {
        Location {
            longitude: 0.0,
            latitude: 0.0,
            altitude: 0.0,
            mode: AltitudeMode::RelativeToGround
        }
    }
}

impl XmlConversion for Location {
    fn parse(&mut self, e: &Element) -> Result<(), ColladaError> {
        if e.name != "coverage".to_owned() {
            return Err(ColladaError::MissingElement {
                structure: "location".to_string(),
                elem: "coverage".to_string(),
            });
        }
        
        if e.children.len() != 1 && 
            e.children[0].name != "geographic_location".to_owned() {
                return Err(ColladaError::MissingElement{
                    structure: "location".to_string(),
                    elem: "geographic_location".to_string(),
                });
            }
        
        let geo = &e.children[0];
        if geo.children.len() != 3 {
            return Err(ColladaError::Invalid{
                msg: "<geographic_location> element must have 3 children: \
                    <longitude>, <latitude>, <altitude>".to_string()
            });
        }
         
        for c in &geo.children {
            let text = match c.text.clone() {
                Some(t) => t,
                None => return Err(ColladaError::MissingData{
                            elem: c.name.clone()
                        }),
            };

            match c.name.as_str() {
                "longitude" => { 
                    self.longitude = text.parse::<f32>().unwrap();
                },
                "latitude" => {
                    self.latitude = text.parse::<f32>().unwrap();
                },
                "altitude" => {
                    self.altitude = text.parse::<f32>().unwrap();
                    self.mode = match c.attributes.get("mode") {
                        Some(m) => {
                            match m.as_str() {
                                "absolute" => AltitudeMode::Absolute,
                                "relativeToGround" => AltitudeMode::RelativeToGround,
                                _ => return Err(ColladaError::InvalidAttrData{
                                        elem: "altitude".to_string(),
                                        attr: "mode".to_string(),
                                        data: m.clone(),
                                    }),
                            }
                        },
                        None => return Err(ColladaError::MissingAttr{
                            elem: "altitude".to_string(),
                            attr: "mode".to_string(),
                        }),
                    }
                }
                _ => return Err(ColladaError::InvalidChild{
                        child: c.name.clone(),
                        parent: "geographic_location".to_string(),
                    }),
            }
        }
        Ok(())
    }

    fn encode(&self) -> Element {
        let long = Element {
            name: String::from("longitude"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(self.longitude.to_string()),
        };

        let lat = Element {
            name: String::from("latitude"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(self.latitude.to_string()),
        };
        
        let mut alt = Element {
            name: String::from("altitude"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some((self.altitude as i32).to_string()),
        };
        alt.attributes.insert(String::from("mode"), self.mode.to_string());
        
        let geo = Element {
            name: String::from("geographic_location"),
            attributes: HashMap::new(),
            children: vec![long, lat, alt],
            text: None,
        };

        Element {
            name: String::from("coverage"),
            attributes: HashMap::new(),
            children: vec![geo],
            text: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::{Element};
    use traits::{XmlConversion};
   
    #[test]
    fn test_location_parse() {
        let data = r#"
        <coverage>
            <geographic_location>
                <longitude>123.0</longitude>
                <latitude>-123.0</latitude>
                <altitude mode="absolute">50.0</altitude>
            </geographic_location>
        </coverage>"#;
        
        let e = Element::parse(data.as_bytes()).unwrap();
        let mut l = Location::new();
        match l.parse(&e) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        assert_eq!(l.longitude, 123.0);
        assert_eq!(l.latitude, -123.0);
        assert_eq!(l.altitude, 50.0);
        match l.mode {
            AltitudeMode::Absolute => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_location_encode() {
        let mut l = Location::new();
        l.longitude = -151.0;
        l.latitude = 151.0;
        l.altitude = 30.0;
        l.mode = AltitudeMode::Absolute;

        let e = l.encode();
        assert_eq!(e.name, "coverage");
        let g = &e.children[0];
        assert_eq!(g.name, "geographic_location");
        assert_eq!(g.children.len(), 3);
        assert_eq!(g.children[0].name, "longitude");
        assert_eq!(g.children[1].name, "latitude");
        
        let a = &g.children[2];
        assert_eq!(a.name, "altitude");
        assert_eq!(a.text, Some(String::from("30")));
        assert_eq!(a.attributes.contains_key("mode"), true);
        assert_eq!(a.attributes.get("mode"), Some(&String::from("absolute")));
    }
}
