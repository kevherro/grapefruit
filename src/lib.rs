#[allow(dead_code)]
struct Button {
    position: (f32, f32),
    size: (f32, f32),
    label: String,
    is_pressed: bool,
}

impl Button {
    #[allow(dead_code)]
    fn new(position: (f32, f32), size: (f32, f32), label: &str) -> Self {
        Button {
            position,
            size,
            label: label.to_owned(),
            is_pressed: false,
        }
    }

    #[allow(dead_code)]
    fn handle_input(&mut self, mouse_pos: (f32, f32), is_mouse_down: bool) {
        if is_mouse_down && self.is_mouse_over(mouse_pos) {
            self.is_pressed = true;
        } else {
            self.is_pressed = false;
        }
    }

    fn is_mouse_over(&self, mouse_pos: (f32, f32)) -> bool {
        let (x, y) = mouse_pos;
        let (button_x, button_y) = self.position;
        let (button_width, button_height) = self.size;
        x >= button_x && x <= button_width && y >= button_y && y <= button_y + button_height
    }

    #[allow(dead_code)]
    fn render(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mouse_over() {
        let button = Button::new((100.0, 100.0), (200.0, 50.0), "Button");

        // Test when the mouse position is inside the button
        let mouse_pos_inside = (150.0, 125.0);
        assert!(button.is_mouse_over(mouse_pos_inside));

        // Test when the mouse position is outside the button
        let mouse_pos_outside = (50.0, 75.0);
        assert!(!button.is_mouse_over(mouse_pos_outside));

        // Test when the mouse position is exactly on the button's boundary
        let mouse_pos_boundary = (100.0, 100.0);
        assert!(button.is_mouse_over(mouse_pos_boundary));
    }
}
