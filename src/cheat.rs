use std::{ffi::c_void, time::Duration};
use crate::memory::Memory;

enum MeetingState {
    NoMeeting,
    MeetingInProgress,
}
pub trait AmongUsCheat {
    unsafe fn cheat_loop(self, base: *const c_void) -> !;
}

impl AmongUsCheat for Memory {
    unsafe fn cheat_loop(self, base: *const c_void) -> ! {
        unsafe {
            let meeting_hub: *const c_void = self.read_ptr(base.add(0x29ACA80));
            let meeting_hub: *const c_void = self.read_ptr(meeting_hub.add(0x5C));

            let mut prev_meeting_hub: *const c_void = self.read_ptr(meeting_hub.add(0x00));

            let mut state = MeetingState::NoMeeting;

            loop {

                match &state {
                    MeetingState::NoMeeting => {
                        let curr_meeting_hub: *const c_void = self.read_ptr(meeting_hub.add(0x00));
                        // println!("{curr_meeting_hub:?}");
                        // println!("{voting_state:?}");

                        // When the meeting hub changes -> there is a new meeting
                        if curr_meeting_hub != prev_meeting_hub {
                            state = MeetingState::MeetingInProgress;
                            prev_meeting_hub = curr_meeting_hub;

                            println!("MEETING IS IN PROGRESS: Unmuting people")
                            // UNMUTE_PEOPLE();
                        }
                    },
                    MeetingState::MeetingInProgress => {
                        let curr_meeting_hub: *const c_void = self.read_ptr(meeting_hub.add(0x00));

                        let voting_state = self.read::<u32>(curr_meeting_hub.add(0x88));

                        // When `voting_state = 4` -> Meeting ended
                        if voting_state == 4 {
                            state = MeetingState::NoMeeting;

                            std::thread::sleep(Duration::from_millis(4000));

                            println!("MEETING ENDED: Muting people")
                            // MUTE_PEOPLE;
                        }
                    }
                }

                std::thread::sleep(Duration::from_millis(100))
            }
        }
    }
}