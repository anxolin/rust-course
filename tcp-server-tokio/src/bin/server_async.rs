use tokio::task;

use serde_tcp_server::{Payload, Request, Response};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    time,
};
use std::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use std::{sync::Arc, thread};

#[derive(Debug)]
struct State {
    count: u32,
    session: HashMap<u32, ClientData>,
    payload: Option<Payload>,
}

impl State {
    fn new() -> State {
        State {
            count: 0,
            session: HashMap::new(),
            payload: None,
        }
    }
}

#[derive(Debug)]
struct ClientData {
    messages: Vec<String>,
}

fn get_new_client_id(state: &Arc<Mutex<State>>) -> u32 {
    let mut state = state.lock().unwrap();
    state.count += 1;

    state.count
}

fn update_state_new_message(client_id: u32, message: String, state: &Arc<Mutex<State>>) {
    let mut state = state.lock().unwrap();

    let client_data = state.session.entry(client_id).or_insert(ClientData {
        messages: Vec::new(),
    });

    // Add message
    client_data.messages.push(message);
}

fn get_payload(state: &Arc<Mutex<State>>) -> Option<Payload> {
    let state = state.lock().unwrap();

    state.payload.clone()
}

fn set_payload(payload: Payload, state: &Arc<Mutex<State>>) {
    let mut state = state.lock().unwrap();
    state.payload = Some(payload);
}

fn print_state(state: &Arc<Mutex<State>>) {
    let state = state.lock().unwrap();

    println!("STATE: {:?}", state)
}

fn get_request(stream: &mut TcpStream) -> Result<Request, io::Error> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    println!("Process request: {}", buffer);
    let request_result: serde_json::Result<Request> = serde_json::from_str(&buffer);

    if request_result.is_err() {
        println!("Error parsing request: {:?}", request_result)
    }

    let request = request_result?;

    Ok(request)
}

fn get_response(
    client_id: u32,
    request: Request,
    state: &Arc<Mutex<State>>,
) -> Result<Response, io::Error> {
    // Wait 3 seconds
    println!("Read from client {}. Request: {:?}", client_id, request);
    thread::sleep(time::Duration::from_secs(3));

    let response = match request {
        Request::GET => Response::Result(get_payload(state)),
        Request::SET(payload) => {
            set_payload(payload, state);
            Response::OK
        }
    };

    Ok(response)
}

fn serialize_response(response: &Response) -> String {
    // let serialized = response.payload.data.clone(); // TODO: serialize using serde
    // format!("Server [{}] >> {}\n", client_id, serialized)
    let serialized = serde_json::to_string(&response).unwrap();

    serialized
}

async fn handle_client(mut stream: TcpStream, state: Arc<Mutex<State>>) -> io::Result<()> {
    // print_state(&state);
    let client_id = get_new_client_id(&state);
    println!("Handling new connection. Client Id: {}", client_id);

    // Get request
    let request = get_request(&mut stream)?;

    // Get response
    let response = get_response(client_id, request, &state)?;

    // Serialize and send response
    println!("Reply to client {}: {:?}", client_id, response);
    let serialized = serialize_response(&response);
    stream.write_all(serialized.as_bytes())?;

    // Update state
    update_state_new_message(client_id, serialized, &state);

    // Print state
    print_state(&state);

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind_address = "127.0.0.1:8080";
    let listener = TcpListener::bind(bind_address)?;
    println!("Server is up! Listening at: {}", bind_address);

    let state = Arc::new(Mutex::new(State::new()));

    let mut tasks = Vec::new();
    for stream_result in listener.incoming() {
        let mut stream = stream_result.unwrap();

        let state = Arc::clone(&state);
        let task = task::spawn(handle_client(stream, state));
        tasks.push(task);
    }

    println!("Waiting for all tasks to finish");

    // Wait for all tasks if still running
    for task in tasks {
        let _ = task.await?;
    }

    Ok(())
}
