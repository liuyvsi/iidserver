use actix_web::{web, App, HttpServer, HttpResponse,Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    message: String,
    status: i32,
    data: Option<Value>, // Modified data type
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse2 {
    message: String,
    status: i32,
    data: Option<TokenData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenData {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IdentityResponse {
    message: String,
    status: i32,
    data: Option<Value>, // Modified data type
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCreateRequest {
    items: Vec<BatchCreateItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCreateItem {
    handle: String,
    template_version: String,
    value: Vec<BatchCreateValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCreateValue {
    data: BatchCreateData,
    auth: String,
    index: i32,
    #[serde(rename = "type")]
    data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCreateData {
    format: String,
    value: String,
}

async fn handle_login(login_request: web::Json<LoginRequest>) -> HttpResponse {
    if login_request.username == "admin" && login_request.password == "******" {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IcCI63YS1jYTAyLTRmZmMtYTE5My1jYjUzYmFmMmU3MGIifQ";
        let response = TokenResponse {
            message: "success".to_string(),
            status: 1,
            data: Some(json!({ "token": token.to_string() })),
        };
        HttpResponse::Ok().json(response)
    } else {
        let response = TokenResponse {
            message: "用户名/密码错误！".to_string(),
            status: -2,
            data: None,
        };
        HttpResponse::BadRequest().json(response)
    }
}

async fn handle_identity_query(query: web::Query<IdentityQuery>) -> HttpResponse {
     if query.handle.is_empty() {
        let response = IdentityResponse {
            message: "缺少必要的请求参数: handle".to_string(),
            status: -2,
            data: None,
        };
        return HttpResponse::BadRequest().json(response);
    }

    // Simulated identity query logic
    let identity_data = json!({
        "code": "200",
        "prefix": "88.1021.150",
        "handle": "88.1021.150/002",
        "template_version": "1.0",
        "value": [
            {
                "data": {
                    "format": "string",
                    "value": "1.0",
                },
                "auth": "1",
                "index": 1001,
                "type": "TEMPLATE",
            },
        ],
    });

    let response = IdentityResponse {
        message: "success".to_string(),
        status: 1,
        data: Some(identity_data),
    };
    HttpResponse::Ok().json(response)
}

async fn handle_batch_create(req_body: web::Json<BatchCreateRequest>) -> HttpResponse {
    let mut response_data = Vec::new();

    for item in &req_body.items {
        let msg = if item.handle == "88.101.0042/001" {
            "success"
        } else {
            "标识已存在"
        };

        response_data.push(json!({
            "handle": item.handle.clone(),
            "msg": msg,
        }));
    }

    let response = TokenResponse {
        message: "success".to_string(),
        status: 1,
        data: Some(json!(response_data)),
    };

    HttpResponse::Ok().json(response)
}

#[derive(Debug, Deserialize)]
struct IdentityQuery {
    handle: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeleteRequest {
    handle: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct BatchDeleteResponse {
    handle: String,
    code: i32,
    msg: String,
}
async fn handle_delete(data: web::Json<DeleteRequest>) -> impl Responder {
    let handle_value = &data.handle;

    // Modifying to ensure that the response data matches the expected type.
    let response_data = if handle_value == "88.1021.150/xx002" {
        Some(TokenData {
            token: "example_token".to_string(), // Replace with appropriate token value
        })
    } else {
        None
    };

    let response = TokenResponse2 {
        message: if response_data.is_some() { "success" } else { "没有找到标识" }.to_string(),
        status: if response_data.is_some() { 1 } else { -2 },
        data: response_data,
    };

    HttpResponse::Ok().json(response)
}

async fn handle_batch_delete(data: web::Json<Vec<String>>) -> HttpResponse {
    let mut response_data = Vec::new();

    for handle in &data.0 {
        if handle == "86.709.612/cc" {
            response_data.push(BatchDeleteResponse {
                handle: handle.to_string(),
                code: 1,
                msg: "success".to_string(),
            });
        } else {
            response_data.push(BatchDeleteResponse {
                handle: handle.to_string(),
                code: -2,
                msg: "标识不存在".to_string(),
            });
        }
    }

    let response = TokenResponse2 {
        message: "success".to_string(),
        status: 1,
        data: Some(TokenData { token: "dummy_token".to_string() }), // Placeholder token data
    };

    HttpResponse::Ok().json(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateQuery {
    prefix: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateData {
    id: u32,
    prefix: String,
    name: String,
    version: String,
    industryCategory: String,
    industrySpecific: String,
    industryTrade: String,
    industrySubclass: String,
    #[serde(rename = "type")]
    template_type: u32,
    description: String,
    items: Vec<TemplateItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateItem {
    name: String,
    idType: String,
    metadataId: u32,
    metadata: Metadata,
    required: bool,
    state: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    r#type: String,
    minLength: u32,
    maxLength: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateResponse {
    message: String,
    status: i32,
    data: Option<TemplateData>,
}

async fn handle_template_inquiry(query: web::Query<TemplateQuery>) -> impl Responder {
    println!("Received template inquiry request: {:?}\n",query);
    if query.prefix.is_empty() || query.version.is_empty() {
        let response = TemplateResponse {
            message: "缺少必要的请求参数: prefix 或 version".to_string(),
            status: -2,
            data: None,
        };
        return HttpResponse::BadRequest().json(response);
    }

    // Simulated template inquiry logic
    let template_data = TemplateData {
        id: 177,
        prefix: "88.101.5".to_string(),
        name: "".to_string(),
        version: "1.0.1".to_string(),
        industryCategory: "A".to_string(),
        industrySpecific: "01".to_string(),
        industryTrade: "011".to_string(),
        industrySubclass: "0111".to_string(),
        template_type: 1,
        description: "模板1.0.1".to_string(),
        items: vec![
            TemplateItem {
                name: "e1".to_string(),
                idType: "ip".to_string(),
                metadataId: 124,
                metadata: Metadata {
                    r#type: "string".to_string(),
                    minLength: 1,
                    maxLength: 10,
                },
                required: false,
                state: 0,
            },
        ],
    };

    let response = TemplateResponse {
        message: "success".to_string(),
        status: 1,
        data: Some(template_data),
    };
    println!("send template inquiry response: {:?}",response);
    HttpResponse::Ok().json(response)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/identity/token").route(web::post().to(handle_login)))
            .service(web::resource("/identityv2/data/detail").route(web::get().to(handle_identity_query)))
            .service(web::resource("/snms/template").route(web::get().to(handle_template_inquiry)))
            .service(web::resource("/identityv2/data/batchCreate").route(web::post().to(handle_batch_create)))
            .service(web::resource("/identityv2/data").route(web::delete().to(handle_delete)))
            .service(web::resource("/identityv2/data/batchDelete").route(web::delete().to(handle_batch_delete)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
