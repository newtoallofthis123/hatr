use actix_web::{get, HttpResponse, Responder, post, web};

use crate::db::{crud::{add_to_db, get_all}, init::Todo};

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AddReq{
    pub title: String,
    pub completed: String,
}

#[get("/")]
async fn home()-> impl Responder{
    HttpResponse::Ok().body(include_str!("../../files/index.html"))
}

#[get("/all")]
async fn all()-> impl Responder{
    let todos = get_all().await;
    HttpResponse::Ok().body(serde_json::to_string(&todos).unwrap())
}

#[post("/add")]
async fn add(req: web::Json<AddReq>)-> impl Responder{
    let req = Todo{
        id: 0,
        title: req.title.clone(),
        completed: req.completed.parse::<bool>().unwrap(),
    };
    add_to_db(&req).await;
    HttpResponse::Ok().body("Added")
}

#[get("/del/{id}")]
async fn del(doc: web::Path<i32>)-> impl Responder{
    let res = crate::db::crud::delete_from_db(doc.into_inner()).await;
    if !res{
        return HttpResponse::Ok().body(format!("Not Found"));
    }
    HttpResponse::Ok().body(format!("Deleted"))
}

#[post("/edit/{id}")]
async fn edit(doc: web::Path<i32>, req: web::Json<AddReq>)-> impl Responder{
    let req = Todo{
        id: doc.into_inner(),
        title: req.title.clone(),
        completed: req.completed.parse::<bool>().unwrap(),
    };
    let res = crate::db::crud::edit_from_db(&req).await;
    if !res{
        return HttpResponse::Ok().body(format!("Not Found"));
    }
    HttpResponse::Ok().body(format!("Edited"))
}

#[get("/complete/{id}")]
async fn complete(doc: web::Path<i32>)-> impl Responder{
    let res = crate::db::crud::set_completed(doc.into_inner()).await;
    if !res{
        return HttpResponse::Ok().body(format!("Not Found"));
    }
    HttpResponse::Ok().body(format!("Completed"))
}