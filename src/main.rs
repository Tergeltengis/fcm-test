use reqwest::{Client, Error};
use std::time::Instant;

const NUM_REQUESTS: usize = 10000;

async fn send_requests(client: &Client) -> Result<(), Error> {
    let start_time = Instant::now(); 
    let mut handles = vec![];

    for i in 0..NUM_REQUESTS {
        let client = client.clone();

        let handle = tokio::spawn(async move {
            if let Err(err) = send_request(&client, i).await {
                println!("Failed to send request: {:?}", err);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Failed to join task");
    }

    let duration = start_time.elapsed().as_secs_f64(); 
    let avg_requests_per_second = NUM_REQUESTS as f64 / duration;

    println!(
        "Average requests per second: {:.2}",
        avg_requests_per_second
    );

    Ok(())
}

async fn send_request(client: &Client, counter: usize) -> Result<(), Error> {
    let request_body = format!(
        r#"{{"message": {{"token": "c38QVqNdTNeVGZiTwDnYDZ:APA91bEkrsDSqtTcWsc0CVQ1mTEWA-Y6S-iB7UhEu8jDaPRhVUOg_4Ep9SXvmnHcIZZ2nKp7VyfHKU7IbFpmCsLQsT01mrnsrgDL8a8mZkL7aRgVTLJKc5yekyEAnlTVA5Eeas1X-26m", "notification": {{"title": "Title new", "body": "This is a test message. Counter: {}"}}}}}}"#,
        counter
    );

    let response = client
        .post("https://fcm.googleapis.com/v1/projects/demonotification-23701/messages:send")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ya29.c.c0AY_VpZgn7IgB2HogA55UNxgfleE8giZJb73mp1EamQ2pUofPo41c2h7Zs3CBH-Ph2K0EW3zF3xcSTSwkI-C_qgeIwAHmPkfVdtmIEK8A1yU5TY0gDn_yIcPOqlgN4bVUEjk5tQBfmxyxPYwEUg550D5x1FKnTVQMvPh2igjWnT-nfAwxmGgzkBNVFt6gFUqkJEFstDIWfenbJ7znN8qUiRRI42tATQ2nRdwantmk6OD_totKxGMEUXEiP06fCvTe1cQg2JUaluWyoyhBUxE7WApaFukrXPVP0YWlq0HI41aj0YuFdgMFqLe0gxLfnEAF89bpX2Wf6yYBVMW3GIDlhPhFdEyOtQWN9JwfpE5hQVKrPUXTY_9WTmjJN385PcOkSWwxr25n7nl9bt7j6cvds5fRjlZpsqzJhXZ2SF71kO0v4gQQat1RktX9dkzvki8JqsOIQu7xyBUncSuVOmchqR1OU5Vf_oSFhY8OwcnnylrS4WZeUuQvS5ji-_uY_SjB3Jarn3-rUaYYeZf2azZ43b72u538YgM9naJSqQeovn86rbUv-ymZZ57kMUhsX57YScQ71aytkklrqp6x5q8FUB6feYduk08tu_W4jOkeFbj5ZqM_FB_7eVrurajycs3_jXzeIdIgf-Fehrfnfo47QdIBcIBqakexFnp0i98rtocWpFbd4rWI0joyBq80aVIV0JYapVVcfgpfdFfJ36Skndwdm-l8S3XjWQgyuzJ9b4stFt8X--6Mgpv3Xlish0hZMwtRY2eBbUebu9VcFp9tinU6VOj8aWUBe0urX9npuBMMoSh9y1mkMUtUivpkSwOk-_JQYvBtk1kgeo1i-02fUy6X_aUuuRUnMQyx_8aB-SoVVniSZBZu8MU1mWsJMh2kuno7lio5U8lieZMO6rsW6izJ_1X3evoFSvJnB1tm4ipvYuOwnMbpnYjd8y7Urat-rpvitn9bIurmFMWJVn32B7sjyoX4guhgrMeSn35hspUJ1vhb645B4-U")
        .body(request_body)
        .send()
        .await?;

    match response.status().as_u16() {
        200 => println!("Notification sent successfully. Counter: {}", counter),
        _ => println!(
            "Failed to send notification. Status code: {}. Counter: {}",
            response.status(),
            counter
        ),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    if let Err(err) = send_requests(&client).await {
        println!("Error sending requests: {:?}", err);
    }
}
