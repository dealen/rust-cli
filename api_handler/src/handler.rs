use log::info;

pub struct Handler {
    pub url: String,
}

impl Handler {
    #[must_use]
    pub fn new(url: String) -> Handler {
        Handler { url }
    }

    /// # Errors
    /// 
    /// Returns a Error when there is some issue with the request
    pub fn get_resposnse(&self) -> Result<String, reqwest::Error> {
        info!("Sending request to {}", self.url);

        let body = reqwest::blocking::get(&self.url)?.text()?;

        info!("Response: {}", body);

        Ok(body)
    }
}
