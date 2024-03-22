pub mod structures {
    use serde::{Deserialize, Serialize};

    // Structure to import credentials from credentials.toml
    // Each field corresponds to a table within said toml 
    #[derive(Debug, Deserialize)]
    pub struct Login {
        pub email: Creds,
        pub ninja: Auth,
    }
    // Email credential fields 
    #[derive(Debug, Deserialize)]
    pub struct Creds {
        pub username: String,
        pub password: String,
        pub relay: String,
    }
    // Authorization fields to be sent as POST in JSON format to Ninja API
    #[derive(Debug, Deserialize)]
    pub struct Auth {
        pub grant_type: String,
        pub client_id: String,
        pub refresh_token: String,
    }
    // Structure to parse JSON response from POST and receive access token from Ninja
    #[derive(Deserialize)]
    pub struct AuthResponse {
        pub access_token: String,
        pub refresh_token: String,
        pub expires_in: isize,
        pub scope: String,
        pub token_type: String,
    }
    // Structure to format 'Create Ticket' to be sent as JSON to Ninja API
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ticket {
        pub client_id: i64,
        pub ticket_form_id: i64,
        pub location_id: i64,
        pub node_id: i64,
        pub subject: String,
        pub description: TicketDescription,
        pub status: String,
        pub cc: CcEmail, 
        pub attributes: Vec<TicketAttributes>,
    }
    // More formatting for ticket description (Ticket message within Ninja RMM service)
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TicketDescription {
        pub public: bool,
        pub body: String,
        pub html_body: String,
    }
    // More formatting for cc email to send to actual requester, 
    // ticket is created under my user account as the requester
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CcEmail {
        pub emails: Vec<String>,
    }
    // More formatting for ticket attributes (What type of service requested, i.e. Desktop Support)
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TicketAttributes {
        pub attribute_id: isize, 
        pub value: String,
    }
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AttributeObjects {
        objects: Vec<serde_json::Value>,
    }
    // Data structure for handling JavaScript object from frontend
    #[derive(Deserialize)]
    pub struct FormData {
        pub ticket_type: Option<String>,
        pub locale: Option<String>,
        pub dealer: Option<String>,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub email: Option<String>,
        pub telephone: Option<String>,
        pub request_type: Option<String>, 
        pub request: Option<String>,
    }
    // Data structure to format email message from FormData
    pub struct EmailMessage {
        pub name: String,
        pub subject: String,
        pub contact: String,
        pub location: String,
        pub details: String,
        pub pcname: String,
    }
    impl FormData {
        pub fn as_vec(&self) -> Vec<String> {
            let mut vec = Vec::new();
            vec.push(
                format!(
                    "Name: {} {}", 
                    self.first_name.clone().unwrap(), 
                    self.last_name.clone().unwrap()
                )
            );
            vec.push(
                format!(
                    "Contact: {} {}", 
                    self.email.clone().unwrap(),
                    self.telephone.clone().unwrap_or_else(|| format!("No extension"))
                )
            );
            vec.push(
                format!(
                    "Location: {} {}", 
                    self.locale.clone().unwrap(), 
                    self.dealer.clone().unwrap_or_else(|| format!("No dealer selected"))
                )
            );
            vec.push(self.request.clone().unwrap()); 

            vec
        }
    }
}
