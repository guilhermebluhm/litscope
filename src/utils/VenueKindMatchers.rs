use crate::model::Publication::VenueKind::VenueKind;

pub fn venue_kind_matchers(kind: &str) -> VenueKind {

    match kind {

        "icse" | "ICSE" => {
            VenueKind::Conference
        }
        _ => VenueKind::Other
    }

}