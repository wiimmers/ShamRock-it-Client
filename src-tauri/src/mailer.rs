#[macro_use]
pub mod mailer {
    use log::error;
    use tauri::Window;

    use crate::info::info::{get_creds, get_datetime, get_pc_name};
    use crate::structures::structures::{EmailMessage, FormData};
    use lettre::{
        message::header::ContentType,
        transport::smtp::authentication::Credentials,
        {Message, SmtpTransport, Transport},
    };
    // Function to handle sending tickets in email form with formatted EmailMessage object
    #[tauri::command]
    pub async fn email_ticket(form_data: FormData, window: Window) {
        // Emit window event, sendingMail with value 0 to mark start of send ticket
        window
            .emit("sendingMail", 0)
            .expect("error emitting sendingMail 0");
        // Create ticket object with format_message function from form data
        let ticket_object = format_message(form_data);
        // Initialize subject variable to format subject of email
        let subject = ticket_object.subject;
        // Initialize title variable to format body of email
        let title =
            format!("<h2 style='text-align:center;font-weight:bold;font-size:20px;'>Ticket</h2>");
        // Initialize heading variable to format body of email
        let heading = format!(
            "Ticket created by {} at {}, Contact: {}, PC Name: {}",
            ticket_object.name, ticket_object.location, ticket_object.contact, ticket_object.pcname
        );
        // Initialize msg variable to format body of email
        let msg = ticket_object.details;

        // Email builder
        let email = Message::builder()
            .from("<maintenance@jakesweeney.com>".parse().unwrap())
            .to("<ithelp@jakesweeney.com>".parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(format!(
                "<div style='text-align:center'>
                    {}
                </div>
                <div style='text-align:center'>
                    {}
                </div>
                <br>
                <h3>Ticket Details</h3>
                <br>
                {}",
                title, heading, msg
            ))
            .unwrap();

        // Initialize creds variable to hold credentials for logging in to email server
        let creds = get_creds().await.expect("Failed to get credentials from server");
        let username_password = Credentials::new(
            creds.email.username.to_owned(),
            creds.email.password.to_owned(),
        );
        let relay = creds.email.relay;
        // Initialze mailer variable to enter credentials and name SMTP relay URL
        let mailer = SmtpTransport::relay(&relay)
            .unwrap()
            .credentials(username_password)
            .build();
        // Send email variable in mailer and handle errors
        match mailer.send(&email) {
            Ok(_) => {
                // Print email sent successfully to backend
                println!("email sent successfully");
                // Emit window event, sending mail with value 1 to mark successful ticket send
                window
                    .emit("sendingMail", 1)
                    .expect("error emitting sendingMail 1");
            }
            Err(e) => {
                error!("{:?}", e)
            }
        }
    }
    // Format message using JavaScript object form_data, converted to string structure EmailMessage type
    fn format_message(form_data: FormData) -> EmailMessage {
        // Initialize new EmailMessage object with new strings
        let mut message_object = EmailMessage {
            name: String::new(),
            subject: String::new(),
            contact: String::new(),
            location: String::new(),
            details: String::new(),
            pcname: get_pc_name(),
        };
        // Fill EmailMessage structure formatted with form data
        message_object.name = format!(
            "{} {}",
            form_data.first_name.unwrap(),
            form_data.last_name.unwrap()
        );
        message_object.subject = format!(
            "{} Ticket Created: {}",
            form_data.ticket_type.unwrap(),
            get_datetime()
        );
        message_object.contact = format!(
            "<a href='mailto:{}'>{}</a>, ext. {}",
            form_data.email.clone().unwrap(),
            form_data.email.clone().unwrap(),
            form_data.telephone.unwrap()
        );
        message_object.location = format!(
            "{} {}",
            form_data.locale.unwrap(),
            form_data.dealer.unwrap_or_else(|| format!("No dealer selected"))
        );
        message_object.details = format!("{}", form_data.request.unwrap());

        message_object
    }
}
