extern crate byteorder;
extern crate libxdo;

use byteorder::{LittleEndian,ReadBytesExt};
use std::io::Cursor;
use std::net;

fn main(){
	let xdo = libxdo::XDo::new(None).unwrap();

	let socket = net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),4764)).unwrap();
	let mut buffer: [u8; 8] = [0; 8];

	while let Ok((buffer_size,address)) = socket.recv_from(&mut buffer){
		if buffer_size==8{
			let mut reader = Cursor::new(&buffer as &[u8]);
			let x = reader.read_u32::<LittleEndian>().unwrap();
			let y = reader.read_u32::<LittleEndian>().unwrap();

			xdo.move_mouse(x as i32,y as i32,0);

			println!("x: {}, y: {}, address: {}",x,y,address);
		}else{
			println!("Expected buffer size 8, got {}",buffer_size);
		}
	}
}
