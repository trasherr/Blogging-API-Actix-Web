use actix_web::{get, post, web};
use chrono::NaiveDateTime;
use chrono::Utc;
use sea_orm::{EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use crate::utils::{api_response, app_state, jwt::Claims};

#[derive(Serialize,Deserialize)]
struct CreatePostModel {
    title: String,
    text: String
}

#[derive(Serialize,Deserialize)]
struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub user: Option<UserModel>
}
#[derive(Serialize,Deserialize)]
struct UserModel {
    name: String,
    email: String
}

#[post("create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: web::Json<CreatePostModel>
) -> Result<api_response::ApiResponse,api_response::ApiResponse> {

    let post_entity = entity::post::ActiveModel 
    {  
        title: Set(post_model.title.clone()), 
        text: Set(post_model.text.clone()), 
        uuid: Set(Uuid::new_v4()), 
        user_id: Set(claim.id), 
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    post_entity.insert(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?;


    Ok(api_response::ApiResponse::new(200, "Post Created".to_owned()))
}

#[get("my-posts")]
pub async fn my_posts(
    app_state: web::Data<app_state::AppState>,
    claim: Claims
)-> Result<api_response::ApiResponse,api_response::ApiResponse> {

    let posts: Vec<PostModel> = entity::post::Entity::find()
    .filter(entity::post::Column::UserId.eq(claim.id)).all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?
    .into_iter()
    .map(|post| 
        PostModel { 
            id: post.id, 
            title: post.title, 
            text: post.text, 
            uuid: post.uuid, 
            image: post.image, 
            user_id: post.user_id, 
            created_at: post.created_at,
            user: None
        }  
    ).collect();
    let res_str = serde_json::to_string(&posts)
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}



#[get("all-posts")]
pub async fn all_posts(
    app_state: web::Data<app_state::AppState>,
)-> Result<api_response::ApiResponse,api_response::ApiResponse> {

    let posts: Vec<PostModel> = entity::post::Entity::find()
    .all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?
    .into_iter()
    .map(|post| 
        PostModel { 
            id: post.id, 
            title: post.title, 
            text: post.text, 
            uuid: post.uuid, 
            image: post.image, 
            user_id: post.user_id, 
            created_at: post.created_at,
            user:None
        }  
    ).collect();
    let res_str = serde_json::to_string(&posts)
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}


#[get("post/{post_uuid}")]
pub async fn one_posts(
    app_state: web::Data<app_state::AppState>,
    post_uuid: web::Path<Uuid>
)-> Result<api_response::ApiResponse,api_response::ApiResponse> {

    let posts: PostModel = entity::post::Entity::find()
    .filter(entity::post::Column::Uuid.eq(post_uuid.clone()))
    .find_also_related(entity::user::Entity)
    .one(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?
    .map(|post| 
        PostModel { 
            id: post.0.id, 
            title: post.0.title, 
            text: post.0.text, 
            uuid: post.0.uuid, 
            image: post.0.image, 
            user_id: post.0.user_id, 
            created_at: post.0.created_at,
            user: post.1.map(|item| UserModel { name: item.name, email: item.email } )
        }  
    )
    .ok_or(api_response::ApiResponse::new(404,"No Post Found".to_string()))?;

    let res_str = serde_json::to_string(&posts)
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}