use crate::util::util;

pub struct Posts {
    pub posts: json::JsonValue
}

pub struct Tags {
    pub tags: json::JsonValue
}

impl Posts {
    pub fn get_random_post(&self) -> Option<String> {
        if self.posts.len() > 0 {
            let num = util::get_rand_num(0, self.posts.len());
            return if !self.posts[num]["large_file_url"].is_empty() {
                Some(self.posts[num]["large_file_url"].to_string())
            } else if !self.posts[num]["file_url"].is_empty() {
                Some(self.posts[num]["file_url"].to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Tags {
    pub fn get_random_tag(&self) -> String {
        let num = util::get_rand_num(0, self.tags.len());
        self.tags[num]["name"].to_string()
    }
}
