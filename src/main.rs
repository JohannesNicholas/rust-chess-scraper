use std::{borrow::BorrowMut, error::Error, option};

use thirtyfour::prelude::*;

//a struct to keep the data of a piece
struct Piece {
    piece_type: String,
    square_x: u8,
    square_y: u8,
    dragged: bool,
    x_pos: f32,
    y_pos: f32,
}

fn u8_from_char(c: &Option<char>) -> Option<u8> {
    let c = *c;
    c?.to_digit(10)?.try_into().ok()
}

fn square_from_class(class: &String) -> Option<(u8,u8)> {

    let mut chars = class.chars();

    let second_last = u8_from_char(&chars.nth(class.len()-2));
    let last = u8_from_char(&chars.last());

    match last {
        Some(last) => {
            match second_last {
                Some(second_last) => {
                    return Some((second_last, last))
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

fn extract_piece_from_classes(classes: String) -> Option<Piece> {

    let mut piece = Piece {
        piece_type: "".to_string(),
        square_x: 0,
        square_y: 0,
        dragged: false,
        x_pos: 0.0,
        y_pos: 0.0,
    };

    let classes_split = classes.split_whitespace();

    
    let mut count: i8 = 0;
    for (index, class) in classes_split.enumerate() {
        count += 1;
        
        match index {
            0 => {
                if class != "piece" {
                    return None;
                }
            },
            1 => {
                piece.piece_type = class.to_string();
            },
            2 => {
                match square_from_class(&class.to_string()) {
                    Some((x,y)) => {
                        piece.square_x = x;
                        piece.square_y = y;
                    },
                    None => {
                        return None;
                    }
                }
            },
            3 => {
                if class == "dragging" {
                    piece.dragged = true;
                }
            }
            _ => {}
        }
    }
    if count < 3 {
        return None;
    }

    Some(piece)
}

fn set_piece_position(mut piece: Piece, transform: String) -> Piece{
    let mut transform = transform.split("matrix(").nth(1);

    match transform {
        Some(t) => {
            transform = t.split(")").nth(0);
        },
        None => {}
    }

    
    match transform {
        Some(transform) => {
            let mut transform = transform.split(", ");
            let x = transform.nth(4);
            let y = transform.next();

            match x {
                Some(x) => {
                    
                    piece.x_pos = x.parse().unwrap_or(0.0);
                },
                None => {}
            }

            match y {
                Some(y) => {
                    piece.y_pos = y.parse().unwrap_or(0.0);
                },
                None => {}
            }
        },
        None => {}
    }

    piece
}

async fn extract_piece_data(elem: WebElement) -> Option<Piece> {

    let mut piece: Option<Piece> = None;

    //get all of the classes of the element
    match elem.attr("class").await.ok()? {
        Some(classes) => {
            piece = extract_piece_from_classes(classes);
        },
        None => {
            return None
        }
    }

    //get the transform css property
    match elem.css_value("transform").await.ok() {
        Some(transform) => {
            match piece {
                Some(piece_guarantee) => {
                    piece = Some(set_piece_position(piece_guarantee, transform));
                },
                None => {}
            }
        },
        None => {}
    }

    return piece

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
            println!(
                "Piece type: {}, Square: {}{}, dragging: {}, x_pos: {}, y_pos: {}", 
                piece.piece_type, 
                piece.square_x, 
                piece.square_y,
                piece.dragged,
                piece.x_pos,
                piece.y_pos
            );
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