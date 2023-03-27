use std::io::Error;

use crawler::CrawlError;

const GOOGLE: &str = "https://www.google.com/search?q=";
pub struct GoogleDriver {}
impl GoogleDriver {
    pub fn new() -> GoogleDriver {
        GoogleDriver {}
    }

    fn get_pages(&self, html: &str) -> Result<Vec<crawler::Page>, Error> {

        let page = crawler::Page {
            url: "static_url".to_string(),
            surrounding_text: "static_surrounding_text".to_string(),
            full_text: "static_full_text".to_string(),
        };

        Ok(vec![page])
    }
}
impl crawler::CrawlDriver for GoogleDriver {
    fn crawl(&self, request: crawler::DriverRequest) -> Result<crawler::CrawlResponse, crawler::CrawlError> {
        let url = format!("{}{}", GOOGLE, request.terms.join("+"));

        let Ok(response_html) = crawler::get_html(&url) else {
          return Err(CrawlError{});  
        };

        let Ok(pages) = self.get_pages(&response_html) else {
            return Err(CrawlError{});
        };

        Ok(crawler::CrawlResponse {
            terms: request.terms.clone(),
            pages,
            results_page: response_html.to_string(),
        })
    }


    
}


