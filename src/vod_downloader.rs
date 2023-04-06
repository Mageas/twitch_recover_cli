use crate::TwitchRecoverResult;

#[derive(Debug)]
pub struct VodDownloader<'a> {
    url: &'a str,
    page: Vec<String>,
}

impl<'a> VodDownloader<'a> {
    pub async fn new(url: &'a str) -> TwitchRecoverResult<VodDownloader> {
        let response = reqwest::get(url).await?.text().await?;
        let page = Self::parse_m3u8_content(&response);

        Ok(VodDownloader { url, page })
    }

    pub fn get_m3u8_content(&self) {
        let urls = self.parse_m3u8_urls();
        dbg!(urls);
    }

    /// Retreive the urls from the m3u8 file
    fn parse_m3u8_urls(&self) -> Vec<&String> {
        self.page
            .iter()
            .filter(|line| !line.starts_with('#'))
            .collect::<Vec<&String>>()
    }

    /// Clean the m3u8 content
    fn parse_m3u8_content(text: &str) -> Vec<String> {
        text.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_owned())
            .collect::<Vec<String>>()
    }

    // pub async fn download(&self) -> TwitchRecoverResult {
    //     let client = reqwest::Client::new();
    //     let response = client.get(self.url).send().await?;

    //     let text = response.text().await?;

    //     let lines = Self::clean_text(&text);

    //     self.export_to_curl(lines);

    //     Ok(())
    // }

    // pub fn export_to_curl(&self, lines: Vec<&str>) {
    //     let url = self.url.replace("index-dvr.m3u8", "");

    //     let lines = lines
    //         .iter()
    //         .map(|line| format!("{}{}", url, line))
    //         .collect::<Vec<String>>();

    //     dbg!(lines);

    //     // for line in lines {
    //     //     output.push_str(&format!("{},", line));
    //     // }

    //     // output.pop();
    //     // output.push('}');
    //     // output.push('"');

    //     // println!("\n\n{}\n\n", output);
    // }
}
