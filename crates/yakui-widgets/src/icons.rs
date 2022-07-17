use yakui_core::geometry::{Color3, Rect, Vec2};
use yakui_core::paint::{PaintDom, PaintMesh, Vertex};

pub fn cross(output: &mut PaintDom, rect: Rect, color: Color3) {
    static POSITIONS: [[f32; 2]; 12] = [
        // Top
        [0.25, 0.0], // 0
        [0.75, 0.0], // 1
        // Left
        [0.0, 0.25], // 2
        [0.0, 0.75], // 3
        // Bottom
        [0.25, 1.0], // 4
        [0.75, 1.0], // 5
        // Right
        [1.0, 0.25], // 6
        [1.0, 0.75], // 7
        // Inner points
        [0.5, 0.25], // 8
        [0.25, 0.5], // 9
        [0.5, 0.75], // 10
        [0.75, 0.5], // 11
    ];

    #[rustfmt::skip]
    static INDICES: [u16; 18] = [
        // '\' part of the X
        0, 2, 5,
        5, 7, 0,
        // bottom left square
        9, 3, 4,
        4, 10, 9,
        // top right square
        1, 8, 11,
        11, 6, 1,
    ];

    let color = color.to_linear().extend(1.0);

    let vertices = POSITIONS
        .into_iter()
        .map(Vec2::from)
        .map(|pos| Vertex::new(rect.pos() + pos * rect.size(), [0.0, 0.0], color));

    let mesh = PaintMesh::new(vertices, INDICES);
    output.add_mesh(mesh);
}

pub fn outline(output: &mut PaintDom, rect: Rect, thickness: Vec2, color: Color3) {
    let vw = rect.size().x;
    let vh = rect.size().y;
    let w = thickness.x;
    let h = thickness.y;

    #[rustfmt::skip]
    let positions: [[f32; 2]; 12] = [
        // Main rectangle
        [0.0, 0.0], // 0
        [0.0, vh], // 1
        [ vw, vh], // 2
        [ vw, 0.0], // 3
        // Left border
        [0.0,      h], // 4
        [0.0, vh - h], // 5
        [  w, vh - h], // 6
        [  w,      h], // 7
        // Right border
        [vw - w,      h], // 8
        [vw - w, vh - h], // 9
        [    vw, vh - h], // 10
        [    vw,      h], // 11
    ];

    #[rustfmt::skip]
    static INDICES: [u16; 24] = [
        // Top
        0, 4, 11,
        11, 3, 0,
        // Bottom
        5, 1, 2,
        2, 10, 5,
        // Left
        4, 5, 6,
        6, 7, 4,
        // Right
        8, 9, 10,
        10, 11, 8,
    ];

    let color = color.to_linear().extend(1.0);

    let vertices = positions
        .into_iter()
        .map(Vec2::from)
        .map(|pos| Vertex::new(rect.pos() + pos, [0.0, 0.0], color));

    let mesh = PaintMesh::new(vertices, INDICES);
    output.add_mesh(mesh);
}
