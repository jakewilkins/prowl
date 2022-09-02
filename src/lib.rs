use std::fmt::Write;
use thiserror::Error;

const MAX_URL_LEN: usize = 512;
const MAX_APP_LEN: usize = 256;
const MAX_EVENT_LEN: usize = 1024;
const MAX_DESC_LEN: usize = 10000;

#[derive(Debug)]
pub struct Notification {
    api_keys: Vec<String>,
    priority: Option<Priority>,
    url: Option<String>,
    application: String,
    event: String,
    description: String,
}

#[derive(Debug)]
pub enum Priority {
    VeryLow,
    Moderate,
    Normal,
    High,
    Emergency,
}

#[derive(Debug, Error)]
pub enum AddError {
    #[error("The prowl API did not accept the request.")]
    Api(reqwest::Response),
    #[error("Failed to send notification to the prowl API. {0}")]
    Send(reqwest::Error),
    #[error("Failed to use format macro to build URL. {0}")]
    Format(std::fmt::Error),
}

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("Max URL length is {MAX_URL_LEN}, but provided {0}.")]
    InvalidUrlLength(usize),
    #[error("Max application length is {MAX_APP_LEN}, but provided {0}.")]
    ApplicationLength(usize),
    #[error("Max event length is {MAX_EVENT_LEN}, but provided {0}.")]
    EventLength(usize),
    #[error("Max description length is {MAX_DESC_LEN}, but provided {0}.")]
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
        url: Option<String>,
        application: String,
        event: String,
        description: String,
    ) -> Result<Self, CreationError> {
        if application.len() > MAX_APP_LEN {
            return Err(CreationError::ApplicationLength(application.len()));
        }

        if event.len() > MAX_EVENT_LEN {
            return Err(CreationError::EventLength(event.len()));
        }

        if description.len() > MAX_DESC_LEN {
            return Err(CreationError::DescriptionLength(description.len()));
        }

        if let Some(ref url) = url {
            if url.len() > MAX_URL_LEN {
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
        write!(url, "&application={safe_application}")?;
        write!(url, "&event={safe_event}")?;
        write!(url, "&description={safe_description}")?;

        if let Some(notification_url) = &self.url {
            let safe_notification_url = urlencoding::encode(notification_url);
            write!(url, "&url={safe_notification_url}")?;
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
