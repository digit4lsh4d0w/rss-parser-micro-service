use url::Url;

#[derive(Debug, Clone)]
pub struct News {
    pub title: String,
    pub description: String,
    pub link: Url,
    pub permanent_link: Option<Url>,
    pub author_email: Option<String>,
    pub categories: Option<Vec<String>>,
    pub media: Option<String>,
    pub comments_url: Option<Url>,
    pub pub_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewsBuilder {
    title: String,
    description: String,
    link: Url,
    permanent_link: Option<Url>,
    author_email: Option<String>,
    categories: Option<Vec<String>>,
    media: Option<String>,
    comments_url: Option<Url>,
    pub_date: Option<String>,
}

impl NewsBuilder {
    pub fn new(title: String, description: String, link: Url) -> Self {
        NewsBuilder {
            title,
            description,
            link,
            permanent_link: None,
            author_email: None,
            categories: None,
            media: None,
            comments_url: None,
            pub_date: None,
        }
    }

    pub fn with_permanent_link(mut self, permanent_link: Url) -> Self {
        self.permanent_link = Some(permanent_link);
        self
    }

    pub fn with_author_email(mut self, author_email: String) -> Self {
        self.author_email = Some(author_email);
        self
    }

    pub fn with_categories(mut self, categories: Vec<String>) -> Self {
        self.categories = Some(categories);
        self
    }

    pub fn with_media(mut self, media: String) -> Self {
        self.media = Some(media);
        self
    }

    pub fn with_comments_url(mut self, comments_url: Url) -> Self {
        self.comments_url = Some(comments_url);
        self
    }

    pub fn with_pub_date(mut self, pub_date: String) -> Self {
        self.pub_date = Some(pub_date);
        self
    }

    pub fn build(self) -> News {
        News {
            title: self.title,
            description: self.description,
            link: self.link,
            permanent_link: self.permanent_link,
            author_email: self.author_email,
            categories: self.categories,
            media: self.media,
            comments_url: self.comments_url,
            pub_date: self.pub_date,
        }
    }
}
