// Terrain rendering utilities
use sdl2::pixels::Color;
use crate::map::Terrain;

pub fn terrain_color(terrain: &Terrain) -> Color {
    match terrain {
        Terrain::Grass => Color::RGB(67, 160, 71),
        Terrain::Water => Color::RGB(25, 118, 210),
        Terrain::Forest => Color::RGB(46, 83, 57),
        Terrain::Mountain => Color::RGB(141, 103, 72),
    }
}
