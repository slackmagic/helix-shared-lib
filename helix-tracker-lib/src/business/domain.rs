use crate::storage::traits::*;
use std::boxed::Box;

pub struct TrackerDomain<I> {
    item_storage: Box<dyn ItemStorageTrait<I>>,
    //log_storage: Box<dyn LogStorageTrait<L>>,
}

impl<I> TrackerDomain<I> {
    pub fn new(
        item_storage: Box<dyn ItemStorageTrait<I>>,
        //log_storage: Box<dyn LogStorageTrait<L>>,
    ) -> Self {
        TrackerDomain {
            item_storage,
            //log_storage,
        }
    }
}
