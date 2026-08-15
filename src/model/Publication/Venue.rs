use crate::model::Publication::VenueKind::VenueKind;

#[derive(Debug)]
pub struct Venue{
    pub raw: String,
    pub kind: VenueKind
}