use crate::foundation::server_error::ServerError;
use actix_web::HttpResponse;
use actix_web_validator::Validate;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::clone::Clone;
use std::sync::RwLock;
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct Cat {
    #[validate(length(
        min = 1,
        max = 8,
        message = "Must be grater than or equal 1 and less than or equal 8."
    ))]
    id: String,
    #[validate(length(
        min = 1,
        max = 8,
        message = "Must be grater than or equal 1 and less than or equal 8."
    ))]
    name: String,
    #[validate(length(
        min = 1,
        max = 200,
        message = "Must be grater than or equal 1 and less than or equal 200."
    ))]
    description: String,
    #[validate(
        length(
            min = 1,
            max = 200,
            message = "Must be grater than or equal 1 and less than or equal 200."
        ),
        url(message = "Not a valid url.")
    )]
    img_url: String,
    #[validate(
        length(
            min = 1,
            max = 200,
            message = "Must be grater than or equal 1 and less than or equal 200."
        ),
        url(message = "Not a valid url.")
    )]
    reference: String,
}

impl Cat {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        img_url: impl Into<String>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            img_url: img_url.into(),
            reference: reference.into(),
        }
    }
}

lazy_static! {
    static ref CAT_LIST: RwLock<Vec<Cat>> = {
        let mut vec = Vec::new();
        vec.push(Cat::new(
            "C001",
            "Casper",
            "Casper (c. 1997 – 14 January 2010) was a male domestic cat who attracted worldwide media attention in 2009 when it was reported that he was a regular bus commuter in Plymouth in Devon, England.",
            "https://upload.wikimedia.org/wikipedia/en/e/e1/Casper_the_cat%2C_sitting_in_bus.jpg",
            "https://en.wikipedia.org/wiki/Casper_(cat)"
        ));
        vec.push(Cat::new(
            "C002",
            "Dewey",
            "Dewey Readmore Books (November 18, 1987 – November 29, 2006) was the library cat of the Spencer, Iowa, Public Library. Having been abandoned in the library's drop box in January 1988, he was adopted by the library.",
            "https://upload.wikimedia.org/wikipedia/en/7/74/Dewey_Readmore_Books.jpg",
            "https://en.wikipedia.org/wiki/Dewey_Readmore_Books"
        ));
        RwLock::new(vec)
    };
}

#[actix_web::get("/cat")]
pub async fn endpoint_get() -> Result<HttpResponse, ServerError> {
    let list = CAT_LIST.read()?;

    Ok(HttpResponse::Ok().json(list.clone()))
}

#[actix_web::put("/cat")]
pub async fn endpoint_put(
    body: actix_web_validator::Json<Cat>,
) -> Result<HttpResponse, ServerError> {
    let new_cat = body.clone();
    let mut list = CAT_LIST.write()?;

    // validate id
    let same_id_cat = list.iter().find(|v| v.id == new_cat.id);
    if same_id_cat.is_some() {
        return Err(ServerError::new_bad_request(
            "id",
            format!("This ID '{}' has already been used.", new_cat.id),
        ));
    }

    list.push(new_cat);
    Ok(HttpResponse::Ok().json(list.clone()))
}
