pub struct DynamicFilter {
    name: String,
    // other options
}
impl Filter for DynamicFilter {
    fn is_matched(// ctx, http_request:Option<HttpRequest>, http_response:Option<HttpResponse>
    ) -> Result<
        bool,
        // TODO: replace std::error with Dakia Error
        Box<dyn std::error::Error>,
    > {
        todo!()
    }
}

pub struct StaticFilter {
    name: String,
    // other options
}

impl Filter for StaticFilter {
    fn is_matched(// ctx, http_request:Option<HttpRequest>, http_response:Option<HttpResponse>
    ) -> Result<
        bool,
        // TODO: replace std::error with Dakia Error
        Box<dyn std::error::Error>,
    > {
        todo!()
    }
}

trait Filter {
    fn is_matched(// ctx, http_request:Option<HttpRequest>, http_response:Option<HttpResponse>
    ) -> Result<
        bool,
        // TODO: replace std::error with Dakia Error
        Box<dyn std::error::Error>,
    >;
}
