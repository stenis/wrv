use js_sys::{Uint16Array,  Uint32Array};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    UsbControlTransferParameters, 
    UsbDevice, 
    UsbDeviceFilter, 
    UsbDeviceRequestOptions, 
    UsbInTransferResult, 
    UsbOutTransferResult, 
    UsbRecipient, 
    UsbRequestType,
};

const AIRSPY_RECEIVER_MODE: u8 = 1;
const AIRSPY_GET_SAMPLERATES: u8 = 25;
const AIRSPY_SET_FREQ: u8 = 13;
const RECEIVER_MODE_OFF: u16 = 0;
const RECEIVER_MODE_RX: u16 = 1;

#[derive(Debug)]
pub struct Airspy {
    pub device: UsbDevice,
}

pub async fn open_async() -> Result<Airspy> {
    let window = web_sys::window().expect("No global 'window' exists!");
    let navigator: web_sys::Navigator = window.navigator();
    let usb = navigator.usb();
    
    let mut filter =  UsbDeviceFilter::new();
    filter.vendor_id(0x1d50);
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, filter.into());
    let filters = UsbDeviceRequestOptions::new(&arr.into());
    let device: UsbDevice = JsFuture::from(usb.request_device(&filters)).await?.dyn_into()?;
    
    JsFuture::from(device.open()).await?;
    JsFuture::from(device.select_configuration(1)).await?;
    JsFuture::from(device.claim_interface(0)).await?;

    Ok(Airspy { device })
}

impl Airspy {
    pub async fn start(&self)-> Result<()> {
        self.set_receiver_mode(RECEIVER_MODE_RX).await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        JsFuture::from(self.device.release_interface(0)).await?;
        JsFuture::from(self.device.close()).await?; 
        Ok(())
    }

    pub async fn read_samplerates(&self) -> Result<Vec<u32>> {
        let res = self.control_transfer_in(&Setup::new(AIRSPY_GET_SAMPLERATES), 4).await?;        
        let index = (Uint32Array::new(&res.data().unwrap().buffer()).length() * 4) as u16; // index is length in this context.
        let res = self.control_transfer_in(&Setup::index(AIRSPY_GET_SAMPLERATES, index), index * 4).await?;
        let sample_rates = Uint32Array::new(&res.data().unwrap().buffer()).to_vec();

        Ok(sample_rates)
    }

    pub async fn read_bulk(&self, length: u32) -> Result<Vec<u16>> {
        let result = JsFuture::from(self.device.transfer_in(1, length)).await?.dyn_into::<UsbInTransferResult>()?;
        let buffer =  Uint16Array::new(&result.data().unwrap().buffer()).to_vec();
        Ok(buffer)
    }

    pub async fn set_freq(&self, value: u32) -> Result<UsbOutTransferResult> {
        let data = Uint32Array::new_with_length(1);
        data.set(&value.into(), 0);
        let setup = Setup::new(AIRSPY_SET_FREQ);
		let res = 
            JsFuture::from(self.device.control_transfer_out_with_buffer_source(&setup, &data))
            .await?.dyn_into::<UsbOutTransferResult>()?;
        
        Ok(res)
    }

    pub async fn start_rx(&self) -> Result<()> {
        self.set_receiver_mode(RECEIVER_MODE_OFF).await?;
        JsFuture::from(self.device.clear_halt(web_sys::UsbDirection::In, 1)).await?;
        self.set_receiver_mode(RECEIVER_MODE_RX).await?;
        Ok(())
    }

    async fn set_receiver_mode(&self, value: u16) -> Result<UsbOutTransferResult>{
        Ok(self.control_transfer_out(&Setup::value(AIRSPY_RECEIVER_MODE, value)).await?)
    }

    async fn control_transfer_in(&self, setup: &UsbControlTransferParameters, index: u16) -> Result<UsbInTransferResult> {
        Ok(JsFuture::from(self.device.control_transfer_in(setup, index)).await?.dyn_into::<UsbInTransferResult>()?)
    }

    async fn control_transfer_out(&self, setup: &UsbControlTransferParameters) -> Result<UsbOutTransferResult> {
        Ok(JsFuture::from(self.device.control_transfer_out(setup)).await?.dyn_into::<UsbOutTransferResult>()?)
    }
}

#[derive(Default)]
struct Setup {
    value: u16,
    request: u8,
    index: u16,
}

impl Setup {
    pub fn new (request: u8) -> UsbControlTransferParameters {
        to_usb_control_transfer_parameters(Setup { request, ..Default::default() })
    }

    pub fn index (request: u8, index: u16) -> UsbControlTransferParameters {
        to_usb_control_transfer_parameters(Setup { request, index, ..Default::default() })
    }

    pub fn value (request: u8, value: u16) -> UsbControlTransferParameters {
        to_usb_control_transfer_parameters(Setup { request, value, ..Default::default() })
    }
}

fn to_usb_control_transfer_parameters(setup: Setup) -> UsbControlTransferParameters {
    UsbControlTransferParameters::new(
        setup.index,
        UsbRecipient::Device,
        setup.request,
        UsbRequestType::Vendor,
        setup.value,
    )
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