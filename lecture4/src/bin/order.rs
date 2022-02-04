use chrono::{DateTime, Utc};

#[allow(dead_code)]
#[derive(Debug)]
enum OrderStatus {
    Processing,
    Packed,
    Shipped { tracking_id: usize, ship_time: DateTime<Utc>},
    Delivered(String)
}

fn print_status(status: OrderStatus) {
    match status {
        OrderStatus::Processing => println!("Order processing"),
        OrderStatus::Packed => println!("Order packed"),
        OrderStatus::Shipped { tracking_id, ship_time}=> println!("order was shipped at {:?} with tracking id {}", ship_time, tracking_id),
        OrderStatus::Delivered(message) => println!("Order delivered at {}", message)
    }
}

/// Return a tuple indicating the expected time until arrival
// #[allow(dead_code)]
// fn estimate_arrival(status: OrderStatus) -> (u8, u8) {
//     match status {
//         OrderStatus::Processing => (7, 14),
//         OrderStatus::Packed => (5, 7),
//         OrderStatus::Shipped => (3, 5),
//         OrderStatus::Delivered => (0, 0)
//     }
// }

fn get_status() -> OrderStatus {
    OrderStatus::Shipped{tracking_id: 123, ship_time: Utc::now()}
}

fn main() {
    let status = get_status();

    let status_string: String = format!("{:?}", status);
    print_status(status);
    println!("{}", status_string);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding() {
        assert_eq!(add(1, 2), 3);
        // assert_ne!(add(1, 2), 3);
        assert!(add(1, 2) == 3);
    }
}