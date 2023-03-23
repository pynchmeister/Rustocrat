use super::handlers::{create_member, create_proposal, with_dao};
use crate::dao::{Dao, Member, Proposal};
use std::sync::{Arc, Mutex};
use warp::{self, Filter, Reply};

pub fn get_routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let dao = Arc::new(Mutex::new(Dao::new()));

    let create_member_route = warp::path!("members")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_dao(dao.clone()))
        .and_then(create_member);

    let create_proposal_route = warp::path!("proposals")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_dao(dao.clone()))
        .and_then(create_proposal);

    create_member_route.or(create_proposal_route)
}
