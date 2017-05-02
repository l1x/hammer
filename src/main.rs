extern crate hyper;

//use hyper::client::response::Response;
use hyper::client::Client;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::SystemTime;
use std::thread;

static NTHREADS:  i32 = 8;
static NREQ:      i32 = 3000;

fn main() {
  
  let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

  for id in 0..NTHREADS {
    let thread_tx = tx.clone();
    thread::spawn(move || {
      let client = Client::new();
      let mut perf = Vec::with_capacity(NREQ as usize);
      
      for _ in 0..NREQ {

        let now = SystemTime::now();
        let res = client.get("http://localhost:8080/hello").send()?.read_to_end();
        let _ = match res {
          Ok(_) => 0,
          Err(_) => 1,
        };
        let sec = match now.elapsed() {
          Ok(elapsed)   => { (elapsed.as_secs() as f64 * 1000.0 ) + (elapsed.subsec_nanos() as f64 / 1000_000.0) }
          Err(_e)       => { 0.0 }
        };
        perf.push(sec);
      }
      let sum: f64 = perf.iter().fold(0.0, |sum, &val| sum + val);
      let performance: i32 = ((NREQ as f64) / (sum / 1000.0)).round() as i32;
      println!("thread {} finished, performance: {} req/sec", id, performance);

      thread_tx.send(performance).unwrap();

    });
  }

  let mut ids = Vec::with_capacity(NTHREADS as usize);

  for _ in 0..NTHREADS {
    ids.push(rx.recv().unwrap());
  }
  
  let overall: i32 = ids.iter().fold(0, |sum, &val| sum + val);
  println!("The overall performance is: {:?}", overall);

}


