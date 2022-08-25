use std::fmt::Write;

#[derive(Debug)]
pub struct Notification {
    api_keys: Vec<String>,
    priority: Option<Priority>,
    url: Option<String>, // max 512
    application: String, // max 256
    event: String,       // max 1024
    description: String, // max 10000
}

#[derive(Debug)]
pub enum Priority {
    VeryLow,
    Moderate,
    Normal,
    High,
    Emergency,
}

#[derive(Debug)]
pub enum AddError {
    Api(reqwest::Response),
    Send(reqwest::Error),
    Format(std::fmt::Error),
}

#[derive(Debug)]
pub enum CreationError {
    InvalidUrlLength(usize),
    ApplicationLength(usize),
    EventLength(usize),
    DescriptionLength(usize),
}

impl Priority {
    fn as_i8(&self) -> i8 {
        match self {
            Priority::VeryLow => -2,
            Priority::Moderate => -1,
            Priority::Normal => 0,
            Priority::High => 1,
            Priority::Emergency => 2,
        }
    }
}

impl Notification {
    pub fn new(
        api_keys: Vec<String>,
        priority: Option<Priority>,
        url: Option<String>, // max 512
        application: String, // max 256
        event: String,       // max 1024
        description: String, // max 10000
    ) -> Result<Self, CreationError> {
        if application.len() > 256 {
            return Err(CreationError::ApplicationLength(application.len()));
        }

        if event.len() > 1024 {
            return Err(CreationError::EventLength(event.len()));
        }

        if description.len() > 10000 {
            return Err(CreationError::DescriptionLength(description.len()));
        }

        if let Some(ref url) = url {
            if url.len() > 512 {
                return Err(CreationError::InvalidUrlLength(url.len()));
            }
        }

        Ok(Self {
            api_keys,
            priority,
            url,
            application,
            event,
            description,
        })
    }

    pub async fn add(&self) -> Result<(), AddError> {
        let safe_application = urlencoding::encode(&self.application);
        let safe_event = urlencoding::encode(&self.event);
        let safe_description = urlencoding::encode(&self.description);

        let mut url: String = "https://prowl.weks.net/publicapi/add".to_string();
        write!(url, "?apikey={}", self.api_keys.join(","))?;
        write!(url, "&application={}", safe_application)?;
        write!(url, "&event={}", safe_event)?;
        write!(url, "&description={}", safe_description)?;

        if let Some(notification_url) = &self.url {
            let safe_notification_url = urlencoding::encode(notification_url);
            write!(url, "&url={}", safe_notification_url)?;
        }

        if let Some(priority) = &self.priority {
            write!(url, "&priority={}", priority.as_i8())?;
        }

        log::trace!("Built URL {}", url);

        let client = reqwest::Client::new();
        let res = client.post(url).send().await?;
        if res.status() != reqwest::StatusCode::OK {
            log::error!("Failed to add notification, {:?}", res);
            Err(AddError::Api(res))
        } else {
            Ok(())
        }
    }
}

impl From<std::fmt::Error> for AddError {
    fn from(error: std::fmt::Error) -> Self {
        AddError::Format(error)
    }
}

impl From<reqwest::Error> for AddError {
    fn from(error: reqwest::Error) -> Self {
        AddError::Send(error)
    }
}
