Let me check the available shape drawing methods in egui's Painter.Based on the documentation [[1]](https://doc.qu1x.dev/bevy_trackball/egui/struct.Painter.html), the Painter struct provides several methods for drawing shapes:

1. Circle methods:
    - `circle(center, radius, fill_color, stroke)` - Circle with fill and stroke
    - `circle_filled(center, radius, fill_color)` - Circle with just fill

2. Line methods:
    - `line_segment([point1, point2], stroke)` - Single line between two points
    - `line(points, stroke)` - Series of connected line segments

3. Rectangle methods:
    - `rect(rect, rounding, fill_color, stroke)` - Rectangle with optional rounded corners
    - `rect_filled(rect, rounding, fill_color)` - Filled rectangle with optional rounded corners
    - `rect_stroke(rect, rounding, stroke)` - Rectangle outline

4. Text:
    - `text(pos, anchor, text, font_id, text_color)` - Draw text at a position

Here's a quick example of how to use some of these:

```rust
impl Draw for MyShape {
    fn draw(&self, ui: &mut Ui) {
        let painter = ui.painter();
        
        // Draw a line
        painter.line_segment(
            [Pos2::new(0.0, 0.0), Pos2::new(100.0, 100.0)],
            Stroke::new(2.0, Color32::RED)
        );
        
        // Draw a filled rectangle
        painter.rect_filled(
            Rect::from_center_size(Pos2::new(50.0, 50.0), Vec2::new(40.0, 30.0)),
            0.0,  // rounding
            Color32::BLUE
        );
        
        // Draw a circle with border
        painter.circle(
            Pos2::new(75.0, 75.0),  // center
            20.0,                    // radius
            Color32::GREEN,          // fill
            Stroke::new(1.0, Color32::WHITE)  // border
        );
    }
}
```


All these methods are direct ways to draw shapes on the screen and are generally simpler to use than creating Shape structs manually.

[1]: https://doc.qu1x.dev/bevy_trackball/egui/struct.Painter.html

***************************** 

Let me check if there's a polyline drawing capability in egui.Yes! For drawing a polyline (a line through multiple points), you have a couple of options in egui:

1. Using the `painter.line()` method:
```rust
let points = vec![
    Pos2::new(0.0, 0.0),
    Pos2::new(50.0, 50.0),
    Pos2::new(100.0, 0.0),
    // ... more points
];
painter.line(points, Stroke::new(2.0, Color32::RED));
```


2. Using `Shape::Path` for more complex paths [[1]](https://traffloat.github.io/api/master/egui/enum.Shape.html):
```rust
let points = vec![
    Pos2::new(0.0, 0.0),
    Pos2::new(50.0, 50.0),
    Pos2::new(100.0, 0.0),
    // ... more points
];
let path = Shape::line(points, Stroke::new(2.0, Color32::RED));
painter.add(path);
```


If you want to create a closed shape (where the last point connects back to the first point), you can use `Shape::closed_line()` instead.

The main difference between these methods is that `painter.line()` is a convenience method that handles the shape creation for you, while using `Shape` directly gives you more control over the path properties.

Both methods support:
- Multiple points
- Stroke width and color
- Open or closed paths (depending on the method used)

The points are connected in the order they appear in the vector.

[1]: https://traffloat.github.io/api/master/egui/enum.Shape.html


