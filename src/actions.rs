use crate::error::Result;
use headless_chrome::browser::tab::point::Point;
use std::thread;
use std::time::Duration;

pub fn wttj_board_action(tab: &headless_chrome::Tab) -> Result<()> {
    let element = tab.wait_for_element("button#axeptio_btn_dismiss")?;
    element.click()?;
    let point = Point { x: 600.0, y: 190.0 };
    tab.click_point(point)?;
    tab.press_key(" ")?;
    let element = tab.wait_for_element("div[data-testid='place-item-0'] div")?;
    element.click()?;
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

pub fn hellowork_board_action(tab: &headless_chrome::Tab) -> Result<()> {
    let element = tab.wait_for_element("button#hw-cc-notice-accept-btn")?;
    element.click()?;
    Ok(())
}
