#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "schema")]
#[macro_use]
extern crate schemars;

#[macro_use]
extern crate async_trait;

macro_rules! common_derives {
    () => {
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        #[derive(Debug, Clone)]
    };
}

pub mod admin;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
