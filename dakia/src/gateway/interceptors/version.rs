use crate::{
    gateway::interceptor::{Hook, Interceptor, InterceptorBuilder, Phase},
    qe::query::Query,
};

struct VersionInterceptorBuilder {
    name: String,
    phase: Phase,
    hook: Hook,
    filter: Option<Query>,
    config: Option<Query>,
}

impl InterceptorBuilder for VersionInterceptorBuilder {
    fn name(&mut self, name: String) {
        self.name = name;
    }

    fn phase(&mut self, phase: Phase) {
        self.phase = phase;
    }

    fn hook(&mut self, hook: Hook) {
        self.hook = hook;
    }

    fn filter(&mut self, filter: Option<Query>) {
        self.filter = filter;
    }

    fn config(&mut self, config: Option<Query>) {
        self.config = config;
    }

    fn build(&mut self) -> impl Interceptor {
        todo!()
    }
    //
}
