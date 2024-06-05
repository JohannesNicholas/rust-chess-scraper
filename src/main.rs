use std::error::Error;

use thirtyfour::prelude::*;

//a struct to keep the data of a piece
struct Piece {
    piece_type: String,
    square: u8,
    dragged: bool,
    x_pos: f32,
    y_pos: f32,
}

fn extract_piece_type_from_classes(classes: &String) -> Option<String> {
    match classes.split_whitespace().nth(1) {
        Some(piece_type) => {
            Some(piece_type.to_string())
        },
        None => {
            None
        }
    }
}

fn extract_square_from_classes(classes: &String) -> Option<u8> {
    let classes = classes.split_whitespace();

    for class in classes {
        if class.starts_with("square-") {
            match class.split("-").nth(1) {
                Some(square) => {
                    return square.parse::<u8>().ok();
                },
                None => {
                    return None;
                }
            } 
        }
    }
    None
}

fn extract_piece_from_classes(classes: String) -> Option<Piece> {

    

    for (i, class) in classes.split_whitespace().enumerate() {
        if (i == 1) {
            
        }
    }

    //get the piece type
    let piece_type = extract_piece_type_from_classes(&classes);
    //get the square
    let square = extract_square_from_classes(&classes);
    //create a piece struct
    match piece_type {
        Some(piece_type) => {
            match square {
                Some(square) => {
                    Some(Piece {
                        piece_type: piece_type,
                        square: square,
                        dragged: false,
                        x_pos: 0.0,
                        y_pos: 0.0,
                    })
                },
                None => {
                    None
                }
            }
        },
        None => {
            None
        }
    }
}

async fn extract_piece_data(elem: WebElement) -> Option<Piece> {

    //get all of the classes of the element
    match elem.attr("class").await {
        Ok(classes) => {
            match classes {
                Some(classes) => {
                    return extract_piece_from_classes(classes)
                },
                None => {
                    None
                }
            }
        },
        Err(_) => {
            None
        }
    }

}

async fn get_pieces_positions(driver: &WebDriver) -> Vec<Piece> {
    // Find the element
    let elems = driver.find_all(By::ClassName("piece")).await;

    
    let pieces = match elems {
        Ok(elems) => {
            let mut pieces: Vec<Piece> = Vec::new();
            for elem in elems {
                match extract_piece_data(elem).await {
                    Some(piece) => {
                        pieces.push(piece);
                    },
                    None => {
                        continue;
                    }
                }
            }
            pieces
        },
        Err(_) => Vec::new(),
    };

    pieces
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    if false {
        // Navigate to https://wikipedia.org.
        driver.goto("https://wikipedia.org").await?;
        let elem_form = driver.find(By::Id("search-form")).await?;
    
        // Find element from element.
        let elem_text = elem_form.find(By::Id("searchInput")).await?;
    
        // Type in the search terms.
        elem_text.send_keys("selenium").await?;
    
        // Click the search button.
        let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
        elem_button.click().await?;
    
        // Look for header to implicitly wait for the page to load.
        driver.query(By::ClassName("firstHeading")).first().await?;
        assert_eq!(driver.title().await?, "Selenium - Wikipedia");
    }




    driver.goto("https://www.chess.com/play/computer").await?;

    //Create a loop to continuously check if the element is present
    loop {

        let pieces = get_pieces_positions(&driver).await;

        //print the pieces
        for piece in pieces {
            println!("Piece type: {}, Square: {}", piece.piece_type, piece.square);
        }
        

        //wait for a second
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        //if chrome is no longer running, break the loop
        if driver.title().await.is_err() {
            println!("Browser closed. Ending the program.");
            break;
        }

    }

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}