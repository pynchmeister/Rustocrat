use clap::{App, Arg, SubCommand};
use dao_framework::dao::{Dao, Member, Proposal};
use std::sync::{Arc, Mutex};
use warp::{self, Filter, Reply};
use dao_framework::api::routes;

type SharedDao = Arc<Mutex<Dao>>;

#[tokio::main]
async fn main() {
    let matches = App::new("Rustocrat")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Decentralized Autonomous Organization (DAO) Framework")
        .subcommand(
            SubCommand::with_name("create_member")
                .about("Create a new member")
                .arg(Arg::with_name("address").required(true).help("Member address"))
                .arg(Arg::with_name("voting_power").required(true).help("Member voting power")),
        )
        .subcommand(
            SubCommand::with_name("create_proposal")
                .about("Create a new proposal")
                .arg(Arg::with_name("id").required(true).help("Proposal ID"))
                .arg(Arg::with_name("title").required(true).help("Proposal title"))
                .arg(Arg::with_name("description").required(true).help("Proposal description"))
                .arg(Arg::with_name("amount").required(true).help("Proposal amount"))
                .arg(Arg::with_name("beneficiary").required(true).help("Proposal beneficiary")),
        )
        .get_matches();

    let dao = Arc::new(Mutex::new(Dao::new()));

    // API routes
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

    let api_routes = create_member_route.or(create_proposal_route);

    // CLI commands
    if let Some(matches) = matches.subcommand_matches("create_member") {
        let address = matches.value_of("address").unwrap().to_string();
        let voting_power = matches.value_of("voting_power").unwrap().parse::<u32>().unwrap();
        dao.lock().unwrap().add_member(address, voting_power);
        println!("Member added!");
    } else if let Some(matches) = matches.subcommand_matches("create_proposal") {
        let id = matches.value_of("id").unwrap().parse::<u32>().unwrap();
        let title = matches.value_of("title").unwrap().to_string();
        let description = matches.value_of("description").unwrap().to_string();
        let amount = matches.value_of("amount").unwrap().parse::<f64>().unwrap();
        let beneficiary = matches.value_of("beneficiary").unwrap().to_string();

        let proposal = Proposal {
            id,
            title,
            description,
            amount,
            beneficiary,
            votes_for: 0,
            votes_against: 0,
        };
        dao.add_proposal(proposal);
        println!("Proposal added!");
    }
}
