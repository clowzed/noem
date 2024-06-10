use crate::{adapter::Adapter, unimail::Mail};
use reqwest::multipart::{Form, Part};
use std::convert::Infallible;

#[derive(Default)]
pub struct UnimailToMultipart;

impl Adapter for UnimailToMultipart {
    type Input = Mail;
    type Output = Form;
    type Error = Infallible;

    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut form = Form::new().text("uid", input.uid.to_string());

        if let Some(subject) = &input.subject {
            form = form.text("subject", subject.clone());
        }

        if let Some(text_body) = &input.text_body {
            form = form.text("text_body", text_body.clone());
        }

        if let Some(html_body) = &input.html_body {
            form = form.text("html_body", html_body.clone());
        }

        for (i, address) in input.from.iter().enumerate() {
            if let Some(name) = &address.name {
                form = form.text(format!("from[{}][name]", i), name.clone());
            }
            if let Some(address_str) = &address.address {
                form = form.text(format!("from[{}][address]", i), address_str.clone());
            }
            if let Some(group_name) = &address.group_name {
                form = form.text(format!("from[{}][group_name]", i), group_name.clone());
            }
        }

        for (i, address) in input.to.iter().enumerate() {
            if let Some(name) = &address.name {
                form = form.text(format!("to[{}][name]", i), name.clone());
            }
            if let Some(address_str) = &address.address {
                form = form.text(format!("to[{}][address]", i), address_str.clone());
            }
            if let Some(group_name) = &address.group_name {
                form = form.text(format!("to[{}][group_name]", i), group_name.clone());
            }
        }

        for (i, attachment) in input.attachments.iter().enumerate() {
            let part = Part::bytes(attachment.contents.clone()).file_name(attachment.name.clone());
            form = form.part(format!("attachment[{}]", i), part);
        }

        Ok(form)
    }
}
