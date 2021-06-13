use goose::prelude::*;
use hyper::http::StatusCode;
use rand::Rng;

async fn read_keys(user: &GooseUser) -> GooseTaskResult {
    let key: i32 = rand::thread_rng().gen_range(1..1000);
    let mut goose = user.get(&format!("/get/{}", key)).await?;
    match goose.response {
        Ok(response) => {
            if response.status() != StatusCode::OK && response.status() != StatusCode::NOT_FOUND {
                return user.set_failure(
                    &format!("Invalid status code {}", response.status()),
                    &mut goose.request,
                    None,
                    None,
                );
            }
        }
        Err(err) => {
            return user.set_failure(
                &format!("No response from memster {}", err),
                &mut goose.request,
                None,
                None,
            );
        }
    }
    Ok(())
}

async fn write_keys(user: &GooseUser) -> GooseTaskResult {
    let key: i32 = rand::thread_rng().gen_range(1..1000);
    let mut goose = user
        .post(
            &format!("/set/{}", key),
            "Soome very important value attached to the request",
        )
        .await?;
    match goose.response {
        Ok(response) => {
            if response.status() != StatusCode::OK {
                return user.set_failure(
                    &format!("Invalid status code {}", response.status()),
                    &mut goose.request,
                    None,
                    None,
                );
            }
        }
        Err(err) => {
            return user.set_failure(
                &format!("No response from memster {}", err),
                &mut goose.request,
                None,
                None,
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), GooseError> {
    println!("Hello, world!");
    GooseAttack::initialize()?
        .register_taskset(
            taskset!("Test")
                .set_host("http://localhost:8080")
                .register_task(task!(read_keys).set_weight(4)?.set_name("Read keys"))
                .register_task(task!(write_keys).set_weight(1)?.set_name("Write keys")),
        )
        .execute()?
        .print();

    Ok(())
}
