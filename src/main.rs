#![feature(plugin,custom_derive,custom_attribute,time2)]

extern crate byteorder;
extern crate libxdo;

//use enum_traits::{FromIndex,Index};
use std::{net,slice,thread,time};
use std::sync::mpsc;

pub mod data{
	use byteorder::{LittleEndian,ReadBytesExt};
	use std::io::Cursor;

	#[derive(Debug,Eq,PartialEq)]
	pub enum Type{
		PRESS   = 1,
		RELEASE = 2,
		MOVE    = 3,
	}

	impl Type{
		pub fn from_value(value: u32) -> Option<Self>{
			use self::Type::*;
			match value{
				1 => Some(PRESS),
				2 => Some(RELEASE),
				3 => Some(MOVE),
				_ => None
			}
		}
	}

	#[derive(Debug,PartialEq)]
	pub struct Data{
		pub ty: Type,
		pub pressure: f32,
		pub size: f32,
		pub x: f32,
		pub y: f32,
	}

	impl Data{
		pub fn deserialize(bytes: &[u8]) -> Result<Self,DataDeserializeErr>{
			if bytes.len()==20{
				use self::DataDeserializeErr::InvalidData;

				let mut reader = Cursor::new(bytes);
				Ok(Data{
					ty: try!(Type::from_value(try!(reader.read_u32::<LittleEndian>().map_err(|_| InvalidData))).ok_or_else(|| InvalidData)),
					pressure: try!(reader.read_f32::<LittleEndian>().map_err(|_| InvalidData)),
					size:     try!(reader.read_f32::<LittleEndian>().map_err(|_| InvalidData)),
					x:        try!(reader.read_f32::<LittleEndian>().map_err(|_| InvalidData)),
					y:        try!(reader.read_f32::<LittleEndian>().map_err(|_| InvalidData)),
				})
			}else{
				Err(DataDeserializeErr::InvalidDataSize(bytes.len()))
			}
		}
	}

	#[derive(Debug,Eq,PartialEq)]
	pub enum DataDeserializeErr{
		InvalidDataSize(usize),
		InvalidData,
	}
}

/*pub mod input_backend{
	pub trait MouseButton: Sized{
		fn left() -> Self;
		fn middle() -> Self;
		fn right() -> Self;
	}

	pub trait Mouse{
		type ButtonId: MouseButton;
		type Axis;

		fn mov(&self,x: Self::Axis,y: Self::Axis);
		fn set(&self,x: Self::Axis,y: Self::Axis);
		fn press(&self,button: Self::ButtonId);
		fn release(&self,x: Self::Axis,y: Self::Axis);
	}

	pub struct XDo(libxdo::XDo);
	impl Default for XDo{
		fn default() -> Self{libxdo::XDo::new(None).unwrap()}
	}
	impl Mouse for XDo{
		
	}
}*/

fn main(){
	////////////////////////////////////////////////
	//Host's input control initialization
	let (repeater_sender,repeater_receiver) = mpsc::channel::<Option<(f32,f32)>>();
	let /*repeater*/_ = thread::spawn(move||{
		const SLEEP_MS: (u32,u32) = (15,100);
		//const POW: f32 = 1.3;
		const MULTIPLIER: f32 = 1.0/80.0;

		let xdo = libxdo::XDo::new(None).unwrap();
		let mut repeat: Option<(f32,f32,u32,bool)> = None;
		fn repeat_from_client_data((x,y): (f32,f32)) -> (f32,f32,u32){
			//Recommendations: abs(DATA)*sleep_compensation >= 1.0, abs(DATA)*sleep_compensation mod 1.0 = 0.0
			//Requirements: SLEEP_MS.0 <= sleep_ms <= SLEEP_MS.1
			//Solve for sleep_compensation and sleep_ms:
			//  abs(DATA) * sleep_compensation * SLEEP_MS.0 <= abs(DATA) * sleep_compensation * sleep_ms <= abs(DATA) * sleep_compensation * SLEEP_MS.1
			//  sleep_compensation * SLEEP_MS.0 <= sleep_compensation * sleep_ms <= sleep_compensation * SLEEP_MS.1
			//
			//  abs(DATA) * sleep_compensation = n
			//  sleep_compensation = n/abs(DATA)
			//
			//  abs(DATA) * sleep_compensation * sleep_ms = abs(DATA)
			//  sleep_compensation * sleep_ms = 1.0
			//  sleep_compensation = 1.0/sleep_ms

			let sleep_ms = SLEEP_MS.0;
			let x = client_data.0 * MULTIPLIER * (sleep_ms as f32);
			let y = client_data.1 * MULTIPLIER * (sleep_ms as f32);

			(x,y,sleep_ms)
		}

		//The accumulated pixels per step
		let mut pixels_per_step = (0.0,0.0);
		loop{
			//println!("{:?}, {:?}",repeat,pixels_per_step);

			//When there are data indicating how the movement should repeat for each step (amount of x and y movement, amount of sleep duration)
			if let Some((x_per_sleep_ms,y_per_sleep_ms,sleep_ms)) = repeat{
				//Check if there are new repeat data, and receive it if there are (without waiting for it if it is not yet ready)
				match repeater_receiver.try_recv(){
					//When there are data
					Ok(client_data) => {
						repeat = client_data.map(repeat_from_client_data);
						pixels_per_step = (0.0,0.0);
					},

					//When there are no data yet
					Err(mpsc::TryRecvError::Empty) => {},

					//Probably some error concerning the receiving process
					e => {
						println!("Error when receiving from the repeater thread: {:?}",e);
						return
					}
				};

				//Move the mouse relative to its current position using the repeat data
				if let Err(e) = xdo.move_mouse_relative(
					{pixels_per_step.0+= x_per_sleep_ms; if pixels_per_step.0.abs() >= 1.0{let tmp = pixels_per_step.0 as i32; pixels_per_step.0-= tmp as f32; tmp}else{0}},
					{pixels_per_step.1+= y_per_sleep_ms; if pixels_per_step.1.abs() >= 1.0{let tmp = pixels_per_step.1 as i32; pixels_per_step.1-= tmp as f32; tmp}else{0}},
				){println!("Error: Unable to move cursor: {:?}",e);}
				thread::sleep(time::Duration::from_millis(sleep_ms as u64));
			}else{
				//Wait for new repeat data, and receive when there are
				match repeater_receiver.recv(){
					//When data has been received
					Ok(client_data) => {
						repeat = client_data.map(repeat_from_client_data);
						pixels_per_step = (0.0,0.0);
					},

					//Probably some error concerning the receiving process
					e => {
						println!("Error when receiving from the repeater thread: {:?}",e);
						return
					}
				};
			}
		}
	});
	let xdo = libxdo::XDo::new(None).unwrap();
	let mut initial_x = 0.0;
	let mut initial_y = 0.0;
	let mut press_time = None;

	////////////////////////////////////////////////
	//Data transfer initialization
	let socket = net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),4764)).unwrap();
	let mut buffer: [u8; 20] = [0; 20];

	////////////////////////////////////////////////
	//Receive data from the network
	while let Ok((buffer_size,/*address*/_)) = socket.recv_from(&mut buffer){
		//Deserialize (parse) the received bytes and check the type of message
		match data::Data::deserialize(unsafe{slice::from_raw_parts(&buffer as *const _,buffer_size)}){
			//Data seems valid (parseable)
			Ok(data) => match data.ty{
				data::Type::PRESS => {
					//Record data for use in release
					initial_x = data.x;
					initial_y = data.y;
					press_time = Some(time::Instant::now());
				},
				data::Type::RELEASE => {
					//Let the repeater cancel any movements (If there were any)
					if let Err(e) = repeater_sender.send(None){println!("Error: Cannot send through channel: {:?}",e);}

					//Check if no movement were between the press and release => A click
					if initial_x == data.x && initial_y == data.y{
						//Check whether it was a long click or short click (in time)
						match press_time{
							Some(time) if {let elapsed = time.elapsed(); elapsed.as_secs()>=1 || elapsed.subsec_nanos()>300_000_000}
							   => if let Err(e) = xdo.click(3){println!("Error: Unable to right click with cursor: {:?}",e);},
							_  => if let Err(e) = xdo.click(1){println!("Error: Unable to left click with cursor: {:?}",e);}
						}
					}
				},
				data::Type::MOVE => {
					//Let the repeater handle movement
					if let Err(e) = repeater_sender.send(Some((
						data.x-initial_x,
						data.y-initial_y
					))){println!("Error: Cannot send through channel: {:?}",e);}
				},
			},

			//Currently all valid packets have the same size
			Err(data::DataDeserializeErr::InvalidDataSize(size)) => println!("Error: Expected buffer size 8, got {}",size),

			//Invalid packet
			Err(data::DataDeserializeErr::InvalidData) => println!("Error: Invalid packet data"),
		};
	}
}
