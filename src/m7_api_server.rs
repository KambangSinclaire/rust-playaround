#[cfg(test)]
mod test {
    // use super::*;
    use std::io::{Error,ErrorKind};
    // FIRST APPROACH without explicit error handling
    // pub async fn api_call(url: &str)->Result<serde_json::Value, reqwest::Error> {
    //     let response:serde_json::Value = reqwest::get(url)
    //     .await?
    //     .json::<serde_json::Value>()
    //     .await?;
    //     Ok(response)
    // }
    
    
    // SECOND APPROACH with explicit error handling
    pub async fn api_call(url: &str)->Result<serde_json::Value, Error> {
        
        let response = reqwest::get(url).await.map_err(|_|Error::new(ErrorKind::Other,"Could not retrieve response"))?;
    
        let json_response = response.json::<serde_json::Value>().await.map_err(|_|Error::new(ErrorKind::Other,"Could not convert response to JSON"))?;
    
        Ok(json_response)
    }

    #[tokio::test]
    async fn test_api_calls() {
        let endpoint = "https://jsonplaceholder.typicode.com/posts/1";
        let api_response = api_call(endpoint).await;

        match api_response {
            Ok(res)=> dbg!(res),
            Err(_)=> panic!("Failed while trying to make api Request")
        };
    }
}
