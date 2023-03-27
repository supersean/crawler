pub struct CrawlRequest {
    pub terms: Vec<String>,
    pub drivers: Vec<Box<dyn CrawlDriver>>,
}
impl CrawlRequest {
    pub fn new(terms: Vec<String>, drivers: Vec<Box<dyn CrawlDriver>>) -> Self {
        CrawlRequest { terms, drivers }
    }
    pub fn crawl(&self) -> Vec<Result<CrawlResponse, CrawlError>> {
        self.drivers
            .iter()
            .map(|driver| driver.crawl(DriverRequest { terms: &self.terms }))
            .collect::<Vec<Result<CrawlResponse, CrawlError>>>()
    }
}
#[derive(Debug)]
pub struct CrawlResponse {
    pub terms: Vec<String>,
    pub pages: Vec<Page>,
    pub results_page: String,
}
#[derive(Debug)]
pub struct Page {
    pub url: String,
    pub surrounding_text: String,
    pub full_text: String,
}

pub trait CrawlDriver {
    fn crawl(&self, request: DriverRequest) -> Result<CrawlResponse, CrawlError>;
}
pub struct DriverRequest<'a> {
    pub terms: &'a Vec<String>,
}

#[derive(Debug)]
pub struct CrawlError {}

#[tokio::main]
pub async fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url).await?.text().await?)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
