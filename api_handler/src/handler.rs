use log::info;

pub struct Handler {
    pub url: String
}

impl Handler {
    pub fn new(url: String) -> Handler {
        Handler {
            url
        }
    }

    pub fn get_resposnse(&self) -> Result<String, reqwest::Error> {
        
        info!("Sending request to {}", self.url);

        let body = reqwest::blocking::get(&self.url)?.text()?;
        
        info!("Response: {}", body);

        Ok(body)
    }
}