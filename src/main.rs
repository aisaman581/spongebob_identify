#[macro_use] extern crate rocket;

use image::DynamicImage;



#[get("/")]
fn index() -> &'static str {
    "Can you Identify the Spongebob Birthday Invitaions?"
}

#[get("/abela")]
fn abela(image: rocket::State<DynamicImage>) -> &'static str{
    "Who is this invitation from?"
}

#[launch]
fn rocket() -> _ {
    let image = image::open("D:/Random Ass Pics/Spongebob Birthday Cards/abela.png").unwrap();
    rocket::build().manage(image);
    rocket::build().mount("/", routes![index]);
    rocket::build().mount("/abela", routes![abela])
}


