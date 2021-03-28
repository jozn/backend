// Requires chromedriver running on port 4444:
// > chromedriver --port=4444


// insta_chrome is not used right now; in future maybe we need this script and for now
// we just keep it.

use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() {
    insta().await;
}

async fn insta() {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    // color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await.unwrap();
    // Navigate to https://wikipedia.org.
    driver.get("https://www.instagram.com/accounts/login/").await.unwrap();
    let co = driver.find_elements(By::Tag("button")).await.unwrap();
    for c in co {
        if c.text().await.unwrap() == "Accept" {
            println!("it's workding Accpet");
            c.click().await;
            break;
        }
    }

    // set Email
    let elems_form = driver.find_elements(By::Tag("input")).await.unwrap();
/*    for elem in elems_form {
        if let Ok(res_opt) = elem.get_attribute("name").await {
            match res_opt {
                None => {}
                Some(val) => {
                    if val == "username" {
                        // elem.send_keys("mailproxy30@gmail.com").await;
                        break;
                    }
                }
            }
        }
    }*/

    // Set Email
    let elems_form = driver.find_element(By::Id("loginForm")).await.unwrap();
    let username_from = elems_form.find_element(By::Name("username")).await.unwrap();
    let password_from = elems_form.find_element(By::Name("password")).await.unwrap();

    username_from.send_keys("webspycat@gmail.com").await;
    password_from.send_keys("insta12345678").await;

    // set Email
    let elems_form = elems_form.find_elements(By::Tag("button")).await.unwrap();
    for elem in elems_form {
        if let Ok(res_opt) = elem.get_attribute("type").await {
            println!("res >>>>> {:?}", res_opt);
            match res_opt {
                None => {}
                Some(val) => {
                    if val == "submit" {
                        elem.click().await;
                        break;
                    }
                }
            }
        }
    }

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let m = driver.get("https://www.instagram.com/pugloulou/?__a=1").await.unwrap();
    let t = driver.find_element(By::Tag("pre")).await.unwrap();
    let txt = t.text().await.unwrap();

    println!(">>>>>>> Json >>>>>> {:}", txt);


    tokio::time::sleep(std::time::Duration::from_secs(5588)).await;

}

async fn wiki() {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    // color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await.unwrap();
    // Navigate to https://wikipedia.org.
    driver.get("https://wikipedia.org").await.unwrap();
    let elem_form = driver.find_element(By::Id("search-form")).await.unwrap();

    // Find element from element.
    let elem_text = elem_form.find_element(By::Id("searchInput")).await.unwrap();

    // Type in the search terms.
    elem_text.send_keys("selenium").await.unwrap();

    // Click the search button.
    let elem_button = elem_form.find_element(By::Css("button[type='submit']")).await.unwrap();
    elem_button.click().await.unwrap();

    // Look for header to implicitly wait for the page to load.
    driver.find_element(By::ClassName("firstHeading")).await.unwrap();
    assert_eq!(driver.title().await.unwrap(), "Selenium - Wikipedia");

    // Ok(())
}