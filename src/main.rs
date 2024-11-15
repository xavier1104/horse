use std::{fmt::format, future::Ready, thread, time::Duration};


#[derive(PartialEq, Eq)]
enum GameState{
    Ready,
    Start,
    End,
}


enum HorseState{
    Health,
    High,
    Lazy,
    Sick,
}

const TRACK_LEN:i32 = 20;

struct Horse{
    state:HorseState,
}

struct Track{
    id:i32,
    current_distance:i32,
    bet:i32,
    horse:Horse,
    odds:f32,
}

fn update_view(tracks:&Vec<Track>){
    update_track(tracks);
    update_status();
    update_bets(tracks);
}

fn update_track(tracks:&Vec<Track>){
    for track in tracks{
        let mut track_view = track.id.to_string();
        for _ in 1..=track.current_distance {
            track_view += "=";
        }

        track_view += ">";

        let rest_distance = TRACK_LEN - track.current_distance;
        for _ in 1..=rest_distance{
            track_view += " ";
        }

        track_view += "|";
        println!("{}", track_view);
    }
}

fn update_bets(tracks:&Vec<Track>){
    for track in tracks{
        let mut bet_view = track.id.to_string();
        bet_view += format!(".TotalBet:{}", track.bet).as_str();
        println!("{}", bet_view);
    }
}

fn update_status(){

}

fn user_add_bet(){

}

fn main() {
    let game_state = GameState::Ready;
    let mut tracks = Vec::<Track>::new();
    for i in 1..=4{
        tracks.push(Track{id:i, current_distance:0, bet:0, odds:1.0, horse:Horse{state:HorseState::Health}});
    }
    
    

    loop {
        if game_state == GameState::Ready{
            update_view(&tracks);
            user_add_bet();
        }

        for t in &mut tracks{
            t.current_distance += 1;
        }
        update_view(&tracks);
        thread::sleep(Duration::from_secs(1));
        clearscreen::clear().expect("failed to clear screen");
    }
}
