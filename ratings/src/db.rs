use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Rating {
    pub product_id: u32,
    pub reviewer: String,
    pub rating: u8,
}

lazy_static! {
    pub static ref RATINGS: Arc<RwLock<Vec<Rating>>> = {
        let init = vec![
            Rating {
                product_id: 0,
                reviewer: "Reviewer1".to_string(),
                rating: 5,
            },
            Rating {
                product_id: 0,
                reviewer: "Reviewer2".to_string(),
                rating: 4,
            },
        ];
        Arc::new(RwLock::new(init))
    };
}
