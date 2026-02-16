use accordion_core::routine::template::RoutineTemplate;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Default, Store, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StoredRoutines {
	pub vec_field: Vec<RoutineTemplate>,
}
