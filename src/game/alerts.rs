use gemini_engine::elements::{
    ascii::TextAlign,
    view::{Modifier, ViewElement},
    Pixel, Text, Vec2D,
};

const ALERT_LIFETIME: u16 = 20;

pub fn generate_alert_for_filled_lines(cleared_lines: isize) -> Option<(isize, String)> {
    match cleared_lines {
        1 => Some((100, String::from("Single!"))),
        2 => Some((300, String::from("Double!"))),
        3 => Some((500, String::from("Triple!"))),
        4 => Some((800, String::from("Tetris!"))),
        0 => None,
        _ => panic!("entered value should be between 0 and 4"),
    }
}

pub struct AlertDisplay {
    pub pos: Vec2D,
    alerts: Vec<(String, u16)>,
}

impl AlertDisplay {
    pub fn new(pos: Vec2D) -> AlertDisplay {
        AlertDisplay {
            pos,
            alerts: vec![],
        }
    }

    pub fn push(&mut self, alert: &str) {
        self.alerts.push((String::from(alert), ALERT_LIFETIME))
    }

    pub fn handle_with_score(
        &mut self,
        score: &mut isize,
        score_and_alert: Option<(isize, String)>,
    ) {
        if let Some((add_score, alert)) = score_and_alert {
            *score += add_score;
            self.push(&alert);
        }
    }

    pub fn frame(&mut self) {
        if !self.alerts.is_empty() {
            let mut i = 0;
            loop {
                if self.alerts.get(i).is_none() {
                    break;
                }
                self.alerts[i].1 -= 1;
                if self.alerts[i].1 == 0 {
                    self.alerts.remove(i);
                    i = i.saturating_sub(1);
                }

                i += 1;
            }
        }
    }
}

impl ViewElement for AlertDisplay {
    fn active_pixels(&self) -> Vec<Pixel> {
        self.alerts
            .iter()
            .enumerate()
            .flat_map(|(i, (alert, _))| {
                Text::draw_with_align(
                    self.pos + Vec2D::new(0, i as isize),
                    alert,
                    TextAlign::Centered,
                    Modifier::None,
                )
            })
            .collect()
    }
}

impl Default for AlertDisplay {
    fn default() -> Self {
        Self::new(Vec2D::ZERO)
    }
}
