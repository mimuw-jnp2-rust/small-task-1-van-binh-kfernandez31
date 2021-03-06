use std::fmt::{Display, Formatter};
use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dish {
    ThaiChicken,
    Tofu,
    FriedRice,
}

impl Dish {
    fn price(&self) -> u32 {
        match self {
            Dish::ThaiChicken => 20,
            Dish::Tofu => 15,
            Dish::FriedRice => 12,
        }
    }
}

const TAKEAWAY_FEE: u32 = 1;

#[derive(Debug, Clone)]
struct Order {
    takeaway: bool,
    dishes: Vec<Dish>,
}

impl Order {
    fn new() -> Order {
        Order {
            takeaway: false,
            dishes: Vec::new(),
        }
    }

    fn add_dish(&mut self, dish: Dish) {
        self.dishes.push(dish);
    }

    fn set_takeaway(&mut self) {
        self.takeaway = true;
    }

    //TODO: czy nie jest to brzydkie, że castuję?
    fn dish_count(&self, dish: Dish) -> u32 {
        self.dishes.iter().filter(|&elem| *elem == dish).count() as u32
    }

    fn items_count(&self) -> u32 {
        self.dishes.len() as u32
    }

    fn is_takeaway(&self) -> bool {
        self.takeaway
    }

    fn total(&self) -> u32 {
        let sum = self.dishes.iter().fold(0, |cnt, elem| cnt + elem.price());

        if self.is_takeaway() {
            sum + self.items_count() * TAKEAWAY_FEE
        } else {
            sum
        }
    }

    fn is_empty(&self) -> bool {
        self.items_count() == 0
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "chicken: {}, tofu: {}, rice: {}, takeaway: {}",
            self.dish_count(Dish::ThaiChicken),
            self.dish_count(Dish::Tofu),
            self.dish_count(Dish::FriedRice),
            self.is_takeaway()
        )
    }
}

struct Customer {
    name: String,
    favorite_order: Order,
}

struct VanBinh {
    orders_count: u32,
    customers: Vec<Customer>,
}

impl VanBinh {
    pub fn new() -> VanBinh {
        VanBinh {
            orders_count: 1,
            customers: Vec::new(),
        }
    }

    fn add_customer(&mut self, name: String, favorite_order: Order) {
        self.customers.push(Customer {
            name,
            favorite_order,
        });
    }

    /*    fn add_favorite_order(&mut self, name: &String, new_favorite: Order) {
        let customer = self.customers
            .swap_remove(self.customers
                .iter().position(|c| c.name == *name)
                .unwrap()); //FIXME: this sort of fails
        self.add_customer(customer.name, new_favorite)
    }*/

    fn get_saved_customer(&self, name: &str) -> Option<&Customer> {
        self.customers.iter().find(|c| c.name == name)
    }

    fn increase_orders_count(&mut self) {
        self.orders_count += 1;
    }

    fn get_orders_count(&self) -> u32 {
        self.orders_count
    }
}

fn get_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

fn yes_no(question: &str) -> bool {
    println!("{} (y/n)", question);
    get_line() == "y"
}

fn get_order() -> Order {
    let mut order = Order::new();
    loop {
        println!("Enter dish name or empty line to finish:");
        let line = get_line();
        if line.is_empty() {
            break;
        }
        if line.contains("chicken") {
            order.add_dish(Dish::ThaiChicken);
        } else if line.contains("tofu") {
            order.add_dish(Dish::Tofu);
        } else if line.contains("rice") {
            order.add_dish(Dish::FriedRice);
        } else {
            println!("Unknown dish name: {}", line);
        }
    }

    if yes_no("Takeaway?") {
        order.set_takeaway();
    }

    order
}

fn main() {
    let mut van_binh = VanBinh::new();

    loop {
        println!("Hi! Welcome to Van Binh! What's your name?");
        let name = get_line();

        if name.is_empty() {
            break;
        }

        let order = if let Some(customer) = van_binh.get_saved_customer(&name) {
            println!("Welcome back, {}!", customer.name);
            if yes_no("Same as usual?") {
                //TODO: is cloning here okay?
                customer.favorite_order.clone() // use customer's favorite order
            } else {
                get_order()
            }
        } else {
            println!("Welcome, {}!", name);
            let order = get_order();
            if yes_no("Would you like to save this order?") {
                //TODO: is cloning here okay?
                van_binh.add_customer(name, order.clone()); // save customer's favorite order in van_binh struct
            }
            order
        };

        // Check if the order is empty
        if order.is_empty() {
            println!("Your order is empty!");
        } else {
            println!("This is order no. {}", van_binh.get_orders_count());
            println!(
                "There you go: {}, it's going to be {} zł",
                order,
                order.total()
            );
            van_binh.increase_orders_count();
        }
    }
    println!("Bye!");
}
