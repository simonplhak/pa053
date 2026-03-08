use anyhow::{Ok, Result};
use ndarray::Array2;
use ndarray_linalg::Determinant;

pub mod hw1 {
    tonic::include_proto!("hw1.tasks");
}

const URL: &str = "http://[::1]:58110";
const EMAIL: &str = "514244@mail.muni.cz";

fn matrix_task_to_array2(matrix_task: &hw1::MatrixTask) -> Array2<f64> {
    let rows = matrix_task.rows.len();
    let cols = matrix_task.rows.first().map(|r| r.values.len()).unwrap();

    let mut data = Vec::with_capacity(rows * cols);
    for row in &matrix_task.rows {
        for val in &row.values {
            data.push(*val as f64);
        }
    }

    Array2::from_shape_vec((rows, cols), data).expect("Valid matrix dimensions")
}

fn display_response(response: &hw1::Response) {
    println!("\n=== Response ===");

    match &response.data {
        Some(hw1::response::Data::Info(info)) => {
            println!("Info: {}", info);
        }
        Some(hw1::response::Data::Error(error)) => {
            println!("Error: {}", error);
        }
        None => {
            println!("Data: <empty>");
        }
    }

    match &response.task {
        Some(hw1::response::Task::UnknownTask(data)) => {
            println!("Task: UnknownTask");
            println!("Data: {:?}", data);
        }
        Some(hw1::response::Task::AdderTask(data)) => {
            println!("Task: AdderTask");
            println!("Data: {:?}", data);
        }
        Some(hw1::response::Task::MatrixTask(data)) => {
            println!("Task: MatrixTask");
            println!("Data: {:?}", data);
        }
        None => {
            println!("Task: <empty>");
        }
    }
    println!("==================\n");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Begin task
    let mut client = hw1::homework_client::HomeworkClient::connect(URL).await?;
    let request = tonic::Request::new(hw1::BeginData {
        email: EMAIL.into(),
    });
    let response = client.begin(request).await?.into_inner();
    let token = &response.token;
    display_response(&response);

    // Adder task
    let adder_data = match &response.task {
        Some(hw1::response::Task::AdderTask(data)) => data,
        _ => panic!("Unexpected task type"),
    };
    let response = client
        .adder(hw1::AdderTaskResponse {
            token: token.clone(),
            result: adder_data.a as f64 + adder_data.b as f64,
        })
        .await?
        .into_inner();
    let token = &response.token;
    display_response(&response);

    // Matrix task
    let matrix_data = match &response.task {
        Some(hw1::response::Task::MatrixTask(data)) => data,
        _ => panic!("Unexpected task type"),
    };
    let matrix = matrix_task_to_array2(matrix_data);
    let determinant = matrix.det()? as i64;
    println!("Determinant: {}", determinant);
    let response = client
        .matrix(hw1::MatrixTaskResponse {
            token: token.clone(),
            determinant,
        })
        .await?
        .into_inner();
    display_response(&response);

    // Flip line task
    let mut response_stream = client
        .read_line(hw1::FlipLineRequest {
            token: token.clone(),
        })
        .await?
        .into_inner();

    // Collect all responses and then send them as a stream
    let mut flip_responses = Vec::new();
    while let Some(response) = response_stream.message().await? {
        flip_responses.push(hw1::FlipLineResponse {
            token: token.clone(),
            point: Some(hw1::Point {
                x: response.y,
                y: response.x,
            }),
        });
    }

    // Send as a stream using futures::stream::iter
    let res = client
        .send_line(futures::stream::iter(flip_responses))
        .await?
        .into_inner();
    display_response(&res);
    Ok(())
}
