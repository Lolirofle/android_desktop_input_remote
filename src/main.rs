#![feature(plugin,custom_derive,custom_attribute)]
#![plugin(enum_traits_macros)]

extern crate byteorder;
extern crate enum_traits;
extern crate libxdo;

use byteorder::{LittleEndian,ReadBytesExt};
use enum_traits::{FromIndex,Index};
use std::io::Cursor;
use std::{net,thread};
use std::sync::mpsc::channel;

#[derive(Debug,EnumIndex,EnumFromIndex)]
enum Type{
	PRESS   = 1,
	RELEASE = 2,
	MOVE    = 3,
}

impl Type{
	fn from_value(value: u32) -> Option<Self>{
		use self::Type::*;
		match value{
			1 => Some(PRESS),
			2 => Some(RELEASE),
			3 => Some(MOVE),
			_ => None
		}
	}
}

fn main(){
	//Host's input control initialization
	let xdo = libxdo::XDo::new(None).unwrap();
	let mut initial_x = 0.0;
	let mut initial_y = 0.0;
	let (repeater_sender,repeater_receiver) = channel();
	let repeater = thread::spawn(||{
		let mut repeat = None;
		loop{
			if let Some(data) = repeat{
				thread::sleep_ms(interval);
				repeater_receiver.try_recv()
			}else{
				repeater_receiver.recv()
			}
		}
	});

	//Data transfer initialization
	let socket = net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),4764)).unwrap();
	let mut buffer: [u8; 20] = [0; 20];

	while let Ok((buffer_size,address)) = socket.recv_from(&mut buffer){
		if buffer_size==20{
			let mut reader = Cursor::new(&buffer as &[u8]);
			let ty = Type::from_value(reader.read_u32::<LittleEndian>().unwrap()/* as <Type as Index>::Type*/).unwrap();
			let pressure = reader.read_f32::<LittleEndian>().unwrap();
			let size = reader.read_f32::<LittleEndian>().unwrap();
			let x = reader.read_f32::<LittleEndian>().unwrap();
			let y = reader.read_f32::<LittleEndian>().unwrap();

			match ty{
				Type::PRESS => {
					initial_x = x;
					initial_y = y;
				},
				Type::RELEASE => {
					if initial_x == x && initial_y == y{
						xdo.click(1).unwrap()
					}
				},
				Type::MOVE => xdo.move_mouse_relative(((x-initial_x)/6.0) as i32,((y-initial_y)/6.0) as i32).unwrap(),
			};

			//println!("x: {}, y: {}, type: {:?}, pressure: {}, size: {} , address: {}",x,y,ty,pressure,size,address);
		}else{
			println!("Expected buffer size 8, got {}",buffer_size);
		}
	}
}
