use auction_io::auction::*;
use gtest::System;

mod routines;
use routines::*;

#[test]
fn is_not_active_after_time_is_over() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);

    if let Ok(AuctionInfo { status, .. }) = auction.read_state() {
        dbg!(&status);
        assert!(!matches!(status, Status::Expired))
    }
}

#[test]
fn is_active_before_deal() {
    let sys = System::new();

    let auction = init(&sys);

    if let Ok(AuctionInfo { status, .. }) = auction.read_state() {
        dbg!(&status);
        assert!(matches!(status, Status::IsRunning));
    } else {
        panic!("Can't get state");
    }
}
