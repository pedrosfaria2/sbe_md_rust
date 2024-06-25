use pcap_file::PcapReader;
use std::fs::File;
use std::io::BufReader;
use sbe_b3__umdf_mbo_sbe::message_header_codec::decoder::MessageHeaderDecoder;
use sbe_b3__umdf_mbo_sbe::ReadBuf;
use std::error::Error;
use sbe_b3__umdf_mbo_sbe::message_type::MessageType;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "pcap-examples/Corvil-local-cne-export-1680639597609180908-1680645600235576416.pcap";
    let file = File::open(path)?;
    let mut pcap_reader = PcapReader::new(BufReader::new(file)).expect("Failed to create PcapReader");

    let mut num_packets = 0;

    while let Some(Ok(packet)) = pcap_reader.next() {
        num_packets += 1;
        println!("packet #{}:", num_packets);

        let data = packet.data;
        println!("Packet data (first 8 bytes): {:?}", &data[0..8.min(data.len())]);

        process_message(&data);
    }

    println!("Total packets: {}", num_packets);

    Ok(())
}

fn process_message(message: &[u8]) {
    if message.len() < 8 {
        eprintln!("Packet too short to decode");
        return;
    }

    let read_buf = ReadBuf::new(message);
    let header_decoder = MessageHeaderDecoder::default().wrap(read_buf, 0);

    let template_id = header_decoder.template_id();
    let block_length = header_decoder.block_length();
    let schema_id = header_decoder.schema_id();
    let version = header_decoder.version();

    println!("Raw message data: {:?}", message);

    println!("Template ID: {}", template_id);
    println!("Block Length: {}", block_length);
    println!("Schema ID: {}", schema_id);
    println!("Version: {}", version);

    // Identificando o tipo de mensagem
    if message.len() > 8 {
        let message_type = MessageType::from(message[8]);
        println!("Message Type: {:?}", message_type);
    } else {
        println!("Message too short to decode type.");
    }

    println!("Received message with template ID: {}", template_id);
}
