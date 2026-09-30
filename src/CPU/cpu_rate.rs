use chrono::TimeDelta;



/**
 * A bounded position in `0..=option_max`. `rate` uses one to pick which of its speed
 * presets is selected, the slider itself knows nothing about speeds.
 */
struct slider{
    option_max: u64,
    current_option: u64,
}

impl slider{



    /**
     * Create a new slider, starting at 0.
     *
     * `option_max` - highest index the slider goes up to (inclusive).
     *
     * Will not pass max or min, moving past either end returns false
     * (see `increment` / `decrement`)
     */
    pub fn new(option_max: u64) -> Self{
        return Self{option_max, current_option: 0};
    }

    /**
     * Increments current position in the slider (right)
     *
     * `returns` - false if increment has failed. Does not
     * throw an error so it is up to caller to catch this.
     */
    pub fn increment(&mut self) -> bool{
        if self.current_option != self.option_max{
            self.current_option += 1;
            return true;
        }
        return false;
    }

    /**
     * Decrements current position in the slider (left)
     *
     * `returns` - false if decrement has failed, same as `increment`
     */
    pub fn decrement(&mut self) -> bool{
        if self.current_option != 0{
            self.current_option -= 1;
            return true;
        }
        return false;
    }

    /**
     * Jumps straight to `option`.
     *
     * `returns` - false (and does not move) if `option` is past max
     */
    pub fn set(&mut self, option: u64) -> bool{
        if option > self.option_max{
            return false;
        }
        self.current_option = option;
        return true;
    }

    pub fn current(&self) -> u64{
        return self.current_option;
    }

}

/**
 * Controls how fast the CPU executes. The render loop calls `tick` once per frame, then
 * runs instructions while `isAllowed` is true, calling `step_taken` after each one.
 */
pub struct rate{
    _slider: slider,      // which preset from rates() is selected
    _rate: u8,            // instructions per second for that preset, MAX means unlimited
    _time: TimeDelta,     // running clock, advanced by tick() while not paused
    _lastTime: TimeDelta, // clock value the last instruction was released at
    _paused: bool,
}


impl rate{
    const MAX: u8 = u8::MAX;
    const OPTION_COUNT: u64 = 6;      // how many presets rates() has
    const DEFAULT_OPTION: u64 = 3;    // 10 instructions per second
    const MAX_FRAME_SECONDS: f32 = 0.25; // longest frame tick() will count, see tick()

    /// Most instructions an unlimited rate will run in one frame, so the window keeps drawing.
    pub const MAX_STEPS_PER_FRAME: u32 = 1000;

    pub fn new() -> Self{
        let mut _slider = slider::new(Self::OPTION_COUNT - 1);
        _slider.set(Self::DEFAULT_OPTION);
        let _rate = Self::rates(_slider.current() as u8).unwrap();
        return Self{
            _slider,
            _rate,
            _time: TimeDelta::zero(),
            _lastTime: TimeDelta::zero(),
            _paused: false,
        };
    }

    /*Gives an index of what each rate  */
    fn rates(index : u8) ->Option<u8>{
        const opt_one: u8 = 1; //one instruction per second
        const opt_two: u8 = 2;
        const opt_three: u8 = 5;
        const opt_four: u8 = 10;
        const opt_five: u8 = 60;
        const opt_unlimited: u8 = u8::MAX;
        match index{
            0 =>{
                return Some(opt_one);
            }
            1 =>{
                return Some(opt_two);
            }
            2 =>{
                return Some(opt_three);
            }
            3 =>{
                return Some(opt_four);
            }
            4 =>{
                return Some(opt_five);
            }
            5 =>{
                return Some(opt_unlimited);
            }
            _ => {
                return None;
            }
        }

    }
    /**
     * Displays how often the rate occurs per second, e.g. "10 instr/s" or "UNLIMITED"
     */
    pub fn display(&self) -> String{
        if self._rate == Self::MAX{
            return String::from("UNLIMITED");
        }
        return format!("{} instr/s", self._rate);
    }

    /**
     * Moves to the next faster preset.
     *
     * `returns` - false if already at the fastest (unlimited)
     */
    pub fn faster(&mut self) -> bool{
        if !self._slider.increment(){
            return false;
        }
        self.apply_option();
        return true;
    }

    /**
     * Moves to the next slower preset.
     *
     * `returns` - false if already at the slowest
     */
    pub fn slower(&mut self) -> bool{
        if !self._slider.decrement(){
            return false;
        }
        self.apply_option();
        return true;
    }

    // pick up the slider's new preset and restart the schedule so it takes effect from now
    fn apply_option(&mut self){
        self._rate = Self::rates(self._slider.current() as u8).unwrap_or(1);
        self._lastTime = self._time;
    }

    // time between instructions at the current rate, not meaningful when unlimited
    fn interval(&self) -> TimeDelta{
        return TimeDelta::seconds(1) / self._rate as i32;
    }

    /**
     * Advances the clock by how long the last frame took. Call once per frame.
     *
     * The clock stands still while paused, so time spent paused doesn't bank up
     * instructions that would all run at once on resume. A stall (dragging the window,
     * a slow instruction like a full screen fill) shows up as one huge frame, so only
     * `MAX_FRAME_SECONDS` of it is counted to keep the catch up afterwards small.
     */
    pub fn tick(&mut self, frame_seconds: f32){
        if self._paused{
            return;
        }
        let counted = frame_seconds.min(Self::MAX_FRAME_SECONDS) as f64;
        self._time += TimeDelta::microseconds((counted * 1_000_000.0) as i64);
    }

    /**
     * Whether the CPU may execute its next instruction right now.
     *
     * Always false while paused and always true when unlimited, otherwise true once
     * 1/rate seconds have passed on the clock since the last instruction.
     */

    pub fn isAllowed(&self) -> bool{
        if self._paused{
            return false;
        }
        if self._rate == Self::MAX{
            return true;
        }
        return self._time >= self._lastTime + self.interval();
    }

    /**
     * Records that the CPU just executed an instruction, scheduling the next one a full
     * interval later. If frames are slower than the interval, `isAllowed` stays true
     * until the CPU has caught up, so the rate holds even at a low frame rate.
     */
    pub fn step_taken(&mut self){
        if self._rate == Self::MAX{
            self._lastTime = self._time;
            return;
        }
        self._lastTime += self.interval();
    }

    /**
     * pauses or resumes the whole CPU.
     */
    pub fn pause(&mut self){
        self._paused = !self._paused;
    }

    pub fn is_paused(&self) -> bool{
        return self._paused;
    }



}
