use thirtyfour_sync::{error::WebDriverError, Keys::{Enter, Shift, Home, Backspace}, prelude::*};
use device_query::{DeviceQuery, DeviceState, Keycode};
use clipboard::{ClipboardProvider, ClipboardContext};
use std::process::Command;
use crate::main;

pub fn google() -> Result<(), WebDriverError> {
    let mut chrome = Command::new("./chromedriver").arg("--port=4444").spawn()?;
    let browser = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &browser)?;
    driver.get("https://www.google.com")?;

    let device_state = DeviceState::new();
        
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::A) {
            let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();
            let main_element = driver.find_element(By::Name("q"))?;
            
            main_element.send_keys(Shift + Home + Backspace)?;
            main_element.send_keys(clip.get_contents().unwrap())?;
            main_element.send_keys(Enter)?;
        }
        
        else if keys.contains(&Keycode::Escape) {
            driver.close()?;
            chrome.kill()?;
            main();
        }
        Ok::<(), WebDriverError>(())?;
    }
}