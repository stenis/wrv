use js_sys::Uint32Array;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{UsbControlTransferParameters, UsbDevice, UsbDeviceRequestOptions, UsbInTransferResult, UsbOutTransferResult, UsbRecipient, UsbRequestType, UsbTransferStatus};
use serde::{Serialize, Deserialize};

const AIRSPY_RECEIVER_MODE: u8 = 1;
const AIRSPY_GET_SAMPLERATES: u8 = 25;
const AIRSPY_SET_FREQ: u8 = 13;
const RECEIVER_MODE_OFF: u8 = 0;
const RECEIVER_MODE_RX: u8 = 1;

#[repr(u8)]
pub enum ReceiverMode { Off, Rx, }
impl From<ReceiverMode> for u8 {
    fn from(rm: ReceiverMode) -> u8 {
        rm as u8
    }
}
#[derive(Debug)]
pub struct Airspy {
    pub device: UsbDevice,
}

impl Airspy {

    async fn control_transfer_in(&self, setup: &UsbControlTransferParameters, index: u16) -> Result<UsbInTransferResult> {
        Ok(JsFuture::from(self.device.control_transfer_in(setup, index)).await?.dyn_into::<UsbInTransferResult>()?)
    }

    async fn control_transfer_out(&self, setup: &UsbControlTransferParameters) -> Result<UsbOutTransferResult> {
        Ok(JsFuture::from(self.device.control_transfer_out(setup)).await?.dyn_into::<UsbOutTransferResult>()?)
    }

    pub async fn read_samplerates(&self, _log: impl Fn(String) -> ()) -> Result<Vec<u32>> {
        let r1 = self.control_transfer_in(&create_setup(AIRSPY_GET_SAMPLERATES, 0), 4).await?;        
        let len : u16 = (Uint32Array::new(&r1.data().unwrap().buffer()).length() * 4) as u16;
        let r2 = self.control_transfer_in(&create_setup(AIRSPY_GET_SAMPLERATES, len), len * 4).await?;
        let sample_rates = Uint32Array::new(&r2.data().unwrap().buffer()).to_vec();

        Ok(sample_rates)
    }

    pub fn start() {
        todo!()
    }

    pub fn stop() {
        todo!()
    }

    pub async fn set_receiver_mode(&self) -> Result<()>{
        let setup = create_setup(AIRSPY_RECEIVER_MODE, 0);
		let _res = self.control_transfer_out(&setup).await?;
        
        Ok(())
    }
}

fn create_setup(request: u8, index: u16) -> UsbControlTransferParameters {
    UsbControlTransferParameters::new(
        index,
        UsbRecipient::Device,
        request,
        UsbRequestType::Vendor,
        0,
    )
}

pub async fn open_async() -> Result<Airspy> {
    let window = web_sys::window().expect("No global 'window' exists!");
    let navigator: web_sys::Navigator = window.navigator();
    let usb = navigator.usb();
    
    //TODO: try and remove Filters and create array to pass to UsbDeviceRequestOptions::new
    let options = serde_wasm_bindgen::to_value(&Filters::new())?;
    let filters = vec![Filter { vendor_id: 0x1d50 }];

    let x = UsbDeviceRequestOptions::new(&serde_wasm_bindgen::to_value(&filters)?);
    let device: UsbDevice = JsFuture::from(usb.request_device(&x)).await?.dyn_into()?;
    //let device: UsbDevice = JsFuture::from(usb.request_device(&options.into())).await?.dyn_into()?;

    JsFuture::from(device.open()).await?;
    JsFuture::from(device.claim_interface(0)).await?;

    Ok(Airspy { device })
}

type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Browser error")]
    BrowserError(String),
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::BrowserError(format!("{:?}", e))
    }
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::BrowserError(format!("{:?}", e))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "vendorId")]
    pub vendor_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Filters {
    pub filters: Vec<Filter>,
}

impl Filters {
    pub fn new() -> Self {
        Filters{ filters: vec![Filter { vendor_id: 0x1d50 }]}
    }
}
