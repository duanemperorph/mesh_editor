//
// Panes.rs - editor pane calculation and border drawing
//

use macroquad::prelude::*;

const STATUS_BAR_HEIGHT: f32 = 20.0;

pub struct Panes {
    left: Rect,
    top_right: Rect,
    bottom_right: Rect,
    bottom_bar: Rect,
    borders: Vec<BorderLine>,
}

struct BorderLine {
    start: Vec2,
    end: Vec2,
}

//
// Calculation / construction
//

impl Panes {
    pub fn calc_from_screen_dims() -> Panes {
        let height_without_status_bar = screen_height() - STATUS_BAR_HEIGHT;
        let mid_height = height_without_status_bar / 2.0;
        let mid_width = screen_width() / 2.0;

        let left = Rect {
            x: 0.0,
            y: 0.0,
            w: mid_width,
            h: height_without_status_bar,
        };

        let top_right = Rect {
            x: mid_width,
            y: 0.0,
            w: mid_width,
            h: mid_height,
        };

        let bottom_right = Rect {
            x: mid_width,
            y: mid_height,
            w: mid_width,
            h: mid_height,
        };

        let bottom_bar = Rect {
            x: 0.0,
            y: height_without_status_bar,
            w: screen_width(),
            h: STATUS_BAR_HEIGHT,
        };

        let center_border_x = BorderLine {
            start: vec2(mid_width, 0.0),
            end: vec2(mid_width, height_without_status_bar),
        };

        let right_side_border_y = BorderLine {
            start: vec2(mid_width, mid_height),
            end: vec2(screen_width(), mid_height),
        };

        let bottom_border = BorderLine {
            start: vec2(0.0, height_without_status_bar),
            end: vec2(screen_width(), height_without_status_bar),
        };

        let borders = vec![center_border_x, right_side_border_y, bottom_border];

        Panes {
            left,
            top_right,
            bottom_right,
            bottom_bar,
            borders,
        }
    }
}

//
// Draw Borders
//

impl Panes {
    pub fn draw_borders(&self) {
        for border in self.borders.iter() {
            Self::draw_border_line(border);
        }
    }

    fn draw_border_line(border: &BorderLine) {
        draw_line(
            border.start.x,
            border.start.y,
            border.end.x,
            border.end.y,
            1.0,
            GRAY,
        );
    }
}

//
// Accessors
//

impl Panes {
    pub fn left_rect(&self) -> Rect {
        self.left
    }

    pub fn left_viewport(&self) -> (i32, i32, i32, i32) {
        rect_to_viewport(self.left)
    }

    pub fn top_right_rect(&self) -> Rect {
        self.top_right
    }

    pub fn top_right_viewport(&self) -> (i32, i32, i32, i32) {
        rect_to_viewport(self.top_right)
    }

    pub fn bottom_right_rect(&self) -> Rect {
        self.bottom_right
    }

    pub fn bottom_right_viewport(&self) -> (i32, i32, i32, i32) {
        rect_to_viewport(self.bottom_right)
    }

    pub fn bottom_bar_rect(&self) -> Rect {
        self.bottom_bar
    }

    pub fn bottom_bar_viewport(&self) -> (i32, i32, i32, i32) {
        rect_to_viewport(self.bottom_bar)
    }
}

fn rect_to_viewport(rect: Rect) -> (i32, i32, i32, i32) {
    (rect.x as i32, rect.y as i32, rect.w as i32, rect.h as i32)
}
