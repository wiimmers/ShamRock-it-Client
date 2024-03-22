pub mod db {
    use rusqlite::{Connection, Result as SQLResult}; 
    use std::{
        fs,
        path::PathBuf
    };
    use serde::{Deserialize, Serialize};
    use tauri::Window;
    use whoami::username; 
    
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatusObject {
        pub ticket_no: String, 
        pub status: String,
    }
    #[derive(Debug, Deserialize, Serialize)]
    struct TicketData {
        id: u32,
        status: Status,
    }
    #[derive(Debug, Deserialize, Serialize)]
    struct Status {
        name: String
    }

    #[tauri::command]
    pub async fn ticket_statuses(window: Window) -> Result<(), tauri::Error> {

        let db = new_db_file().await.expect("Failed to open database connection");
        let tickets = get_all_tickets(window.clone()).await.unwrap();
        let client = reqwest::Client::new(); 
        let update_sql = "UPDATE tickets SET status = ? WHERE ticketNo = ?";
        let delete_sql = "DELETE FROM tickets WHERE ticketNo = ?";

        let json = serde_json::to_string(&tickets)?;

        let request = client
            .post("http://10.2.1.57/status")
            .header("Content-Type", "application/json")
            .header("accept", "application/json")
            .body(json)
            .send()
            .await
            .expect("Failed to send request to ShamRock-it server");

        let response: serde_json::Value = request.json().await.expect("Failed to parse server response as JSON");     

        for i in 0..tickets.len() {
            let response_ticket: StatusObject = serde_json::from_value(response[i].clone())?;
            let current_status = &response_ticket.status;
            if current_status != &tickets[i].status {
                db.execute(update_sql, [ &current_status, &tickets[i].ticket_no ]).expect("Failed to update database");
            } else if current_status == "CLOSED" {
                db.execute(delete_sql, [ &tickets[i].ticket_no ]).expect("Failed to update database"); 
            }
        }


        Ok(())
    }

    async fn get_all_tickets(window: Window) -> SQLResult<Vec<StatusObject>> {
        let db = new_db_file().await?;
        let mut stmt = db.prepare("SELECT ticketNo, status FROM tickets")?;

        let tickets_iter = stmt.query_map([], |row| {
            Ok(StatusObject {
                ticket_no: row.get(0)?,
                status: row.get(1)?
            })
        })?;

        let mut tickets = Vec::new(); 

        for ticket in tickets_iter {
            tickets.push(ticket?);
        }

        let json = serde_json::to_string(&tickets).expect("Failed to parse JSON data"); 
        window.emit("ticketObject", json).expect("Failed to emit ticket status"); 

        Ok(tickets)
    }

    pub async fn insert_ticket_row(ticket_data: &String) -> Result<(), serde_json::Error> {

        let db = new_db_file().await.expect("failed to open ticket.db");

        let json: TicketData = serde_json::from_str(ticket_data)?;
        let stmt = "INSERT INTO tickets (ticketNo, status) VALUES (?, ?)";

        match db.execute(stmt, (&json.id.to_string(), &json.status.name)) {
            Ok(_) => {
                println!("Ticket inserted successfully");
                db.close().expect("Failed to close db"); 
                Ok(())
            },
            Err(err) => {
                println!("Error: {}", err);
                Ok(())
            }
        }
    }

    // Opens a connection to the database file created in the AppData directory
    // Creates a new one if not found 
    async fn new_db_file() -> SQLResult<Connection>{
        let db_dir: PathBuf = format!("C:/Users/{}/AppData/Local/ShamRock-it-db/", username()).into();  
        let db_path: PathBuf = db_dir.join("tickets.db");
        let _ = fs::create_dir_all(db_dir); // Create if not found 

        let db = Connection::open(db_path)?; 
        let stmt = "CREATE TABLE IF NOT EXISTS tickets(
            ticketNo TEXT,
            status TEXT
        )";
        db.execute(stmt, [])?;

        Ok(db) // Return it for future connection 
    }

}