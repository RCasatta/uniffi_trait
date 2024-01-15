uniffi::setup_scaffolding!();

#[uniffi::export]
pub trait MyTrait: Send + Sync {
    fn my_trait(&self) -> String;
}

#[uniffi::export]
pub fn ciao(my_trait: std::sync::Arc<dyn MyTrait>) -> String {
    my_trait.my_trait()
}
