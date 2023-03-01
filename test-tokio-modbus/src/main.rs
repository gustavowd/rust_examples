//! TCP server example

use futures::future;
use once_cell::sync::OnceCell;
use std::mem;
use std::net::SocketAddr;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use std::thread;
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Service};

#[derive(Debug)]
struct MbServer;

mod sunspec;

use sunspec::{Sunspec, SunspecID, Model1, Model213,ModelEnd};

const SUNSPEC_INIT_ADDR: u16 = 40000;

fn error_response(req: &Request, exc: Exception) -> std::io::Result<Response> {
    Ok(Response::Custom(req2functioncode(&req) | 0x80, vec![exc as u8]))
}

fn req2functioncode(req: &Request) -> u8 {
    match *req {
        Request::ReadCoils(_, _) => 0x01,
        Request::ReadDiscreteInputs(_, _) => 0x02,
        Request::WriteSingleCoil(_, _) => 0x05,
        Request::WriteMultipleCoils(_, _) => 0x0F,
        Request::ReadInputRegisters(_, _) => 0x04,
        Request::ReadHoldingRegisters(_, _) => 0x03,
        Request::WriteSingleRegister(_, _) => 0x06,
        Request::WriteMultipleRegisters(_, _) => 0x10,
        Request::ReadWriteMultipleRegisters(_, _, _, _) => 0x17,
        Request::Custom(code, _) => code,
        _ => 0x00,
    }
}

#[repr(u8)]
#[allow(unused)]
enum Exception {
    IllegalFunction = 0x01,
    IllegalDataAddress = 0x02,
    IllegalDataValue = 0x03,
    ServerDeviceFailure = 0x04,
    Acknowledge = 0x05,
    ServerDeviceBusy = 0x06,
    MemoryParityError = 0x08,
    GatewayPathUnavailable = 0x0A,
    GatewayTargetDevice = 0x0B,
}

#[derive(Debug, Clone, Copy)]
pub struct Models {
    id: SunspecID,
    model1: Model1,
    model213: Model213,
    model_end: ModelEnd,
}

impl Models {
    fn new () -> Models {
        let ret = Models {
            id: SunspecID::new(),
            model1: Model1::new(),
            model213: Model213::new(),
            model_end: ModelEnd::new()
        };
        ret
    }
}

static MODELS: OnceCell<Mutex<Models>> = OnceCell::new();

fn ensure_models() -> &'static Mutex<Models> {
    MODELS.get_or_init(|| Mutex::new(Models::new()))
}

pub fn get_models() -> MutexGuard<'static, Models> {
    ensure_models().lock().unwrap()
}

pub fn set_models(models: Models) {
    *ensure_models().lock().unwrap() = models;
}

impl Service for MbServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future 
    {
        print!("Responding request: {:?}", self);
        match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                
                let models = get_models();
                //println!("{:?}", models);

                let mut registers = Vec::<u16>::from(models.id);
                registers.extend(Vec::<u16>::from(models.model1));
                registers.extend(Vec::<u16>::from(models.model213));
                registers.extend(Vec::<u16>::from(models.model_end));
                drop(models);

                if addr >= SUNSPEC_INIT_ADDR && (addr + cnt) <= (SUNSPEC_INIT_ADDR + (mem::size_of::<Models>() / 2) as u16) {
                    println!("Request addr: {} - cnt: {}", addr, cnt);
                    let registers = &registers[(addr - SUNSPEC_INIT_ADDR) as usize..(cnt + (addr - SUNSPEC_INIT_ADDR)) as usize];
                    future::ready(Ok(Response::ReadHoldingRegisters(registers.to_vec())))
                }else {
                    future::ready(error_response(&req, Exception::IllegalDataAddress))
                }
            }
            _ => future::ready(error_response(&req, Exception::IllegalFunction))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:8443".parse().unwrap();
    let uart_addr = "/dev/ttyACM0";

    ensure_models();

    let mut models_tmp = get_models();

    sunspec::srt_to_vec_u8("Manufactor", &mut models_tmp.model1.manufacturer);
    sunspec::srt_to_vec_u8("Model", &mut models_tmp.model1.model);
    sunspec::srt_to_vec_u8("ABCD1234", &mut models_tmp.model1.serial_number);
    sunspec::srt_to_vec_u8("Options", &mut models_tmp.model1.options);

    models_tmp.model213.a = 12.5;
    models_tmp.model213.hz = 60.05;
    models_tmp.model213.pf = 0.92;
    drop(models_tmp);

/*
    tokio::spawn(Box::pin(async move {
        client_context(uart_addr).await;
    }));
*/
    thread::spawn(move || client_context(uart_addr));
    tokio::select! {
        _ = server_context(socket_addr) => unreachable!(),
        //_ = client_context(uart_addr) => println!("Exiting"),
    }
    //Ok(())
}

//async fn server_context(socket_addr: SocketAddr, models_srv: Arc<tokio::sync::Mutex<Models>>) {
async fn server_context(socket_addr: SocketAddr) {
    println!("Starting up server...");

    let server = server::tls::Server::new(socket_addr);
    server.serve(|| Ok(MbServer)).await.unwrap();
}

/*
async fn client_context(uart_addr: &str) {
    use tokio_serial::SerialStream;

    let tty_path = uart_addr;
    let slave = Slave(0x03);

    let builder = tokio_serial::new(tty_path, 115200);
    let builder = builder.parity(tokio_serial::Parity::Even);
    let port = SerialStream::open(&builder).unwrap();

    let mut ctx = rtu::connect_slave(port, slave).await.unwrap();
    loop {
        println!("Reading a sensor value");
        let rsp = ctx.read_holding_registers(32001, 2).await.unwrap();
        println!("Sensor value is: {rsp:?}");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut models = get_models();
        models.model213.a += 0.1;
        println!("{:?}", models);
        drop(models);

        println!("Reading a sensor value");
        let rsp = ctx.read_holding_registers(32262, 2).await.unwrap();
        println!("Sensor value is: {rsp:?}");
        tokio::time::sleep(Duration::from_secs(1)).await;        
    }
}
*/

fn client_context(uart_addr: &str) {
    let tty_path = uart_addr;
    let slave = Slave(0x03);

    let builder = tokio_serial::new(tty_path, 115200);
    let builder = builder.parity(tokio_serial::Parity::Even);

    let mut ctx = sync::rtu::connect_slave(&builder, slave).unwrap();
    loop {
        println!("Reading a sensor value");
        let rsp = ctx.read_holding_registers(32001, 2).unwrap();
        println!("Sensor value is: {:?}", rsp);
        thread::sleep(Duration::from_secs(1));

        let mut models = get_models();
        models.model213.a += 0.1;
        drop(models);

        println!("Reading a sensor value");
        let rsp = ctx.read_holding_registers(32262, 2).unwrap();
        println!("Sensor value is: {:?}", rsp);
        thread::sleep(Duration::from_secs(1));
    }
}