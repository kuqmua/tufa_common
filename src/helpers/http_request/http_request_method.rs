//not all, coz reqwest impl not all
pub enum HttpRequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

impl HttpRequestMethod {
    pub fn get_async_request_builder(
        &self,
        client: reqwest::Client,
        url: &str,
    ) -> reqwest::RequestBuilder {
        match self {
            HttpRequestMethod::Get => client.get(url),
            HttpRequestMethod::Post => client.post(url),
            HttpRequestMethod::Put => client.put(url),
            HttpRequestMethod::Patch => client.patch(url),
            HttpRequestMethod::Delete => client.delete(url),
            HttpRequestMethod::Head => client.head(url),
        }
    }
}

impl HttpRequestMethod {
    pub fn get_sync_request_builder(
        &self,
        client: reqwest::blocking::Client,
        url: &str,
    ) -> reqwest::blocking::RequestBuilder {
        match self {
            HttpRequestMethod::Get => client.get(url),
            HttpRequestMethod::Post => client.post(url),
            HttpRequestMethod::Put => client.put(url),
            HttpRequestMethod::Patch => client.patch(url),
            HttpRequestMethod::Delete => client.delete(url),
            HttpRequestMethod::Head => client.head(url),
        }
    }
}
