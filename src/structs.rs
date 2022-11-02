use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Item {
    pub name: String,
    pub tier: i32,
    pub skills: Vec<String>,
    pub lvl: i32,
    pub ids: Value,
    pub itemIDs: ItemIDs,
    pub consumableIDs: ConsumableIDs,
    pub posMods: PosMods,
    pub id: i32,
    pub displayName: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ItemIDs {
    pub dura: i32,
    pub strReq: i32,
    pub dexReq: i32,
    pub intReq: i32,
    pub defReq: i32,
    pub agiReq: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsumableIDs {
    pub charges: i32,
    pub dura: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct PosMods {
    pub left: i32,
    pub right: i32,
    pub above: i32,
    pub under: i32,
    pub touching: i32,
    pub notTouching: i32
}
impl PosMods {
    pub fn sum(&self) -> i32 {
        self.left + 
        self.right + 
        self.above + 
        self.under + 
        self.touching + 
        self.notTouching
    }
    pub fn has_some(&self) -> bool {
        self.left != 0 ||
        self.right != 0 ||
        self.above != 0 ||
        self.under != 0 ||
        self.touching != 0 ||
        self.notTouching != 0
    }
}