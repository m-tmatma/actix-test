use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    id: u32,
    name: String,
    price: f64,
}

static mut PRODUCTS: Vec<Product> = Vec::new();

fn get_products_mut() -> &'static mut Vec<Product> {
    unsafe { &mut *std::ptr::addr_of_mut!(PRODUCTS) }
}

async fn get_product_by_id(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let products = get_products_mut();
    if let Some(product) = products.iter().find(|p| p.id == id) {
        HttpResponse::Ok().json(product)
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

async fn create_product(product: web::Json<Product>) -> impl Responder {
    let products = get_products_mut();
    let new_product = product.into_inner();
    println!("POST /proucts - createdct: {:?}", new_product);

    if products.iter().any(|p| p.id == new_product.id) {
        return HttpResponse::Conflict().body("Product with this ID already exists")
    }
    products.push(new_product.clone());
    HttpResponse::Created().json(new_product)
}

async fn update_product(
    path: web::Path<u32>,
    product: web::Json<Product>,
) -> impl Responder {
    let id = path.into_inner();
    let update_product = product.into_inner();
    let products = get_products_mut();
    println!("PUT /products/{} - updated product: {:?}", id, update_product);

    if id != update_product.id {
        return HttpResponse::BadRequest().body("Product ID in path and body do not match")
    }

    if let Some(index) = products.iter().position(|p| p.id == id) {
        products[index] = update_product.clone();
        HttpResponse::Ok().json(update_product)
    } else {
        products.push(update_product.clone());
        HttpResponse::Created().json(update_product)
    }
}

async fn delete_product(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let products = get_products_mut();
    println!("DELETE /products/{} - deleting product", id);

    if let Some(index) = products.iter().position(|p| p.id == id) {
        products.remove(index);
        HttpResponse::NoContent().body("Product deleted")
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}
async fn get_all_products() -> impl Responder {
    let products = get_products_mut();
    HttpResponse::Ok().json(products.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    get_products_mut().push(Product { id: 1, name: "Laptop".to_string(), price: 999.99 });
    get_products_mut().push(Product { id: 2, name: "Mouse".to_string(), price: 25.00 });
    println!("Starting server");
    HttpServer::new(|| App::new()
        .service(web::scope("/products")
            .route("", web::get().to(get_all_products))
            .route("", web::post().to(create_product))
            .route("/{id}", web::get().to(get_product_by_id))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
