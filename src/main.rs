use std::{fmt, fmt::format, future::Ready, io::stdin, path::Display, thread, time::Duration};

use rand::Rng;

#[derive(PartialEq, Eq)]
enum GameState{
    Init,
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
const TRACK_NUM:i32 = 4;

struct Horse{
    state:HorseState,
}

impl Horse {
    fn random_state(&mut self){
        let mut rng = rand::thread_rng();
        self.state = match rng.gen_range(0..4){
            0 => HorseState::Health,
            1 => HorseState::High,
            2 => HorseState::Lazy,
            _ => HorseState::Sick,
        };
    }
}

struct Track{
    id:i32,
    current_distance:i32,
    bet:i32,
    horse:Horse,
    odds:i32,
    rank:i32,
}

impl Track{
    fn init(&mut self){
        self.current_distance = 0;
        self.bet = 0;
        self.rank = 0;
        self.horse.random_state();
        self.odds = match self.horse.state{
            HorseState::Health => 3,
            HorseState::High => 2,
            HorseState::Lazy => 4,
            HorseState::Sick => 8,
        }
    }

    fn run(&mut self){
        let add_distance = match(self.horse.state){
            HorseState::Health => {
                let mut rng = rand::thread_rng();
                let num = rng.gen_range(0..100);
                if num >= 0 && num < 5 {
                    0
                }
                else if num >= 5 && num < 75{
                    1
                }
                else if num >= 75 && num < 95{
                    2
                }
                else{
                    3
                }
            },
            HorseState::High => {
                let mut rng = rand::thread_rng();
                let num = rng.gen_range(0..100);
                if num >= 0 && num < 2 {
                    0
                }
                else if num >= 2 && num < 72{
                    1
                }
                else if num >= 72 && num < 92{
                    2
                }
                else{
                    3
                }
            },
            HorseState::Lazy => {
                let mut rng = rand::thread_rng();
                let num = rng.gen_range(0..100);
                if num >= 0 && num < 10 {
                    0
                }
                else if num >= 10 && num < 80{
                    1
                }
                else if num >= 80 && num < 95{
                    2
                }
                else{
                    3
                }
            },
            HorseState::Sick => {
                let mut rng = rand::thread_rng();
                let num = rng.gen_range(0..100);
                if num >= 0 && num < 7 {
                    0
                }
                else if num >= 7 && num < 80{
                    1
                }
                else if num >= 80 && num < 97{
                    2
                }
                else{
                    3
                }
            },
        };

        self.current_distance += add_distance;
        if self.current_distance >= TRACK_LEN{
            self.current_distance = TRACK_LEN;
        }
    }
}

struct Player{
    money:i32,
}

fn update_view(tracks:&Vec<Track>){
    update_track(tracks);
    update_status(tracks);
}

fn update_track(tracks:&Vec<Track>){
    println!("######賽道######");
    for track in tracks{
        let mut track_view = format!("賽道{}",track.id.to_string());
        for _ in 1..=track.current_distance {
            track_view += "=";
        }

        track_view += ">";

        let rest_distance = TRACK_LEN - track.current_distance;
        for _ in 1..=rest_distance{
            track_view += " ";
        }

        track_view += "|";
        
        if track.rank > 0{
            track_view += format!(" 第{}名", track.rank).as_str();
        }

        println!("{}", track_view);
    }
    println!("################");
}

fn update_status(tracks:&Vec<Track>){
    println!("######押注區####");
    for track in tracks{
        let mut status_view = format!("賽道{}",track.id.to_string());
        let state_str = match track.horse.state{
            HorseState::Health => "健康",
            HorseState::High => "躍躍欲試",
            HorseState::Lazy => "有點懶散",
            HorseState::Sick => "生病",
        };

        status_view += format!(".賠率:{}    狀態:{}     總押注:{}", track.odds, state_str, track.bet).as_str();
        println!("{}", status_view);
    }
    println!("################");
}

fn user_action(player:&mut Player, tracks:&mut Vec<Track>)->bool{
    
    println!("請輸入 1:下注 或 任意鍵後enter開始");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("輸入失敗");
    let input = input.trim();

    if input == "1"{
        println!("你的現金:{}", player.money);
        println!("請輸入下注跑道 ex: 1 100 => <跑道> <下注金額>");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("下注失敗");
        let input = input.trim();
        let bet_action:Vec<&str> = input.split_whitespace().collect();
        if bet_action.len() == 2{
            let track_id = bet_action[0].parse::<usize>();
            let bet = bet_action[1].parse::<i32>();
            match (track_id, bet){
                (Ok(track_id), Ok(bet)) => 
                if track_id > 0 && track_id <= tracks.len() && bet <= player.money{
                    tracks[track_id - 1].bet += bet;
                    player.money -= bet;
                    println!("下注成功");
                }
                _=>{
                    println!("下注格式錯誤");
                }
            }
        }
        else{
            println!("輸入格式錯誤");
        }

        return false;
    }

    return true;
}

fn give_rank(rank:i32, tracks:&mut Vec<Track>)->bool{
    let mut is_give = false;
    for t in tracks{
        if t.current_distance == TRACK_LEN && t.rank == 0{
            t.rank = rank;
            is_give = true;
        }
    }

    return is_give;
}

fn is_finish(tracks:&Vec<Track>)->bool{
    for t in tracks{
        if t.rank == 0{
            return false;
        }
    }

    return true;
}

fn cal_result(player:&mut Player, tracks:&Vec<Track>){
    let mut win = 0;

    for t in tracks{
        win -= t.bet;
        if t.rank == 1{
            let money = t.bet * t.odds;
            player.money += money;
            win += money;
        }
    }

    if win > 0{
        println!("恭喜你贏得{}", win);
    }
    else if win < 0{
        println!("賠錢囉!{}", win);
    }
}

fn main() {
    let mut game_state = GameState::Init;
    let mut tracks = Vec::<Track>::new();
    for i in 1..=TRACK_NUM{
        tracks.push(Track{id:i, current_distance:0, bet:0, odds:1, horse:Horse{state:HorseState::Health}, rank:0});
    }
    let mut player = Player{money:1000};
    let mut current_rank = 1;

    loop {
        if game_state == GameState::Init{
            current_rank = 1;
            for t in &mut tracks{
                t.init();
            }
            game_state = GameState::Ready;
        }
        else if game_state == GameState::Ready{
            update_view(&tracks);
            let finish = user_action(&mut player, &mut tracks);
            if finish {
                game_state = GameState::Start;
            }
        }
        else if game_state == GameState::Start{
            for t in &mut tracks{
                t.run();
            }

            if give_rank(current_rank, &mut tracks){
                current_rank += 1;
            }

            if is_finish(&tracks){
                game_state = GameState::End;
            }
            
            update_view(&tracks);
            thread::sleep(Duration::from_secs(1));
        }
        else if game_state == GameState::End{
            cal_result(&mut player, &tracks);
            update_view(&tracks);
            game_state = GameState::Init;
        }
        clearscreen::clear().expect("failed to clear screen");
    }
}
