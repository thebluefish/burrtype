use burrtype_derive::*;

#[derive(Burr)]
pub struct DeepTestStruct(u64);

mod bar {
    #[derive(burrtype_derive::Burr)]
    pub struct DeeperBarStruct {
        foo: u64,
        bar: u128,
    }
}