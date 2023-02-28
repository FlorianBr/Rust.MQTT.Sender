/** 
 * Simple CLI Tool to send a MQTT message to a broker
 * 
 * The first steps in RUST, so probably a crappy implementation...
 */

use clap::Parser;               // CLI parameter parsing    
use paho_mqtt as mqtt;          // Connecting MQTT
use anyhow::{Result};           // Return results

// Note: clap uses the comments in the struct to generate the cli-help
#[derive(Parser)]
struct Cli {
    /// Address of the broker (mqtt://<IP>:1883)
    #[arg(short, long)]
    address: String,
    /// The topic to send the message to
    #[arg(short, long)]
    topic: String,
    /// Message to send
    #[arg(short, long)]
    msg: String,
    /// QoS to use (0,1,2)
    #[arg(short, long, default_value_t = 1)]
    qos: i32,
}

fn main() -> Result<()> {

    // Read in parameters
    let args = Cli::parse();
    let host = args.address;

    println!("Connecting to Server: {}", host);

    let cli = mqtt::AsyncClient::new(host)?;        // Create Client
    let conn_opts = mqtt::ConnectOptions::new();        // Create Connection Options
    cli.connect(conn_opts).wait()?;                             // Connect to MQTT server

    let topic = mqtt::Topic::new(&cli, args.topic.clone(),  args.qos.clone()); // Create a topic
    let message = args.msg;

    println!("Publishing message '{}' on the '{}' topic with QOS={}", message, args.topic, args.qos);

    let tok = topic.publish(message.clone());   // Publish
    tok.wait()?;    // Wait...

    let tok = cli.disconnect(None); // Disconnect
    tok.wait()?;    // Wait...

    Ok(())
}
