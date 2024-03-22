pub mod ninja {
    use crate::db::db::insert_ticket_row;
    use crate::structures::structures::{AuthResponse, Ticket, CcEmail, TicketAttributes, TicketDescription, FormData};
    use std::collections::HashMap;
    use crate::info::info::{get_creds, get_pc_name};
    use serde_json::Value;
    use tauri::Window;

    // Start Ninja One authentication, handle whether or not a refresh token is present
    // If no refresh token, store it in Windows credential manager
    #[tauri::command]
    pub async fn ninja_auth(window: Window, form_data: FormData) {
        window.emit("submitTicket", 0).expect("failed to emit submitTicket"); 

        let client = reqwest::Client::new(); 
        let tokens = get_access(&client).await.unwrap();
        let access = format!("Bearer {}", tokens.access_token);  
        let id = get_device_id(&client, &access).await.unwrap();

        let request_type = get_attributes(&client, &access, form_data.request_type.as_ref().unwrap()).await; 

        let ticket = build_ticket(id, &form_data, request_type); 
        let _ = submit_ticket(window.clone(), &client, &access, &ticket).await; 
    
    }
    // Send post request to get access token and initial refresh token for the application
    async fn get_access(client: &reqwest::Client) -> Result<AuthResponse, reqwest::Error> {
        let mut map = HashMap::new();
        let client_credentials = get_creds().await?;

        map.insert("grant_type", &client_credentials.ninja.grant_type);
        map.insert("client_id", &client_credentials.ninja.client_id);
        map.insert("refresh_token", &client_credentials.ninja.refresh_token);

        let request = client
            .post("https://jakesweeney.rmmservice.com/ws/oauth/token")
            .form(&map)
            .send()
            .await?;

        let response = request.json::<AuthResponse>().await?;
        
        Ok(response)
    }
    // Get device ID and location ID
    async fn get_device_id(client: &reqwest::Client, access: &String) -> Result<(Option<i64>, Option<i64>), serde_json::Error> {
        let organization_url = format!("https://jakesweeney.rmmservice.com/v2/devices/search?q={}&limit=1", get_pc_name());
        let request = client
            .get(organization_url)
            .header("accept", "application/json")
            .header("Authorization", access.as_str())
            .send()
            .await
            .unwrap();

        let response: Value = serde_json::from_str(
            request
            .text()
            .await
            .unwrap()
            .as_str()
        )?;

        let device_id = &response["devices"][0]["id"].as_i64();
        let location_id = &response["devices"][0]["locationId"].as_i64();

        Ok((device_id.clone(), location_id.clone()))
    }
    // Builds ticket using other ticket builder functions (build_description, build_attributes)
    fn build_ticket(id: (Option<i64>, Option<i64>), form_data: &FormData, request_type: String) -> Ticket {
        let location_id = id.1.unwrap(); 
        let node_id = id.0.unwrap(); 

        let mut emails = Vec::new();
        emails.push(form_data.email.clone().unwrap());
        let cc = CcEmail {
            emails
        };

        let ticket = Ticket {
            client_id: 2, // JSA
            ticket_form_id: 1, // Default
            location_id,
            node_id,
            subject: format!("New Ticket Created: {}", form_data.request_type.as_ref().unwrap()),
            description: build_description(&form_data),
            status: String::from("1000"),
            cc,
            attributes: build_attributes(
                form_data.first_name.as_ref().unwrap(),
                form_data.last_name.as_ref().unwrap(),
                form_data.email.as_ref().unwrap(),
                form_data.telephone.as_ref().unwrap(),
                &request_type
            ),
        };

        ticket
    }
    // Builds the body of the message sent for the ticket, i.e. the user's request 
    fn build_description(form_data: &FormData) -> TicketDescription {
        let mut html_vec = Vec::new(); 
        let mut plain_vec = Vec::new(); 

        for paragraphs in form_data.as_vec().iter() {
            plain_vec.push(format!("{paragraphs}")); 
            html_vec.push(format!("{paragraphs}<br>"));
        }

        let html = html_vec.join(""); 
        let plain = plain_vec.join("\n"); 

        let description = TicketDescription {
            public: true,
            body: plain,
            html_body: html,
        };

        description
    }
    // Builds the attributes of the JSON object, name, extension, email, and the type of request 
    fn build_attributes(first: &String, last: &String, email: &String, extension: &String, request_type: &String) -> Vec<TicketAttributes> {
        let mut attributes = Vec::new(); // Vector of TicketAttributes
        // Push name structure
        attributes.push(
            TicketAttributes {
                attribute_id: 28, // Name
                value: format!("{} {}", first, last),
            }
        ); 
        // Push email structure
        attributes.push(
            TicketAttributes {
                attribute_id: 29, // Email
                value: format!("{}", email),
            }
        );
        // Push extension structure
        attributes.push(
            TicketAttributes {
                attribute_id: 30, // Extension
                value: format!("{}", extension),
            }
        ); 
        // Push type of ticket request **desktop support, printer help, etc.
        attributes.push(
            TicketAttributes {
                attribute_id: 27, // Request type, for organizing tickets
                value: format!("{}", request_type),
            }
        ); 
    
        attributes // return vector of structures
    }
    // Gets the different attribute values
    // and matches the name with ticket type, "Printer Support", then assigns the id
    // i.e. 288c8370-c673-4a21-ae7c-85b87df31dfd - Printer Support
    async fn get_attributes(client: &reqwest::Client, access: &String, ticket_type: &String) -> String {
        let mut id = String::new();
        let organization_url = String::from("https://jakesweeney.rmmservice.com/v2/ticketing/attributes");
        let request: Value = client
            .get(organization_url)
            .header("accept", "application/json")
            .header("Authorization", access.as_str())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        // Gets the values for dropdown menu of ticket types and creates an array
        // Response is an array of objects, the attribute is at index 4
        // Nested object 'content', and then another array of values
        let attribute_values = request[4]["content"]["values"].as_array().unwrap(); 

        // Iterates through the array and finds ID for corresponding ticket type
        for objects in attribute_values.iter() {
            let mut object = objects.to_owned();
            let name = object["name"].take().to_string(); 
            if name.trim_matches('\"') == ticket_type.to_string() {
                id = object["id"].take().to_string(); 
            }
        }

        id.trim_matches('\"').to_string()
    }
    // Create ticket at Ninja API endpoint 
    // Ticket is also submitted to the ShamRock-it Server for the TV and into an SQLite DB for the ticket statuses page
    async fn submit_ticket(window: Window, client: &reqwest::Client, access: &String, ticket: &Ticket) -> Result<(), reqwest::Error> {
        let body = serde_json::to_string(&ticket).unwrap();  

        let request = client
            .post("https://jakesweeney.rmmservice.com/v2/ticketing/ticket")
            .header("Content-Type", "application/json")
            .header("accept", "application/json")
            .header("Authorization", access.as_str())
            .body(body)
            .send()
            .await?; 

        let response = request.status();
        let response_body = request.text().await?; 

        if response.is_success() {
            let _ = submit_tv(&client, &response_body).await; // Server submit
            let _ = insert_ticket_row(&response_body).await; // Insert row in SQLite DB
            window.emit("ticketSubmit", response.as_u16()).expect("failed to emit ticketSubmit"); 
        } else {
            window.emit("ticketSubmit", 500).expect("failed to emit ticketSubmit"); 
        }

        Ok(())
    }
    // Submit to server for ShamRock-iTV ticket viewer
    async fn submit_tv(client: &reqwest::Client, ticket_response: &String) -> Result<(), reqwest::Error> {

        let _request = client
            .post("http://10.2.1.57/webhook")
            .header("Content-Type", "application/json")
            .header("accept", "application/json")
            .body(ticket_response.clone())
            .send()
            .await?; 

        Ok(())
    }
}
