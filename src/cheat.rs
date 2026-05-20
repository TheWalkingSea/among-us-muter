mod memory;

#[allow(unused_imports)]
pub use memory::Memory;

use std::{ffi::c_void, time::Duration};

enum GameState {
    NoMeeting,
    MeetingInProgress,
    NotJoined, 
    Joined, 
    Started, 
    Ended
}

pub trait AmongUsCheat {
    unsafe fn block_until_join(&self, base: *const c_void);
    unsafe fn cheat_loop(&self, base: *const c_void, on_emergency: impl FnMut(bool)) -> !;
    unsafe fn get_gamestate(&self, amongusclient__instance: *const c_void) -> Option<GameState>
}

#[allow(non_snake_case)]
impl AmongUsCheat for memory::Memory {

    unsafe fn get_gamestate(&self, amongusclient__instance: *const c_void) -> Option<GameState> {
        unsafe {
            return match self.read::<u32>(amongusclient__instance.add(0x64)) {
                0 => Some(GameState::NotJoined),
                1 => Some(GameState::Joined),
                2 => Some(GameState::Started),
                3 => Some(GameState::Ended),
                _ => None
            }
        }
    }

    unsafe fn block_until_join(&self, base: *const c_void) {
        unsafe {
            loop {
                let meeting_hub: *const c_void = self.read_ptr(base.add(0x29ACA80));
                println!("Waiting for user to join a lobby: {meeting_hub:?}");
                if meeting_hub != 0x2000f6dd as *const c_void {
                    println!("User joined lobby!");
                    break;
                }

                std::thread::sleep(Duration::from_millis(1000))
            }
        }
    }
    
    unsafe fn cheat_loop(&self, base: *const c_void, mut on_emergency: impl FnMut(bool)) -> ! {
        unsafe {

            self.block_until_join(base);

            let game_data: *const c_void = self.read_ptr(base.add(0x29CD380));
            let game_data__static_fields: *const c_void = self.read_ptr(game_data.add(0x5C));
            let game_data__instance: *const c_void = self.read_ptr(game_data__static_fields.add(0x00));

            let amongusclient__instance: *const c_void = self.read_ptr(base.add(0x29AB228));

            let meeting_hub: *const c_void = self.read_ptr(base.add(0x29ACA80));
            let meeting_hub__static_fields: *const c_void = self.read_ptr(meeting_hub.add(0x5C));

            let mut prev_meeting_hub: *const c_void = self.read_ptr(meeting_hub__static_fields.add(0x00));

            let mut state = GameState::NoMeeting;

            loop {
                let game_data__all_players: *const c_void = self.read_ptr(game_data__instance.add(0x10));
                let all_players__size: i32 = self.read::<i32>(game_data__all_players.add(0x0C));
                // println!("all_players__size: {}", all_players__size);

                let all_players__items_array: *const c_void = self.read_ptr(game_data__all_players.add(0x08));

                for i in 0..all_players__size as usize {
                    let player: *const c_void = self.read_ptr(all_players__items_array.add(0x10 + i * 4));
                    let player__is_dead: bool = self.read::<bool>(player.add(0x54));

                    let player__friend_code: *const c_void = self.read_ptr(player.add(0x30));
                    let friend_code = self.read_il2cpp_string(player__friend_code);

                    // dbg!(player__is_dead);
                    // dbg!(friend_code);
                }

                dbg!(self.in_lobby(base));


                match &state {
                    GameState::NoMeeting => {
                        let curr_meeting_hub: *const c_void = self.read_ptr(meeting_hub__static_fields.add(0x00));
                        // println!("{curr_meeting_hub:?}");
                        // println!("{prev_meeting_hub:?}");
                        // let voting_state = self.read::<u32>(curr_meeting_hub.add(0x88));
                        // println!("{voting_state:?}");

                        // When the meeting hub changes -> there is a new meeting
                        if curr_meeting_hub != prev_meeting_hub {
                            state = GameState::MeetingInProgress;
                            prev_meeting_hub = curr_meeting_hub;

                            on_emergency(true);
                            println!("MEETING IS IN PROGRESS: Unmuting people")
                            // UNMUTE_PEOPLE();
                        }
                    },
                    GameState::MeetingInProgress => {
                        let curr_meeting_hub: *const c_void = self.read_ptr(meeting_hub__static_fields.add(0x00));

                        let voting_state = self.read::<u32>(curr_meeting_hub.add(0x88));

                        // When `voting_state = 4` -> Meeting ended
                        if voting_state == 4 {
                            state = GameState::NoMeeting;

                            std::thread::sleep(Duration::from_millis(12000));
                            on_emergency(false);
                            println!("MEETING ENDED: Muting people")
                            // MUTE_PEOPLE;
                        }
                    }
                }

                std::thread::sleep(Duration::from_millis(500))
            }
        }
    }
}