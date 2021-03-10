use async_std::task;
use grammers_client::{Client, Config, SignInError};
use grammers_mtsender::AuthorizationError;
use grammers_session as session;
use grammers_session::FileSession;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};
use std::io;
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};

pub async fn get_new_session() -> Result<Client<FileSession>, AuthorizationError> {
    println!("Connecting to Telegram...");
    let api_id = 123259;
    let api_hash = "e88ec58aa1ce01f5630e194e9571d751".to_string();
    let cf = Config {
        session: FileSession::load_or_create("./s4.session").unwrap(), //session::Session::load_or_create("./s1.session").unwrap(),
        api_id: api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    };
    let mut client = Client::connect(cf).await.unwrap();
    println!("Connected!");

    println!("Sending ping...");
    dbg!(client.invoke(&tl::functions::Ping { ping_id: 90 }).await?);
    println!("Ping sent successfully!");

    if !client.is_authorized().await? {
        println!("Signing in...");
        // let phone = prompt("Enter your phone number (international format): ")?;
        let phone = "79620285396";
        let phone = "989338828058";
        let token = client
            .request_login_code(&phone, api_id, &api_hash)
            .await
            .unwrap();
        let code = prompt("Enter the code you received: ").unwrap();
        let signed_in = client.sign_in(&token, &code).await;
        match signed_in {
            Err(SignInError::PasswordRequired(password_token)) => {
                // Note: this `prompt` method will echo the password in the console.
                //       Real code might want to use a better way to handle this.
                // let hint = password_token.hint().unwrap();
                // let prompt_message = format!("Enter the password (hint {}): ", &hint);
                let prompt_message = format!("Enter the password (hint ): ");
                let password = prompt(prompt_message.as_str()).unwrap();

                client
                    .check_password(password_token, password.trim())
                    .await
                    .unwrap();
            }
            Ok(_) => (),
            Err(e) => panic!(e),
        };
        println!("Signed in!");
        match client.session().save() {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "NOTE: failed to save the session, will sign out when done: {}",
                    e
                );
                // sign_out = true;
            }
        }
    }
    Ok(client)
}

fn read() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => panic!("Can not get input value: {:?}", e),
    }
}

fn prompt(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}
