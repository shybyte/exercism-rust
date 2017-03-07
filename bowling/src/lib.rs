const MAX_PINS: u32 = 10;
const NORMAL_FRAMES_COUNT: usize = 10;

#[derive(Clone, Debug)]
enum Frame {
    Open(u32, u32),
    Spare(u32), // u32 = pins in first throw
    Strike,
}

impl Frame {
    pub fn first_throw_points(&self) -> u32 {
        match *self {
            Frame::Strike => 10,
            Frame::Open(t1, _) |
            Frame::Spare(t1) => t1,
        }
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    incomplete_frame: Option<u32>,
    fill_balls_count: u32,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::new(),
            incomplete_frame: None,
            fill_balls_count: 0,
        }
    }

    fn needs_more_fill_balls(&self) -> bool {
        if self.frames.len() >= NORMAL_FRAMES_COUNT {
            match self.frames[NORMAL_FRAMES_COUNT - 1] {
                Frame::Open(_, _) => false,
                Frame::Spare(_) => self.fill_balls_count < 1,
                Frame::Strike => self.fill_balls_count < 2,
            }
        } else {
            false
        }
    }

    pub fn roll(&mut self, pins: u32) -> Result<(), String> {
        if pins + self.incomplete_frame.unwrap_or(0) > MAX_PINS {
            return Err(format!("Invalid number of pins: {}", pins));
        }

        if self.frames.len() >= NORMAL_FRAMES_COUNT {
            if self.needs_more_fill_balls() {
                self.fill_balls_count += 1;
            } else {
                return Err("Finished, no rolls anymore!".to_string());
            }
        }

        match self.incomplete_frame {
            Some(prev_pins) => {
                self.frames.push(match prev_pins + pins {
                    MAX_PINS => Frame::Spare(prev_pins),
                    _ => Frame::Open(prev_pins, pins),
                });
                self.incomplete_frame = None
            }
            None => {
                if pins == MAX_PINS {
                    self.frames.push(Frame::Strike)
                } else {
                    self.incomplete_frame = Some(pins)
                }
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u32, String> {
        if self.frames.len() < NORMAL_FRAMES_COUNT {
            return Err(format!("Invalid number of frames: {}", self.frames.len()));
        } else if self.needs_more_fill_balls() {
            return Err(format!("Need more fill balls, have just {}", self.fill_balls_count));
        }

        let frames = &mut self.frames.clone();
        frames.push(Frame::Open(self.incomplete_frame.unwrap_or(0), 0));
        frames.push(Frame::Open(0, 0)); // make sure the later .windows(3) gets happy

        Ok(frames.windows(3)
            .take(NORMAL_FRAMES_COUNT)
            .map(|frame_window| match frame_window[0] {
                Frame::Strike => {
                    10 +
                    (match frame_window[1] {
                        Frame::Open(pins1, pins2) => pins1 + pins2,
                        Frame::Spare(_) | Frame::Strike => {
                            10 + frame_window[2].first_throw_points()
                        }
                    })
                }
                Frame::Spare(_) => 10 + frame_window[1].first_throw_points(),
                Frame::Open(pins1, pins2) => pins1 + pins2,
            })
            .sum())
    }
}
