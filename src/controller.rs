use crate::{
    ip::{AppState, QueryOptions, Ip},
    response::{GenericResponse, SingleIpResponse, IpData, IpListResponse},
};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use redis::Commands;


const EXPIRATION_IN_SECONDS :usize = 60*60*24; // 24h

#[get("/healthcheck")]
async fn health_checker_controller() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}


fn get_ttl(key: &String, con: &mut redis::Connection) -> usize {
    let mut expiry : usize = con.ttl(key).unwrap_or(0);
    if (expiry as u64) == std::u64::MAX {
       expiry = 0;
    }
    expiry
}

#[get("/ips.txt")]
pub async fn ips_txt_controller(
    data: web::Data<AppState>,
) -> impl Responder {

    let c = data.redis_con.lock().expect("Error retrieving Redis client");
    let mut con = c.get_connection().expect("Error trying to connect to Redis {:?}");



    let it = con.scan().expect("Error during scan");
    let ips = it.collect::<Vec<String>>();

    let mut ip_txt : String = ips.clone()
                             .into_iter()
                             .map(|ip| {format!("{};# {}",ip, get_ttl(&ip,&mut con))})
                             .collect::<Vec<String>>()
                             .join("\n");

    ip_txt.push(';');

    HttpResponse::Ok().body(ip_txt)
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

    let c = data.redis_con.lock().expect("Error retrieving Redis client");
    let mut con = c.get_connection().expect("Error trying to connect to Redis {:?}");

    let r : bool =  con.exists(&body.ipv4).unwrap_or_else(|err| {
        println!("Error trying to test if key exist in Redis {:?}",err);
        false
    });

    println!("Exist: {:?}",r);
    if !r {
        if con.set(&body.ipv4, &body.desc).expect("Error during set") {
            println!("Set ok ");

            if body.expire {
                // EXPIRE KEY
                let _ : String = con.get_ex(&body.ipv4,redis::Expiry::EX(EXPIRATION_IN_SECONDS)).expect("Error during expire");
            }
        } else {
            println!("Set non ok");
        }
    } else {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Key exists {:?}", r),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let ip = body.to_owned();


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
