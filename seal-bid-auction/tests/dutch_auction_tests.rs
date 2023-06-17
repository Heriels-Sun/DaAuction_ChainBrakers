use auction_io::auction::{Action, Error, Event};
use gstd::{ActorId, Encode};
use gtest::{Log, System};
mod routines;
use routines::*;

#[test]
fn bid() {
    let sys = System::new();

    let auction = init(&sys);

    let nft_program = sys.get_program(2);
    let token_id: u64 = 0;
    let result = auction.send_with_value(USERS[1], Action::Bid, 1_050_000_000);

    let decoded_reply = result.decoded_log::<String>();
    println!("{:#?}", decoded_reply);

    assert!(result.contains(&(
        USERS[1],
        Ok::<Event, Error>(Event::Offered {
            offered_price: 1_050_000_000,
        })
        .encode()
    )));

    let buyer_balance = sys.balance_of(USERS[1]);

    assert_eq!(buyer_balance, 950_000_000);
}

#[test]
fn buy_two_times() {
    let sys = System::new();

    let auction = init(&sys);
    auction.send_with_value(USERS[1], Action::Bid, 1_000_000_000);
    let result = auction.send_with_value(USERS[2], Action::Bid, 1_000_000_000);
    println!("{:?}", result.decoded_log::<Result<Event, Error>>());
    assert!(result.contains(&(
        USERS[2],
        Ok::<Event, Error>(Event::Offered {
            offered_price: 1_000_000_000,
        })
        .encode()
    )));
}


#[test]
fn bid_with_less_money() {
    let sys = System::new();

    let auction = init(&sys);
    let result = auction.send_with_value(USERS[1], Action::Bid, 999_000_000);

    let decoded_reply = result.decoded_log::<String>();
    println!("{:#?}", decoded_reply);

    assert!(result.contains(&(
        USERS[1],
        Err::<Event, Error>(Error::InsufficientMoney).encode()
    )));
}

#[test]
fn create_auction_twice_in_a_row() {
    let sys = System::new();

    let auction = init(&sys);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);

    assert!(result.contains(&(
        USERS[1],
        Err::<Event, Error>(Error::AlreadyRunning).encode()
    )));
}

#[test]
fn create_auction_twice_after_time_and_stop() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    let owner_user = USERS[0];
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);
    println!("{:?}", result.decoded_log::<Result<Event, Error>>());

    let result = auction.send(owner_user, Action::ForceStop);

    assert!(result.contains(&(
        owner_user,
        Ok::<Event, Error>(Event::AuctionStopped {
            token_owner: owner_user.into(),
            token_id: 0.into(),
        })
        .encode()
    )));
}

#[test]
fn create_auction_with_low_price() {
    let sys = System::new();

    let auction = init(&sys);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, (DURATION / 1000 - 1).into());

    assert!(result.contains(&(
        USERS[1],
        Err::<Event, Error>(Error::AlreadyRunning).encode()
    )));
}

#[test]
fn create_and_stop() {
    let sys = System::new();
    let owner_user = USERS[0];
    let auction = init(&sys);

    let result = auction.send(owner_user, Action::ForceStop);

    assert!(result.contains(&(
        owner_user,
        Ok::<Event, Error>(Event::AuctionStopped {
            token_owner: owner_user.into(),
            token_id: 0.into(),
        })
        .encode()
    )));
}

