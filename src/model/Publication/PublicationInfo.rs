use std::time::Instant;
use crate::model::Dblp::DblpHitNestedContent::DblpHitNestedContent;
use crate::model::Dblp::DblpInfo::DblpInfo;
use crate::model::Dblp::DblpResult::DblpResult;
use crate::model::Publication::Author::Author;
use crate::model::Publication::Doi::Doi;
use crate::model::Publication::Provenace::Provenance;
use crate::model::Publication::PubKey::PubKey;
use crate::model::Publication::PubKind::PubKind;
use crate::model::Publication::SourceId::SourceId;
use crate::model::Publication::Venue::Venue;
use crate::utils::CheckValidDOI::check_valid;
use crate::utils::PubKindMatchers::pub_kind_matchers;
use crate::utils::RemoveDOIPrefix::remove_prefix;
use crate::utils::VenueKindMatchers::venue_kind_matchers;
use crate::wrapper::AuthorWrapper::AuthorWrapper;
use crate::wrapper::HitWrapper::HitWrapper;

#[derive(Debug, Default)]
pub struct PublicationInfo {
    pub key: PubKey,
    pub title: String,
    pub authors: Vec<Author>,
    pub year: Option<u16>,
    pub venue: Option<Venue>,
    pub doi: Option<Doi>,
    pub kind: PubKind,
    pub abstract_text: Option<String>,
    pub provenance: Vec<Provenance>,
}

pub trait Rules{
    fn normalize_and_produce_publication(type_from: DblpResult, data_query: &str) -> Result<Vec<PublicationInfo>, String>;
}

impl Rules for PublicationInfo{
    fn normalize_and_produce_publication(type_from: DblpResult, data_query: &str) -> Result<Vec<PublicationInfo>, String> {

        let mut pub_vec:Vec<PublicationInfo> = Vec::new();

        if let Some(hit) = type_from.result.hits.hit{

            match hit {
                HitWrapper::TIPO_SIMPLES(x) => {
                    let mut r#pub = PublicationInfo::default();

                    r#pub.title = x.info.title.clone();
                    r#pub.kind = pub_kind_matchers(x.info.type_pub_info.as_str());
                    r#pub.year = Some(x.info.year.trim().parse::<u16>().unwrap());

                    let flag = check_valid_doi(&mut r#pub, &x, &x.info.doi);
                    if !flag{
                        let is_arvix = x.info.key.split_once("/").unwrap().0.to_string();
                        define_doi_and_key(&is_arvix, &mut r#pub);
                    }

                    retrieve_data_for_mount_author(&x.info, &mut r#pub);
                    define_vanue_value(!x.info.venue.is_empty(), &mut r#pub, &x.info.venue);
                    r#pub.abstract_text = None;
                    pub_vec.push(r#pub);
                }
                HitWrapper::TIPO_COMPOSTO(x) => {
                    for i in x{
                        let mut r#pub = PublicationInfo::default();

                        r#pub.title = i.info.title.clone();
                        r#pub.kind = pub_kind_matchers(i.info.type_pub_info.as_str());
                        r#pub.year = Some(i.info.year.trim().parse::<u16>().unwrap());
                        let flag = check_valid_doi(&mut r#pub, &i, &i.info.doi);

                        if !flag{
                            if !i.info.key.is_empty(){
                                let is_arvix = i.info.key.split_once("/").unwrap().0.to_string();
                                define_doi_and_key(&is_arvix, &mut r#pub);
                            }
                        }

                        retrieve_data_for_mount_author(&i.info, &mut r#pub);
                        define_vanue_value(!i.info.venue.is_empty(), &mut r#pub, &i.info.venue);
                        r#pub.abstract_text = None;
                        pub_vec.push(r#pub);

                    }
                }
            }
        }
        else{
            return Err("Hit objet not found or not mount correctly".to_string())
        }
        
        let length_pub = pub_vec.len()-1;
        let publ = pub_vec.get_mut(length_pub).unwrap();

        publ.provenance.push(Provenance{
            source: SourceId::Dblp,
            fetched_at: Instant::now(),
            query: data_query.to_string(),
        });

        Ok(pub_vec)
    }
}

fn retrieve_data_for_mount_author(infovalue: &DblpInfo, publdata: &mut PublicationInfo) {

    if let Some(at) = infovalue.authors.author.clone(){
        match at {
            AuthorWrapper::TIPO_SIMPLES(at_s) => {
                clean_and_put_author_data(at_s, publdata);
            }
            AuthorWrapper::TIPO_COMPOSTO(at_c) => {
                for at in at_c{
                    clean_and_put_author_data(at, publdata);
                }
            }
        }
    }

}

fn clean_and_put_author_data(author: crate::model::Dblp::Author::Author, publ: &mut PublicationInfo) {

    let d = author.text.split(" ").collect::<Vec<&str>>();
    let e = d.get(d.len() - 1).unwrap();
    let mut d_aux:Vec<&str> = d.clone();

    let _ = d_aux.retain(|x| !x.eq_ignore_ascii_case(e));
    let content = d_aux.join(" ");

    let aut = Author{
        fullName: author.text.clone(),
        family: e.to_string(),
        given: Some(content),
        sourceId: Some(author.pid)
    };

    publ.authors.push(aut);

}

fn define_vanue_value(is_not_empty: bool, publ: &mut PublicationInfo, value: &str) -> () {
    if is_not_empty == true {
        publ.venue = Some(Venue{
            kind: venue_kind_matchers(value),
            raw: value.to_string(),
        })
    }
}

fn define_doi_and_key(key: &str, publ: &mut PublicationInfo) -> () {
    if key.contains("journals"){
        publ.doi = Some(Doi::new("".to_string()));
        publ.key = PubKey::ArxivId(key.to_string());
    }
    else{
        publ.doi = Some(Doi::new("".to_string()));
        publ.key = PubKey::Fingerprint("".to_string());
    }
}

fn check_valid_doi(publ: &mut PublicationInfo, dblp_hit: &DblpHitNestedContent, doi_check: &String) -> bool {
    if check_valid(doi_check){

        let doi = remove_prefix(&dblp_hit.info.doi);
        if !doi.is_empty(){
            r#publ.doi = Some(Doi::new(doi.clone()));
            r#publ.key = PubKey::Doi(Doi::new(doi));
        }
        else{
            return false;
        }
        return true
    }
    false
}