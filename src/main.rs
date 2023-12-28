use std::net::UdpSocket;

const HEADER_BYTES_LENGTH: usize = 12;

#[derive(Debug)]
struct Header {
    id: u16,
    qr_indicator: u8,
    opcode: u8,
    authoritative_answer: u8,
    truncation: u8,
    recursion_desired: u8,
    recursion_available: u8,
    reserved: u8,
    response_code: u8,
    question_count: u16,
    answer_count: u16,
    authority_count: u16,
    additional_count: u16,
}

impl Header {
    fn new(id: u16) -> Self {
        Self {
            id,
            qr_indicator: 1,
            opcode: 0,
            authoritative_answer: 0,
            truncation: 0,
            recursion_desired: 0,
            recursion_available: 0,
            reserved: 0,
            response_code: 0,
            question_count: 0,
            answer_count: 0,
            authority_count: 0,
            additional_count: 0,
        }
    }

    fn as_buf(&self) -> [u8; HEADER_BYTES_LENGTH] {
        let mut buf = [0; HEADER_BYTES_LENGTH];

        buf[0] = (self.id >> 8) as u8;
        buf[1] = self.id as u8;
        buf[2] = (self.qr_indicator << 7)
            | (self.opcode << 3)
            | (self.authoritative_answer << 2)
            | (self.truncation << 1)
            | self.recursion_desired;
        buf[3] = (self.recursion_available << 7) | (self.reserved << 4) | self.response_code;
        buf[4..6].copy_from_slice(&self.question_count.to_be_bytes());
        buf[6..8].copy_from_slice(&self.answer_count.to_be_bytes());
        buf[8..10].copy_from_slice(&self.authority_count.to_be_bytes());
        buf[10..12].copy_from_slice(&self.additional_count.to_be_bytes());

        buf
    }
}

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let header = Header::new(1234);
                let response = header.as_buf();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
