pub const SECTION_NAME: &str = "[Difficulty]";

/// 谱面难度
#[derive(Debug)]
pub struct Difficulty {
    pub hp_drain_rate: f64,
    pub circle_size: f64,
    pub overall_difficulty: f64,
    pub approach_rate: f64,
    pub slider_multiplier: f64,
    pub slider_tick_rate: f64,
}

/// 默认值
impl Default for Difficulty {
    fn default() -> Self {
        Self {
            hp_drain_rate: 0.0,
            circle_size: 0.0,
            overall_difficulty: 0.0,
            approach_rate: 0.0,
            slider_multiplier: 0.0,
            slider_tick_rate: 0.0,
        }
    }
}
