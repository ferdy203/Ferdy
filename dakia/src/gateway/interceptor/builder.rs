use crate::qe::query::Query;

use super::{hook::Hook, interceptor::Interceptor, phase::Phase};

pub trait InterceptorBuilder {
    fn name(&mut self, name: String);
    fn phase(&mut self, phase: Phase);
    fn hook(&mut self, hook: Hook);
    fn filter(&mut self, filter: Option<Query>);
    fn config(&mut self, config: Option<Query>);
    fn build(&mut self) -> impl Interceptor;
}
