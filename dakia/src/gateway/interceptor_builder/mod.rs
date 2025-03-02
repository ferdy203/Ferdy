use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    gateway::interceptor::{HookMask, Interceptor, InterceptorName},
    qe::query::Query,
};

use super::interceptors::version;

pub trait InterceptorBuilder: Sync + Send {
    fn name(&self) -> &InterceptorName;
    fn hook(&mut self, hook_mask: Option<HookMask>);
    fn filter(&mut self, filter: Option<Query>);
    fn config(&mut self, config: Option<Query>);
    fn build(&mut self) -> Box<dyn Interceptor>;
}

#[derive(Clone)]
pub struct InterceptorBuilderRegistry {
    /*
    Arc<dyn InterceptorBuilder> used instead of Box<dyn InterceptorBuilder> because of the error the trait `InterceptorBuilder` cannot be made into an object `InterceptorBuilder` cannot be made into an objec
    - https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
    - https://www.reddit.com/r/rust/comments/7q3bz8/trait_object_with_clone/
    - https://stackoverflow.com/questions/64725210/how-to-make-a-trait-and-a-struct-implementing-it-clonable
    */
    // Mutex does not support Clone so wrapped in Arc
    pub registry: HashMap<InterceptorName, Arc<Mutex<Arc<dyn InterceptorBuilder>>>>,
}

impl InterceptorBuilderRegistry {
    pub fn build() -> Self {
        let mut registry: HashMap<InterceptorName, Arc<Mutex<Arc<dyn InterceptorBuilder>>>> =
            HashMap::new();

        registry.insert(
            InterceptorName::Version,
            Arc::new(Mutex::new(Arc::new(version::Builder::default()))),
        );

        Self { registry }
    }
}
