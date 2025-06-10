use actix_web::{
    delete, get, patch, post, web,
    web::{Json, Path},
    Either, HttpResponse, Responder,
};

use crate::auth_validator::Claims;
use crate::db;
use crate::dto::{ResultId, UpdateResult};
use crate::errors::internal_server_error;
use crate::todos::{CreateTodo, Todo, UpdateTodo};

pub type TodoResponse<T> = Either<Json<T>, HttpResponse>;

#[get("/todos")]
async fn find_all(claims: Option<web::ReqData<Claims>>) -> TodoResponse<Vec<Todo>> {
    match claims {
        Some(_) => match db::todos().await {
            Ok(result) => Either::Left(Json(result)),
            Err(error) => Either::Right(internal_server_error(error)),
        },
        _ => Either::Right(HttpResponse::Unauthorized().body("Unauthorized")),
    }
}

#[get("/todos/{id}")]
async fn find(claims: Option<web::ReqData<Claims>>, path: Path<String>) -> TodoResponse<Todo> {
    let Ok(id) = path.into_inner().parse::<i64>() else {
        return Either::Right(HttpResponse::BadRequest().body("invalid ID"));
    };

    match claims {
        Some(_) => match db::get_todo(id).await {
            Ok(record) => Either::Left(Json(record)),
            Err(error) => Either::Right(internal_server_error(error)),
        },
        _ => Either::Right(HttpResponse::Unauthorized().body("Unauthorized")),
    }
}

#[post("/todos")]
async fn create(claims: Option<web::ReqData<Claims>>, params: Json<CreateTodo>) -> impl Responder {
    match claims {
        Some(_) => {
            let todo_id = db::create_todo(params.into_inner()).await;

            match todo_id {
                Ok(id) => Either::Left(Json(ResultId { id })),
                Err(error) => Either::Right(internal_server_error(error)),
            }
        }
        None => Either::Right(HttpResponse::Unauthorized().body("Unauthorized")),
    }
}

#[patch("/todos/{id}")]
async fn update(
    claims: Option<web::ReqData<Claims>>,
    path: Path<String>,
    params: Json<UpdateTodo>,
) -> impl Responder {
    let Ok(id) = path.into_inner().parse::<i64>() else {
        return Either::Right(HttpResponse::BadRequest().body("invalid ID"));
    };

    match claims {
        Some(_) => match db::update_todo(id, params.into_inner()).await {
            Ok(result) => Either::Left(Json(UpdateResult { result })),
            Err(error) => Either::Right(internal_server_error(error)),
        },
        _ => Either::Right(HttpResponse::Unauthorized().body("Unauthorized")),
    }
}

#[delete("/todos/{id}")]
async fn delete(claims: Option<web::ReqData<Claims>>, path: Path<String>) -> impl Responder {
    let Ok(id) = path.into_inner().parse::<i64>() else {
        return Either::Right(HttpResponse::BadRequest().body("invalid ID"));
    };

    match claims {
        Some(_) => match db::delete_todo(id).await {
            Ok(_) => Either::Left(HttpResponse::NoContent()),
            Err(error) => Either::Right(internal_server_error(error)),
        },
        _ => Either::Right(HttpResponse::Unauthorized().body("Unauthorized")),
    }
}

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
