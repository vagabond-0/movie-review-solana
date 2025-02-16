use actix_cors::Cors;
use actix_web::{web::{Data, Json, Path as pa}, App, Error, HttpResponse, HttpServer, Responder, http, get, post, put, delete};
use mongodb::bson::oid::ObjectId;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::ServiceConfig;

mod db;
use db::MongoRepo;
mod models;
use models::movie::Movie;

#[get("/getallmovies")]
async fn getmovies(db: Data<MongoRepo>) -> HttpResponse {
    match db.get_all_movies().await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/add_movies")]
async fn add_movies(db: Data<MongoRepo>, new_movie: Json<Movie>) -> HttpResponse {
    let data = Movie {
        id: None,
        Movie_title: new_movie.Movie_title.clone(),
        Description: new_movie.Description.clone(),
        imdb: new_movie.imdb.clone(),
        img_link: new_movie.img_link.clone(),
    };

    let movie_detail = db.create_user(data).await;
    match movie_detail {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/update_movie/{id}")]
async fn update_movie(db: Data<MongoRepo>, path: pa<String>, new_movie: Json<Movie>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let object_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId format"),
    };

    let data = Movie {
        id: Some(object_id),
        Movie_title: new_movie.Movie_title.clone(),
        Description: new_movie.Description.clone(),
        imdb: new_movie.imdb.clone(),
        img_link: new_movie.img_link.clone(),
    };

    let update_result = db.update_Movie(&id, data).await;
    match update_result {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/delete_movie/{id}")]
async fn delete_movie(db: Data<MongoRepo>, path: pa<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }
    let result = db.delete_movie(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Movie successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Movie with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let factory = move |cfg: &mut ServiceConfig| {
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);

        let app = App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .service(getmovies)
            .service(add_movies)
            .service(update_movie)
            .service(delete_movie);

        cfg.service(app);
    };

    Ok(factory.into())
}
