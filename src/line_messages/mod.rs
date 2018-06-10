use serde::ser::{Serialize, Serializer, SerializeStruct};

use line_actions::ImagemapAction;
use line_templates::TemplateComponent;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LineMessageType {
    Text{text: String},
    Image{original_content_url: String, preview_image_url: String },
    Sticker{package_id: String, sticker_id: String },
    Video{ original_content_url: String, preview_image_url: String },
    Audio{ original_content_url: String, duration: u32 },
    Location{ address: String, latitude: u32, longitude: String },
    Imagemap{ base_url: String, alt_text: String, base_size: u32, actions: Vec<ImagemapAction> },
    Template{ alt_text: String, template: TemplateComponent }
}

#[derive(Serialize, Deserialize)]
pub struct LineMessage {
    id:   String,
    #[serde(flatten)]
    kind: LineMessageType,
}

impl LineMessage {
    pub fn new(id: &str, kind: LineMessageType) -> LineMessage {
        LineMessage { id: String::from(id), kind }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }  
}

// impl Serialize for LineMessage {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // 3 is the number of fields in the struct.
//         let mut state = serializer.serialize_struct("LineMessage", 3)?;
//         state.serialize_field("id", &self.id)?;
//         state.serialize_field(&self.kind)?;
//         state.end()
//     }
// }
