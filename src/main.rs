use std::env;
struct ListItem {
    name: String,
    quantity: u32,
    complete: bool,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let instruction = match args.get(1) {
        Some(value) => value,
        None => {
            eprintln!("No instruction found");
            return;
        }
    };
    // create the vector to store items
    let mut list_items = vec![
        ListItem {
            name: "Bread".to_string(),
            quantity: 1,
            complete: false,
        },
        ListItem {
            name: "Bananas".to_string(),
            quantity: 3,
            complete: true,
        },
    ];
    match instruction.as_str() {
        "add" => {
            let item_name = match args.get(2) {
                Some(value) => value,
                None => {
                    eprintln!("No item found - try again");
                    return;
                }
            };
            let item = ListItem {
                name: item_name.clone(),
                quantity: 1,
                complete: false,
            };
            list_items.push(item);

            println!("Adding: {}", item_name);
            print_todos(&list_items);
        }
        "list" => {
            // Display the list
            println!("Listing todos");
            print_todos(&list_items);
        }
        "done" => match parse_todo_number(&args) {
            Ok(index_to_mark) => match mark_done(&mut list_items, index_to_mark) {
                Ok(()) => print_todos(&list_items),
                Err(message) => eprintln!("{message}"),
            },
            Err(message) => eprintln!("{message}"),
        },
        _ => println!("Unknown command. Usage: todo <add|list|done>"),
    }
}

fn print_todos(list_items: &Vec<ListItem>) {
    for (i, item) in list_items.iter().enumerate() {
        // borrow the list to read it
        if item.complete {
            println!("{}. [x] {}", i + 1, item.name)
        } else {
            println!("{}. [ ] {}", i + 1, item.name)
        }
    }
}
fn mark_done(list_items: &mut Vec<ListItem>, index_to_mark: usize) -> Result<(), String> {
    if index_to_mark <= list_items.len() && index_to_mark > 0 {
        list_items[index_to_mark - 1].complete = true;
        Ok(())
    } else {
        Err(format!(
            "you entered {index_to_mark}, please select a number between 1 and {}",
            list_items.len()
        ))
    }
}
fn parse_todo_number(args: &[String]) -> Result<usize, String> {
    let value = args
        .get(2)
        .ok_or("Missing todo number, e.g. todo done 2".to_string())?;
    let index_to_mark = value
        .parse::<usize>()
        .map_err(|e| format!("'{value}' is not a number. ({e})"))?;
      std::result::Result::Ok(index_to_mark)
}
