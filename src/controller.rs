use crate::{
    ip::{AppState, QueryOptions, Ip},
    response::{GenericResponse, SingleIpResponse, IpData, IpListResponse},
};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use redis::Commands;

#[get("/healthcheck")]
async fn health_checker_controller() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/ips.txt")]
pub async fn ips_txt_controller(
    data: web::Data<AppState>,
) -> impl Responder {
    let ips = data.ip_db.lock().unwrap();


    let mut con = data.redis_con.lock().unwrap();
    let it = con.scan().expect("Error during scan");
    let ip_r_txt : String = it.collect::<Vec<String>>().join("\n");

    let ip_txt : String = ips.clone()
                             .into_iter()
                             .map(|ip| {format!("{};",ip.ipv4)})
                             .collect::<Vec<String>>()
                             .join("\n");

    HttpResponse::Ok().body(format!("{:?} \n == Redis == \n {:?}",
                                    ip_txt,
                                    ip_r_txt))
}


#[get("/ips")]
pub async fn ips_list_controller(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let ips = data.ip_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let ips: Vec<Ip> = ips.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = IpListResponse {
        status: "success".to_string(),
        results: ips.len(),
        ips,
    };
    HttpResponse::Ok().json(json_response)
}

#[post("/ips")]
async fn create_ip_controller(
    body: web::Json<Ip>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.ip_db.lock().unwrap();

    let ip = vec.iter().find(|ip| ip.ipv4 == body.ipv4);


    let mut con = data.redis_con.lock().unwrap();
    let r : bool =  con.exists(&body.ipv4).expect("Error during exist");
    println!("Exist: {:?}",r);
    if r {
        let _ : bool = con.set(&body.ipv4, &body.desc).expect("Error during set");
    } else {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("ERROR REDIS {:?}", r),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    if ip.is_some() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Ip with ipv4: '{}' already exists", body.ipv4),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let ip = body.to_owned();

    vec.push(body.into_inner());

    let json_response = SingleIpResponse {
        status: "success".to_string(),
        data: IpData { ip },
    };

    HttpResponse::Ok().json(json_response)
}


#[delete("/ips/{ipv4}")]
async fn delete_ip_controller(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.ip_db.lock().unwrap();

    let ipv4 = path.into_inner();
    let ip = vec.iter_mut().find(|ip| ip.ipv4 == ipv4.to_owned());

    if ip.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Ip with IPV4: {} not found", ipv4),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    vec.retain(|ip| ip.ipv4 != ipv4.to_owned());
    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_controller)
        .service(ips_list_controller)
        .service(ips_txt_controller)
        .service(create_ip_controller)
        .service(delete_ip_controller);

    conf.service(scope);
}
