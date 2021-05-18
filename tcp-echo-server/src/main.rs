use std::{
    collections::HashMap,
    io::{self, Read, Write},
    thread::JoinHandle,
    time,
};
use std::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use std::{sync::Arc, thread};

// static state: Mutex<i32> = Mutex::new(0);

#[derive(Debug)]
struct State {
    count: u32,
    session: HashMap<u32, ClientData>,
}

impl State {
    fn new() -> State {
        State {
            count: 0,
            session: HashMap::new(),
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

fn print_state(state: &Arc<Mutex<State>>) {
    let mut state = state.lock().unwrap();

    println!("{:?}", state)
}

fn handle_client(
    client_number: usize,
    mut stream: TcpStream,
    state: &Arc<Mutex<State>>,
) -> JoinHandle<Result<(), io::Error>> {
    println!("client_number: {}", client_number); // TODO: I don't need client_number any more. I need

    let state = Arc::clone(state);
    // Handle client in its own thread
    let result = thread::spawn(move || -> Result<(), io::Error> {
        let client_id = get_new_client_id(&state);
        println!("Handling new connection. Client Id: {}", client_id);

        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;

        // Wait 3seconds
        println!("Read from client {}. Message: {}", client_id, buffer);
        thread::sleep(time::Duration::from_secs(3));

        println!("Echo back to client {}: {}", client_id, buffer);
        stream.write_all(format!("Server [{}] >> {}\n", client_id, buffer).as_bytes())?;

        // Update state
        update_state_new_message(client_id, buffer, &state);

        // Print state
        print_state(&state);

        Ok(())
    });

    result
}

fn main() -> io::Result<()> {
    let bind_address = "127.0.0.1:8080";
    let listener = TcpListener::bind(bind_address)?;
    println!("Server is up! Listening at: {}", bind_address);

    let state = Arc::new(Mutex::new(State::new()));

    let mut threads: Vec<JoinHandle<Result<(), io::Error>>> = Vec::new();
    for (client_number, stream_result) in listener.incoming().enumerate() {
        let mut stream = stream_result.unwrap();
        let thread = handle_client(client_number, stream, &state);

        threads.push(thread);
    }

    println!("Waiting for all threads to finish");

    // Wait for all threads
    for thread in threads {
        thread.join();
    }

    Ok(())
}
