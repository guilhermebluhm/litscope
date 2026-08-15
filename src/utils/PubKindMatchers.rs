use crate::model::Publication::PubKind::PubKind;
use crate::model::Publication::VenueKind::VenueKind;

pub fn pub_kind_matchers(type_pub: &str) -> PubKind {

    match type_pub {

        "Conference and Workshop Papers" => {
            PubKind::Conference
        }
        "Journal Articles" => {
            PubKind::Journal
        }
        "Informal and Other Publications" => {
            PubKind::Preprint
        }
        "Parts in Books or Collections" => {
            PubKind::Bookchapter
        }
        "Books and Theses" => {
            PubKind::Thesis
        }
        _ => PubKind::Other

    }

}