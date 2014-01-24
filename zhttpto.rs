//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::io::buffered::BufferedReader;
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut counter: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap(); // addr = IP + PORT
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            unsafe {
                counter += 1;
                println("Number of visits: " + counter.to_str());
            }

            // Get the first line of the request
            let mut host_index = 0;
            while(true) {  
                if(str::eq(&request_str.slice(host_index, host_index+4).to_owned(), &~"Host")) {
                    break;
                }
                host_index += 1;
            }
            let restful = request_str.slice(0, host_index);

            // Split the line into its three components and store them
            let mut temp = ~[];
            for s in restful.split(' ') {
                temp.push(s.to_owned());
            }

            let http_type = temp[0].clone(); 
            let path = temp[1].slice_from(1).to_owned().clone(); 
            let http_meta = temp[2].slice(0, 8).to_owned().clone(); 

            // TODO: Make sure the error page is okay. Something vague about this in the instructions

            let mut path_ending = ~"";
            if(path.len() > 4) {
                path_ending = path.slice(path.len()-4, path.len()).to_owned();
            }

            let mut response: ~str = ~"";

            // Check the validity of the HTTP params
            if(path_ending != ~"html") {
                response = 
                    ~"HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>403</title>
                     <style>body 
                            h1 { font-size:2cm; text-align: center; color: black;}
                     </style></head>
                     <body>
                     <h1>four, oh, three</h1>
                     </body></html>\r\n";
            } else {
                if (http_type == ~"GET" && http_meta == ~"HTTP/1.1") {
                    // Check the validity of the path
                    if(path.len() == 0 || !&Path::new(path.clone()).is_file()) {
                        response = 
                        ~"HTTP/1.1 404 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>404</title>
                         <style>body 
                                h1 { font-size:2cm; text-align: center; color: black;}
                         </style></head>
                         <body>
                         <h1>four, oh, four</h1>
                         </body></html>\r\n";
                    } else {
                        let file = File::open(&Path::new(path.clone()));
                        match (file) {
                            Some(contents) => {
                                let mut reader = BufferedReader::new(contents);
                                for line in reader.lines() {
                                    response = response + "\n" + line;
                                }
                            }
                            None => {
                            }
                        }
                    }
                }
                else {
                    // Invalid HTTP params
                    response = 
                        ~"HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>404</title>
                         <style>body 
                                h1 { font-size:2cm; text-align: center; color: black;}
                         </style></head>
                         <body>
                         <h1>four, oh, three</h1>
                         </body></html>\r\n";
                }
            }


            stream.write(response.as_bytes());
            println!("Connection terminates.");
        }
    }
}
