use crate::handler::model::{AppState, CreateTask, CreateUser, Task, TaskAction, User};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Jai Mata Di Server is Working Fine";
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/get_tasks/{user_id}")]
pub async fn get_tasks(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(Task, "SELECT * FROM task WHERE user_id = $1", user_id,)
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(tasks) => {
            let task_response = serde_json::json!({
                "status": "success",
                "Totals Task": tasks.len(),
                "data": serde_json::json!({ "tasks": tasks }),
            });

            HttpResponse::Ok().json(task_response)
        }
        Err(e) => {
            eprintln!("Error fetching tasks: {:?}", e);
            let message = format!("Failed to fetch tasks");
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": message,
            }))
        }
    }
}

#[get("/get_all_users")]
pub async fn get_all_users(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(User, "SELECT * FROM public.user",)
        .fetch_all(&data.db)
        .await
    {
        Ok(users) => {
            let user_response = serde_json::json!({
                "status": "success",
                "Total Users" : users.len(),
                "data": serde_json::json!({ "users": users }),
            });

            HttpResponse::Ok().json(user_response)
        }
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            let message = format!("Failed to fetch users");
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": message,
            }))
        }
    }
}

#[get("/get_all_tasks")]
pub async fn get_all_tasks(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(Task, "SELECT * FROM task")
        .fetch_all(&data.db)
        .await
    {
        Ok(tasks) => {
            let json_response = serde_json::json!({
                "status": "success",
                "results": tasks.len(),
                "tasks": tasks,
            });
            HttpResponse::Ok().json(json_response)
        }
        Err(e) => {
            eprintln!("Error fetching tasks: {:?}", e);
            let message = "Failed to fetch tasks";
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": message,
            }))
        }
    }
}

#[post("/add_task")]
pub async fn add_task(body: web::Json<CreateTask>, data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        User,
        "SELECT * FROM public.user WHERE user_id = $1 ",
        body.user_id.to_string(),
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => {
            sqlx::query!(
                "UPDATE public.user SET task_remain = $1 WHERE user_id = $2",
                user.task_remain + 1,
                user.user_id.to_string()
            )
            .execute(&data.db)
            .await
            .map_err(|e| {
                eprintln!("Error updating user: {:?}", e);
                HttpResponse::InternalServerError().finish()
            })
            .unwrap();
            
            let query_result = sqlx::query_as!(
                Task,
                "INSERT INTO task (user_id, task_name) VALUES ($1, $2) RETURNING *",
                body.user_id.to_string(),
                body.task_name.to_string(),
            )
            .fetch_one(&data.db)
            .await;

            match query_result {
                Ok(task) => HttpResponse::Ok().json(json!({
                    "status": "success",
                    "data": { "task": task },
                })),
                Err(e) => {
                    if e.to_string().contains("duplicate key value violates unique constraint") {
                        HttpResponse::BadRequest().json(json!({
                            "status": "fail",
                            "message": "Task with that ID already exists",
                        }))
                    } else {
                        HttpResponse::InternalServerError().json(json!({
                            "status": "error",
                            "message": format!("{:?}", e),
                        }))
                    }
                }
            }
        }
        Err(e) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": format!("{:?}", e),
        })),
    }
}

#[post("/add_user")]
pub async fn add_user(body: web::Json<CreateUser>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO public.user (user_id,user_name) VALUES ($1, $2) RETURNING *",
        body.user_id.to_string(),
        body.user_name.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(task) => {
            let task_response = serde_json::json!({
                "status": "success",
                "data": { "task": task },
            });

            HttpResponse::Ok().json(task_response)
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                HttpResponse::BadRequest().json(json!({
                    "status": "fail",
                    "message": "task with that title already exists",
                }))
            } else {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                }))
            }
        }
    }
}

#[post("/task_done")]
pub async fn task_done(body: web::Json<TaskAction>, data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM public.user WHERE user_id = $1",
        body.user_id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(user) => {
            let query_result = sqlx::query_as!(
                Task,
                "UPDATE task SET is_done = 1 WHERE task_id = $1 AND user_id = $2",
                body.task_id,
                body.user_id.to_string(),
            )
            .execute(&data.db)
            .await;

            match query_result {
                Ok(_) => {
                    let update_user = sqlx::query_as!(
                        User,
                        "UPDATE public.user SET task_done = $1, task_remain = $2 WHERE user_id = $3",
                        user.task_done + 1,
                        user.task_remain - 1,
                        user.user_id
                    )
                    .execute(&data.db)
                    .await;

                    match update_user {
                        Ok(_) => HttpResponse::Ok().json(json!({
                            "status": "Success"
                        })),
                        Err(e) => HttpResponse::InternalServerError().json(json!({
                            "status": "error",
                            "message": format!("{:?}", e)
                        })),
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": format!("{:?}", e)
                    }))
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            }))
        }
    }
}

#[get("/get_user/{user_id}")]
pub async fn get_user(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM public.user WHERE user_id = $1 ",
        user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => HttpResponse::Ok().json(json!({
            "status" : "Success",
            "data" : user
        })),
        Err(e) => HttpResponse::BadRequest().json(json!(
            {
                "status" : "Fail",
                "message" : format!("{:?}",e)
            }
        )),
    }
}

#[post("/delete_task")]
pub async fn delete_task(body: web::Json<TaskAction>, data: web::Data<AppState>) -> impl Responder {
    // Fetch the user by user_id
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM public.user WHERE user_id = $1",
        body.user_id.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(mut user) => {
            // Fetch the task to determine if it is done
            let task_result = sqlx::query_as!(
                Task,
                "SELECT * FROM task WHERE task_id = $1 AND user_id = $2",
                body.task_id,
                body.user_id.to_string(),
            )
            .fetch_optional(&data.db)
            .await;

            // Check if the task exists and if it is done
            if let Ok(Some(task)) = task_result {
                if task.is_done == 1 {
                    user.task_done -= 1;
                } else {
                    user.task_remain -= 1;
                }

                // Execute the delete query for the task
                let _ = sqlx::query!(
                    "DELETE FROM task WHERE task_id = $1 AND user_id = $2",
                    body.task_id,
                    body.user_id.to_string(),
                )
                .execute(&data.db)
                .await;

                // Execute the update query for the user
                let _ = sqlx::query!(
                    "UPDATE public.user SET task_done = $1, task_remain = $2 WHERE user_id = $3",
                    user.task_done,
                    user.task_remain,
                    body.user_id.to_string(),
                )
                .execute(&data.db)
                .await;

                return HttpResponse::Ok().body("Task deleted successfully");
            } else {
                // Task not found
                return HttpResponse::NotFound().json(json!({
                    "status": "fail",
                    "message": "Task not found",
                }));
            }
        }
        Err(e) => {
            // User not found
            return HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": format!("User not found: {:?}", e),
            }));
        }
    }
}
