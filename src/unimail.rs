use mail_parser::{Address as MailParserAddress, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub name: Option<String>,
    pub address: Option<String>,
    pub group_name: Option<String>,
}

impl Address {
    pub fn is_undefined(&self) -> bool {
        self.name.is_none() && self.address.is_none() && self.group_name.is_none()
    }
}

pub struct AddressVec(Vec<Address>);

#[derive(Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub contents: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Mail {
    pub uid: u32,
    pub from: Vec<Address>,
    pub to: Vec<Address>,
    pub subject: Option<String>,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
    pub attachments: Vec<Attachment>,
}

#[derive(thiserror::Error, Debug)]
pub enum MailParseError {
    #[error("No headers were found in the message")]
    NoHeaders,
}

impl From<&MailParserAddress<'_>> for AddressVec {
    fn from(value: &MailParserAddress<'_>) -> Self {
        AddressVec(match value {
            MailParserAddress::List(list) => list
                .iter()
                .map(|address| Address {
                    name: address.name().map(String::from),
                    address: address.address().map(String::from),
                    group_name: None,
                })
                .filter(|address| !address.is_undefined())
                .collect(),

            MailParserAddress::Group(groups) => groups
                .iter()
                .flat_map(|group| {
                    group.addresses.iter().map(|address| Address {
                        name: address.name().map(String::from),
                        address: address.address().map(String::from),
                        group_name: group.name.as_deref().map(String::from),
                    })
                })
                .collect(),
        })
    }
}

impl TryFrom<(u32, Vec<u8>)> for Mail {
    type Error = MailParseError;

    fn try_from(value: (u32, Vec<u8>)) -> Result<Self, Self::Error> {
        let uid = value.0;

        let parsed_mail = MessageParser::default()
            .parse(&value.1)
            .ok_or(MailParseError::NoHeaders)?;

        let to: Vec<Address> = match parsed_mail.to() {
            Some(to) => AddressVec::from(to).0,
            None => vec![],
        };

        let from: Vec<Address> = match parsed_mail.from() {
            Some(from) => AddressVec::from(from).0,
            None => vec![],
        };

        let subject = parsed_mail.subject().map(String::from);

        let text_body = parsed_mail.body_text(0).map(String::from);

        let html_body = parsed_mail.body_html(0).map(String::from);

        let attachments: Vec<_> = parsed_mail
            .attachments()
            .filter(|attachment| attachment.attachment_name().is_some())
            .map(|attachment| Attachment {
                name: attachment.attachment_name().unwrap().to_string(), // Safe as none handled line before
                contents: attachment.contents().to_vec(),
            })
            .collect();

        Ok(Mail {
            uid,
            from,
            to,
            subject,
            text_body,
            html_body,
            attachments,
        })
    }
}
