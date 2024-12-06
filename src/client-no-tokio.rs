use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

//#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Hello, from non-tokio!");
    let rt = tokio::runtime::Runtime::new()?;
    let mut client = rt.block_on(async {
        GreeterClient::connect("http://[::1]:50051").await
    })?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = futures_executor::block_on(async {
        client.say_hello(request).await
    })?;
    
    println!("RESPONSE={:?}", response);
    let mut client_clone = client.clone();

    // Start a new thread and make client.sayhello from it
    let handle = std::thread::spawn(move || {
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        futures_executor::block_on(async {
            let result = client_clone.say_hello(request).await;
            println!("RESPONSE={:?}", result);
        });
    });

    handle.join().unwrap();

    Ok(())
}
