use lazy_static::lazy_static;
use scraper::{Html, Selector};
use undetected_chromedriver::{chrome, By, WebDriver};

/// Represents information about a skater.
struct Skater {
    url: String,          // URL профиля
    name: String,         // ФИО спортсмена
    region: String,       // Регион
    birthday: String,     // Дата рождения
}

lazy_static! {
    static ref ROW_SELECTOR: Selector = Selector::parse("#tablepress-25058 > tbody > tr").unwrap();
}

/// Clicks the next button to navigate to the next page.
///
/// ## Arguments
///
/// * `driver`: A reference to the WebDriver.
///
/// ## Returns
///
/// Returns `Result<bool, Box<dyn std::error::Error>>`. Ok with true if the next page is successfully navigated,
/// otherwise returns Ok with false if the next button is disabled or an error if any error occurs.
async fn click_next_button(driver: &WebDriver) -> Result<bool, Box<dyn std::error::Error>> {
    // Find the next button element
    let next_button = driver.find(By::ClassName("paginate_button.next")).await?;
    let class_name = next_button.class_name().await?;

    // Click the next button if it's not disabled
    if !class_name.unwrap().contains("disabled") {
        driver.execute(r#"document.getElementById("tablepress-25058_next").click()"#, Vec::new()).await?;
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        Ok(true)
    } else {
        Ok(false)
    }
}


/// The main entry point of the program.
///
/// This function navigates through the skaters' information pages,
/// parses the data, and writes it into a CSV file.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Launch Chrome WebDriver
    let driver = chrome().await.unwrap();

    // Navigate to the skaters' page
    driver.goto("https://allskaters.info/skaters/rus/").await?;

    // Maximize the window
    driver.fullscreen_window().await.unwrap();

    let mut skaters = Vec::new();

    // Iterate through the years
    for year in 2010..=2015 {
        let year_xpath = format!("//select[@class='widget-1']/option[@value='{}']", year);
        let year_select = driver.find(By::XPath(&year_xpath)).await?;
        year_select.click().await?;
    }

    // Loop through the pages
    loop {
        // Parsing skaters from the current page
        let html = Html::parse_document(&driver.source().await?);
        let rows = html.select(&ROW_SELECTOR);

        for row in rows {
            let class_name = row.value().attr("class").unwrap_or("");
            if class_name.starts_with("row-") {
                // Parse skater's name
                let name_selector = Selector::parse("td.column-1 a").unwrap();
                let name_element = row.select(&name_selector).next().unwrap();
                let name_text = name_element.text().collect::<Vec<_>>().join(" ");
                let name_parts: Vec<&str> = name_text.split('|').collect();
                let name = name_parts[1].trim().to_string();

                // Parse skater's URL
                let url = name_element.value().attr("href").unwrap().to_string();

                // Parse skater's region
                let region_selector = Selector::parse("td.column-5").unwrap();
                let region = row.select(&region_selector).next().unwrap().text().collect::<Vec<_>>().join(" ");

                // Parse skater's birthday
                let birthday_selector = Selector::parse("td.column-7, td.column-8").unwrap();
                let birthday_parts: Vec<String> = row.select(&birthday_selector)
                    .map(|node| node.text().collect::<String>())
                    .collect();
                let birthday = format!("{} {} {}", birthday_parts[0], birthday_parts[1], row.select(&Selector::parse("td.column-2").unwrap()).next().unwrap().text().collect::<String>());

                let skater = Skater {
                    url,
                    name,
                    region,
                    birthday,
                };

                skaters.push(skater);
            }
        }

        let has_next_page = click_next_button(&driver).await?;
        if !has_next_page {
            break;
        }
    }

    // Quit the WebDriver
    driver.quit().await?;

    // Create the CSV output file
    let path = std::path::Path::new("skaters.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // Append the header to the CSV
    writer
        .write_record(&["ФИО", "Дата рождения", "Регион", "Ссылка на профиль"])
        .unwrap();

    // Populate the output file
    for skater in &skaters {
        let url = skater.url.as_str();
        let name = skater.name.as_str();
        let region = skater.region.as_str();
        let birthday = skater.birthday.as_str();
        writer.write_record(&[name, birthday, region, url]).unwrap();
    }

    // Free up the writer resources
    writer.flush().unwrap();

    Ok(())
}