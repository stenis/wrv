use js_sys::Uint32Array;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{UsbControlTransferParameters, UsbDevice, UsbDeviceFilter, UsbDeviceRequestOptions, UsbInTransferResult, UsbOutTransferResult, UsbRecipient, UsbRequestType};

const AIRSPY_RECEIVER_MODE: u16 = 1;
const AIRSPY_GET_SAMPLERATES: u8 = 25;
const AIRSPY_SET_FREQ: u8 = 13;
const RECEIVER_MODE_OFF: u8 = 0;
const RECEIVER_MODE_RX: u8 = 1;

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
        let setup = create_setup(Setup { request: AIRSPY_GET_SAMPLERATES, ..Default::default() });
        let res = self.control_transfer_in(&setup, 4).await?;        
        let index : u16 = (Uint32Array::new(&res.data().unwrap().buffer()).length() * 4) as u16; // index is length in this context.
        let setup = create_setup(Setup { request: AIRSPY_GET_SAMPLERATES, index, ..Default::default() });
        let res = self.control_transfer_in(&setup, index * 4).await?;
        let sample_rates = Uint32Array::new(&res.data().unwrap().buffer()).to_vec();

        Ok(sample_rates)
    }

    pub async fn start(&self)-> Result<()> {
        self.set_receiver_mode(AIRSPY_RECEIVER_MODE).await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        JsFuture::from(self.device.release_interface(0)).await?;
        JsFuture::from(self.device.close()).await?; 
        Ok(())
    }

    pub async fn set_receiver_mode(&self, value: u16) -> Result<()>{
        let setup = create_setup(Setup { value, ..Default::default() });
		let _ = self.control_transfer_out(&setup).await?;
        Ok(())
    }
}

#[derive(Default)]
struct Setup {
    value: u16,
    request: u8,
    index: u16,
}

fn create_setup(setup: Setup) -> UsbControlTransferParameters {
    UsbControlTransferParameters::new(
        setup.index,
        UsbRecipient::Device,
        setup.request,
        UsbRequestType::Vendor,
        setup.value,
    )
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