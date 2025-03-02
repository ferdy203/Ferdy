use crate::{
    gateway::{
        interceptor::{Hook, HookMask, Interceptor, InterceptorName, Phase},
        interceptor_builder::InterceptorBuilder,
    },
    qe::query::Query,
};

pub struct Builder {
    hook_mask: Option<HookMask>,
    filter: Option<Query>,
    config: Option<Query>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            hook_mask: Default::default(),
            filter: Default::default(),
            config: Default::default(),
        }
    }
}

impl InterceptorBuilder for Builder {
    fn name(&self) -> &InterceptorName {
        &InterceptorName::Version
    }

    fn hook(&mut self, hook: Option<HookMask>) {
        self.hook_mask = hook;
    }

    fn filter(&mut self, filter: Option<Query>) {
        self.filter = filter;
    }

    fn config(&mut self, config: Option<Query>) {
        self.config = config;
    }

    fn build(&mut self) -> Box<dyn Interceptor> {
        todo!()
    }
}
