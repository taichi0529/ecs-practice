use warp::Filter;
use std::collections::HashMap;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: i32,
    name: String,
    price: i32,
}

fn item_factory(id: i32, name: &str, price: i32) -> Item {
    Item {
        id,
        name: String::from(name),
        price,
    }
}

fn add_item (items: &mut HashMap<String, Item>, id: i32, name: &str, price: i32) {
    items.insert(String::from(id.to_string()), item_factory(id, name, price));
}

#[tokio::main]
async fn main() {


    let mut items: HashMap<String, Item>= HashMap::new();
    add_item(&mut items, 1,"みかん", 30);
    add_item(&mut items, 2,"バナナ", 100);
    add_item(&mut items, 3,"みかん", 150);

    let items2 = items.clone(); // 非同期でborrowすると上手く行かないのでcloneしてmoveする・・・。


    let route = warp::path!("items" / String).map(move |id| {
        let n = String::from(id);
        if let Some(item) = items.get(&n) {
            return warp::reply::with_status(
                warp::reply::json(&item),
                warp::http::StatusCode::OK,
            );
        }
        warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::NOT_FOUND,
        )
    }).or(warp::path!("items").map(move || {
        let mut all_items: Vec<&Item> = Vec::new();
        for i in items2.values() {
            all_items.push(i);
        }
        warp::reply::with_status(
            warp::reply::json(&all_items),
            warp::http::StatusCode::OK,
        )
    }));

    // Serverの起動
    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}