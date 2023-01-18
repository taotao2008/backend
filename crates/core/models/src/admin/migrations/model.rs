/// Document representing migration information
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct MigrationInfo {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: i32,
    /// Current database revision
    pub revision: i32,
}

impl MigrationInfo {
    pub fn method_on_struct_itself(&self) {
        println!("hello");
    }
}
