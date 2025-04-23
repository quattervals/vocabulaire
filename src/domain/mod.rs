use actix_web::web;


/// Domain
pub mod voci;


/// Use Cases
pub mod create_translation;
pub mod read_translation;
pub mod update_translation;
pub mod delete_translation;




use crate::{Repository};
use crate::domain::voci::{TranslationRecord};

pub trait Entity {}



async fn does_translation_exist_by_name<'a, T: Repository<TranslationRecord>>(repository: &web::Data<T>, name: &str) -> bool {

    // let s = FindSandwich {
    //     id: None,
    //     name: String::from(name),
    //     ingredients: vec![]
    // };

    // repository.find_one(s).await.is_ok()
    //todo: implement
  false
}
