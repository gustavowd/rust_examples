
use std::io::Write;
use std::mem;

#[derive(Debug, Clone, Copy)]
pub struct SunspecID {
    pub id: [u8; 4]
}


#[derive(Debug, Clone, Copy)]
pub struct Model1 {
    pub model_number: u16,
    pub qtd: u16,
    pub manufacturer: [u8; 32],
    pub model: [u8; 32],
    pub options: [u8; 16],
    pub version: [u8; 16],
    pub serial_number: [u8; 32],
    pub device_address: u16,
    pub pad: u16
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum Evt {
    NONE = 0,
    NULL1 = 1,
    NULL2 = 1 << 1,
    MEVENTPowerFailure = 1 << 2,
    MEVENTUnderVoltage = 1 << 3,
    MEVENTLowPF = 1 << 4,
    MEVENTOverCurrent = 1 << 5,
    MEVENTOverVoltage = 1 << 6,
    MEVENTMissingSensor = 1 << 7,
    MEVENTReserved1 = 1 << 8,
    MEVENTReserved2 = 1 << 9,
    MEVENTReserved3 = 1 << 10,
    MEVENTReserved4 = 1 << 11,
    MEVENTReserved5 = 1 << 12,
    MEVENTReserved6 = 1 << 13,
    MEVENTReserved7 = 1 << 14,
    MEVENTReserved8 = 1 << 15,
    MEVENTOEM01 = 1 << 16,
    MEVENTOEM02 = 1 << 17,
    MEVENTOEM03 = 1 << 18,
    MEVENTOEM04 = 1 << 19,
    MEVENTOEM05 = 1 << 20,
    MEVENTOEM06 = 1 << 21,
    MEVENTOEM07 = 1 << 22,
    MEVENTOEM08 = 1 << 23,
    MEVENTOEM09 = 1 << 24,
    MEVENTOEM010 = 1 << 25,
    MEVENTOEM011 = 1 << 26,
    MEVENTOEM012 = 1 << 27,
    MEVENTOEM013 = 1 << 28,
    MEVENTOEM014 = 1 << 29,
    MEVENTOEM015 = 1 << 30,
    NULL3 = 1 << 31
}

#[derive(Debug, Clone, Copy)]
pub struct Model213 {
    pub model_number: u16,
    pub qtd: u16,
    pub a: f32,
    pub apha: f32,
    pub aphb: f32,
    pub aphc: f32,
    pub phv: f32,
    pub phvpha: f32,
    pub phvphb: f32,
    pub phvphc: f32,
    pub ppv: f32,
    pub ppvphab: f32,
    pub ppvphbc: f32,
    pub ppvphca: f32,
    pub hz: f32,
    pub w: f32,
    pub wpha: f32,
    pub wphb: f32,
    pub wphc: f32,
    pub va: f32,
    pub vapha: f32,
    pub vaphb: f32,
    pub vaphc: f32,
    pub var: f32,
    pub varpha: f32,
    pub varphb: f32,
    pub varphc: f32,
    pub pf: f32,
    pub pfpha: f32,
    pub pfphb: f32,
    pub pfphc: f32,
    pub totwhexp: f32,
    pub totwhexppha: f32,
    pub totwhexpphb: f32,
    pub totwhexpphc: f32,
    pub totwhimp: f32,
    pub totwhimppha: f32,
    pub totwhimpphb: f32,
    pub totwhimpphc: f32,
    pub totvahexp: f32,
    pub totvahexppha: f32,
    pub totvahexpphb: f32,
    pub totvahexpphc: f32,
    pub totvahimp: f32,
    pub totvahimppha: f32,
    pub totvahimpphb: f32,
    pub totvahimpphc: f32,
    pub totvarhimpq1: f32,
    pub totvarhimpq1pha: f32,
    pub totvarhimpq1phb: f32,
    pub totvarhimpq1phc: f32,
    pub totvarhimpq2: f32,
    pub totvarhimpq2pha: f32,
    pub totvarhimpq2phb: f32,
    pub totvarhimpq2phc: f32,
    pub totvarhimpq3: f32,
    pub totvarhimpq3pha: f32,
    pub totvarhimpq3phb: f32,
    pub totvarhimpq3phc: f32,
    pub totvarhimpq4: f32,
    pub totvarhimpq4pha: f32,
    pub totvarhimpq4phb: f32,
    pub totvarhimpq4phc: f32,
    pub evt: Evt
}

#[derive(Debug, Clone, Copy)]
pub struct ModelEnd {
    pub model_number: u16,
    pub qtd: u16
}

// Declare the struct
pub trait Sunspec {
    // This new function acts as a constructor
    // allowing us to add additional logic to instantiating a struct
    // This particular method belongs to the trait
    fn new () -> Self;
   // Signature of other functions that belong to this trait
    // we include a mutable version of the struct in birthday
    //fn birthday(&mut self);
    //fn sound (&self);
}

impl Sunspec for SunspecID {
    fn new () -> SunspecID {
        let mut ret = SunspecID {
            id: [0; 4],
        };
        srt_to_vec_u8("SunS", &mut ret.id);
        return ret;
    }
}

impl Sunspec for Model1 {
    fn new () -> Model1 {
        let ret = Model1 {
            model_number: 1,
            qtd: 66,
            manufacturer: [0; 32],
            model: [0; 32],
            options: [0; 16],
            version: [0; 16],
            serial_number: [0; 32],
            device_address: 0,
            pad: 0,
        };
        return ret;
    }
}

impl Sunspec for Model213 {

    fn new () -> Model213 {
        let ret = Model213 {
            model_number: 213,
            qtd: 124,
            a: 0.0,
            apha: 0.0,
            aphb: 0.0,
            aphc: 0.0,
            phv: 0.0,
            phvpha: 0.0,
            phvphb: 0.0,
            phvphc: 0.0,
            ppv: 0.0,
            ppvphab: 0.0,
            ppvphbc: 0.0,
            ppvphca: 0.0,
            hz: 0.0,
            w: 0.0,
            wpha: 0.0,
            wphb: 0.0,
            wphc: 0.0,
            va: 0.0,
            vapha: 0.0,
            vaphb: 0.0,
            vaphc: 0.0,
            var: 0.0,
            varpha: 0.0,
            varphb: 0.0,
            varphc: 0.0,
            pf: 0.0,
            pfpha: 0.0,
            pfphb: 0.0,
            pfphc: 0.0,
            totwhexp: 0.0,
            totwhexppha: 0.0,
            totwhexpphb: 0.0,
            totwhexpphc: 0.0,
            totwhimp: 0.0,
            totwhimppha: 0.0,
            totwhimpphb: 0.0,
            totwhimpphc: 0.0,
            totvahexp: 0.0,
            totvahexppha: 0.0,
            totvahexpphb: 0.0,
            totvahexpphc: 0.0,
            totvahimp: 0.0,
            totvahimppha: 0.0,
            totvahimpphb: 0.0,
            totvahimpphc: 0.0,
            totvarhimpq1: 0.0,
            totvarhimpq1pha: 0.0,
            totvarhimpq1phb: 0.0,
            totvarhimpq1phc: 0.0,
            totvarhimpq2: 0.0,
            totvarhimpq2pha: 0.0,
            totvarhimpq2phb: 0.0,
            totvarhimpq2phc: 0.0,
            totvarhimpq3: 0.0,
            totvarhimpq3pha: 0.0,
            totvarhimpq3phb: 0.0,
            totvarhimpq3phc: 0.0,
            totvarhimpq4: 0.0,
            totvarhimpq4pha: 0.0,
            totvarhimpq4phb: 0.0,
            totvarhimpq4phc: 0.0,
            evt: Evt::NONE,
        };
        return ret;
    }
}

impl Sunspec for ModelEnd {

    fn new () -> ModelEnd {
        let ret = ModelEnd {
            model_number: 0xFFFF,
            qtd: 0,
        };
        return ret;
    }
}

fn vec_u8_to_vec_u16(src: &[u8], dst: &mut Vec<u16>, mut idx: usize, size: usize) -> usize {
    for i in (0..size).step_by(2) {
        dst[idx] = (src[i] as u16) << 8;
        dst[idx] += src[i+1] as u16;
        idx += 1;
    }
    idx
}

fn f32_to_vec_u16(src: f32, dst: &mut Vec<u16>, idx: usize) {
    let tmp = src.to_bits();
    dst[idx] = (tmp >> 16) as u16;
    dst[idx+1] = (tmp & 0xFFFF) as u16;
}

fn u32_to_vec_u16(src: u32, dst: &mut Vec<u16>, idx: usize) {
    dst[idx] = (src >> 16) as u16;
    dst[idx+1] = (src & 0xFF) as u16;
}

pub fn srt_to_vec_u8(src: &str, mut dst: &mut [u8]){
    dst.write(src.as_bytes()).unwrap();
}

impl From<SunspecID> for Vec<u16> {
    fn from(from: SunspecID) -> Self {
        let size = mem::size_of::<SunspecID>() / 2;
        let mut registers: Vec<u16> = vec![0; size];
        
        vec_u8_to_vec_u16(&from.id, &mut registers, 0, from.id.len());
        registers
    }
}

impl From<Model1> for Vec<u16> {
    fn from(from: Model1) -> Self {
        let size = mem::size_of::<Model1>() / 2;
        let mut registers: Vec<u16> = vec![0; size];
        registers[0] = from.model_number;
        registers[1] = from.qtd;
        
        let mut idx = vec_u8_to_vec_u16(&from.manufacturer, &mut registers, 2, from.manufacturer.len());
        idx = vec_u8_to_vec_u16(&from.model, &mut registers, idx, from.model.len());
        idx = vec_u8_to_vec_u16(&from.options, &mut registers, idx, from.options.len());
        idx = vec_u8_to_vec_u16(&from.version, &mut registers, idx, from.version.len());
        idx = vec_u8_to_vec_u16(&from.serial_number, &mut registers, idx, from.serial_number.len());
        registers[idx] = from.device_address;
        registers[idx+1] = from.pad;
        registers
    }
}

impl From<Model213> for Vec<u16> {
    fn from(mut from: Model213) -> Self {
        let size = mem::size_of::<Model213>() / 2;
        let mut registers: Vec<u16> = vec![0; size];
        registers[0] = from.model_number;
        registers[1] = from.qtd;
        let pointer = &mut from.a as *mut f32;

        let mut j = 0;
        for i in (2..size-2).step_by(2) {
            f32_to_vec_u16(unsafe{*pointer.offset(j)}, &mut registers, i);
            j += 1;
        }

        u32_to_vec_u16(from.evt as u32, &mut registers, 124);
        registers
    }
}

impl From<ModelEnd> for Vec<u16> {
    fn from(from: ModelEnd) -> Self {
        let size = mem::size_of::<ModelEnd>() / 2;
        let mut registers: Vec<u16> = vec![0; size];
        registers[0] = from.model_number;
        registers[1] = from.qtd;
        registers
    }
}
