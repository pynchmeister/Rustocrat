use crate::dao::{Dao, Member, Proposal};
use std::sync::{Arc, Mutex};
use warp::Filter;

type SharedDao = Arc<Mutex<Dao>>;

pub async fn create_member(dao: SharedDao, member: Member) -> Result<impl warp::Reply, warp::Rejection> {
    let mut dao = dao.lock().await;
    dao.add_member(member.address.clone(), member.voting_power);
    Ok(warp::reply::json(&member))
}

pub async fn create_proposal(dao: SharedDao, proposal: Proposal) -> Result<impl warp::Reply, warp::Rejection> {
    let mut dao = dao.lock().await;
    dao.add_proposal(proposal.clone());
    Ok(warp::reply::json(&proposal))
}

pub fn with_dao(dao: SharedDao) -> impl Filter<Extract = (SharedDao,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || dao.clone())
}

