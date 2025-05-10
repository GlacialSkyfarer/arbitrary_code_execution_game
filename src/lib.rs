use macroquad::math::IVec2;

pub const DIRECTIONS: [IVec2; 4] = [
    IVec2 { x: 0, y: -1 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 1, y: 0 },
];
