use lettre::message::header::ContentType;
use lettre::transport::smtp::response::Response;
use lettre::{SmtpTransport, Transport};
use lettre::{message::MessageBuilder, transport::smtp::authentication::Credentials};
use std::error::Error;
use tera::{Context, Tera};

pub struct HtmlMailer {
    pub tempalte_engine: Tera,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl HtmlMailer {
    pub fn send(
        self,
        to: String,
        template_name: &str,
        template_context: &Context,
    ) -> Result<Response, Box<dyn Error>> {
        let html_body = self
            .tempalte_engine
            .render(template_name, template_context)?;

        let message = MessageBuilder::new()
            .subject("Cr8s digest")
            .from("Cr8s <noreply@cr8s.com>".parse()?)
            .to(to.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let credentials = Credentials::new(self.smtp_username, self.smtp_password);
        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(credentials)
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}
