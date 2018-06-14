/*  Expected:

    browser::new()
        .homepage(ReqType::Join, "spp://localhost/")
        .homepage(ReqType::Query, "spp://localhost/query")
        .homepage(ReqType::OfflinePing, "spp://localhost/ping")
        .adapt_mc_bedrock("localhost:19132")
        .adapt_mc_java("localhost:25565") // only for example
        .unwrap();
*/

// fn new() -> Browser {}


enum ReqType {
    Join,
    Query,
    OfflinePing
}

struct Browser {
    homepage: HashMap<ReqType, URL>
}


