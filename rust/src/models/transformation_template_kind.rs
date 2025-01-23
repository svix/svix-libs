use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TransformationTemplateKind {
    #[default]
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "CustomerIO")]
    CustomerIo,
    #[serde(rename = "Discord")]
    Discord,
    #[serde(rename = "Hubspot")]
    Hubspot,
    #[serde(rename = "Inngest")]
    Inngest,
    #[serde(rename = "Salesforce")]
    Salesforce,
    #[serde(rename = "Segment")]
    Segment,
    #[serde(rename = "Slack")]
    Slack,
    #[serde(rename = "Teams")]
    Teams,
    #[serde(rename = "TriggerDev")]
    TriggerDev,
    #[serde(rename = "Windmill")]
    Windmill,
    #[serde(rename = "Zapier")]
    Zapier,
}

impl std::fmt::Display for TransformationTemplateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Custom => write!(f, "Custom"),
            Self::CustomerIo => write!(f, "CustomerIO"),
            Self::Discord => write!(f, "Discord"),
            Self::Hubspot => write!(f, "Hubspot"),
            Self::Inngest => write!(f, "Inngest"),
            Self::Salesforce => write!(f, "Salesforce"),
            Self::Segment => write!(f, "Segment"),
            Self::Slack => write!(f, "Slack"),
            Self::Teams => write!(f, "Teams"),
            Self::TriggerDev => write!(f, "TriggerDev"),
            Self::Windmill => write!(f, "Windmill"),
            Self::Zapier => write!(f, "Zapier"),
        }
    }
}
