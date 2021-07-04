use euclid::Point2D;
use glium::implement_vertex;
use glium::vertex::{Attribute as GLAttribute, AttributeType as GLAttributeType};

use crate::geometry::Line as GLine;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vf2(f32, f32);

impl Vf2 {
    pub fn new(x: f32, y: f32) -> Vf2 {
        Vf2(x, y)
    }
}

unsafe impl GLAttribute for Vf2 {
    /// Get the type of data.
    fn get_type() -> GLAttributeType {
        GLAttributeType::F32F32
    }
}

impl<T> From<Point2D<f32, T>> for Vf2 {
    fn from(p: Point2D<f32, T>) -> Self {
        Vf2::new(p.x, p.y)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(f32, f32, f32, f32);

unsafe impl GLAttribute for Color {
    /// Get the type of data.
    fn get_type() -> GLAttributeType {
        GLAttributeType::F32F32F32F32
    }
}

impl Color {
    pub const MAROON: Color = Color(128. / 255., 0. / 255., 0. / 255., 1.);
    pub const DARK_RED: Color = Color(139. / 255., 0. / 255., 0. / 255., 1.);
    pub const BROWN: Color = Color(165. / 255., 42. / 255., 42. / 255., 1.);
    pub const FIREBRICK: Color = Color(178. / 255., 34. / 255., 34. / 255., 1.);
    pub const CRIMSON: Color = Color(220. / 255., 20. / 255., 60. / 255., 1.);
    pub const RED: Color = Color(1., 0. / 255., 0. / 255., 1.);
    pub const TOMATO: Color = Color(1., 99. / 255., 71. / 255., 1.);
    pub const CORAL: Color = Color(1., 127. / 255., 80. / 255., 1.);
    pub const INDIAN_RED: Color = Color(205. / 255., 92. / 255., 92. / 255., 1.);
    pub const LIGHT_CORAL: Color = Color(240. / 255., 128. / 255., 128. / 255., 1.);
    pub const DARK_SALMON: Color = Color(233. / 255., 150. / 255., 122. / 255., 1.);
    pub const SALMON: Color = Color(250. / 255., 128. / 255., 114. / 255., 1.);
    pub const LIGHT_SALMON: Color = Color(1., 160. / 255., 122. / 255., 1.);
    pub const ORANGE_RED: Color = Color(1., 69. / 255., 0. / 255., 1.);
    pub const DARK_ORANGE: Color = Color(1., 140. / 255., 0. / 255., 1.);
    pub const ORANGE: Color = Color(1., 165. / 255., 0. / 255., 1.);
    pub const GOLD: Color = Color(1., 215. / 255., 0. / 255., 1.);
    pub const DARK_GOLDEN_ROD: Color = Color(184. / 255., 134. / 255., 11. / 255., 1.);
    pub const GOLDEN_ROD: Color = Color(218. / 255., 165. / 255., 32. / 255., 1.);
    pub const PALE_GOLDEN_ROD: Color = Color(238. / 255., 232. / 255., 170. / 255., 1.);
    pub const DARK_KHAKI: Color = Color(189. / 255., 183. / 255., 107. / 255., 1.);
    pub const KHAKI: Color = Color(240. / 255., 230. / 255., 140. / 255., 1.);
    pub const OLIVE: Color = Color(128. / 255., 128. / 255., 0. / 255., 1.);
    pub const YELLOW: Color = Color(1., 1., 0. / 255., 1.);
    pub const YELLOW_GREEN: Color = Color(154. / 255., 205. / 255., 50. / 255., 1.);
    pub const DARK_OLIVE_GREEN: Color = Color(85. / 255., 107. / 255., 47. / 255., 1.);
    pub const OLIVE_DRAB: Color = Color(107. / 255., 142. / 255., 35. / 255., 1.);
    pub const LAWN_GREEN: Color = Color(124. / 255., 252. / 255., 0. / 255., 1.);
    pub const CHART_REUSE: Color = Color(127. / 255., 1., 0. / 255., 1.);
    pub const GREEN_YELLOW: Color = Color(173. / 255., 1., 47. / 255., 1.);
    pub const DARK_GREEN: Color = Color(0. / 255., 100. / 255., 0. / 255., 1.);
    pub const GREEN: Color = Color(0. / 255., 128. / 255., 0. / 255., 1.);
    pub const FOREST_GREEN: Color = Color(34. / 255., 139. / 255., 34. / 255., 1.);
    pub const LIME: Color = Color(0. / 255., 1., 0. / 255., 1.);
    pub const LIME_GREEN: Color = Color(50. / 255., 205. / 255., 50. / 255., 1.);
    pub const LIGHT_GREEN: Color = Color(144. / 255., 238. / 255., 144. / 255., 1.);
    pub const PALE_GREEN: Color = Color(152. / 255., 251. / 255., 152. / 255., 1.);
    pub const DARK_SEA_GREEN: Color = Color(143. / 255., 188. / 255., 143. / 255., 1.);
    pub const MEDIUM_SPRING_GREEN: Color = Color(0. / 255., 250. / 255., 154. / 255., 1.);
    pub const SPRING_GREEN: Color = Color(0. / 255., 1., 127. / 255., 1.);
    pub const SEA_GREEN: Color = Color(46. / 255., 139. / 255., 87. / 255., 1.);
    pub const MEDIUM_AQUA_MARINE: Color = Color(102. / 255., 205. / 255., 170. / 255., 1.);
    pub const MEDIUM_SEA_GREEN: Color = Color(60. / 255., 179. / 255., 113. / 255., 1.);
    pub const LIGHT_SEA_GREEN: Color = Color(32. / 255., 178. / 255., 170. / 255., 1.);
    pub const DARK_SLATE_GRAY: Color = Color(47. / 255., 79. / 255., 79. / 255., 1.);
    pub const TEAL: Color = Color(0. / 255., 128. / 255., 128. / 255., 1.);
    pub const DARK_CYAN: Color = Color(0. / 255., 139. / 255., 139. / 255., 1.);
    pub const AQUA: Color = Color(0. / 255., 1., 1., 1.);
    pub const CYAN: Color = Color(0. / 255., 1., 1., 1.);
    pub const LIGHT_CYAN: Color = Color(224. / 255., 1., 1., 1.);
    pub const DARK_TURQUOISE: Color = Color(0. / 255., 206. / 255., 209. / 255., 1.);
    pub const TURQUOISE: Color = Color(64. / 255., 224. / 255., 208. / 255., 1.);
    pub const MEDIUM_TURQUOISE: Color = Color(72. / 255., 209. / 255., 204. / 255., 1.);
    pub const PALE_TURQUOISE: Color = Color(175. / 255., 238. / 255., 238. / 255., 1.);
    pub const AQUA_MARINE: Color = Color(127. / 255., 1., 212. / 255., 1.);
    pub const POWDER_BLUE: Color = Color(176. / 255., 224. / 255., 230. / 255., 1.);
    pub const CADET_BLUE: Color = Color(95. / 255., 158. / 255., 160. / 255., 1.);
    pub const STEEL_BLUE: Color = Color(70. / 255., 130. / 255., 180. / 255., 1.);
    pub const CORN_FLOWER_BLUE: Color = Color(100. / 255., 149. / 255., 237. / 255., 1.);
    pub const DEEP_SKY_BLUE: Color = Color(0. / 255., 191. / 255., 1., 1.);
    pub const DODGER_BLUE: Color = Color(30. / 255., 144. / 255., 1., 1.);
    pub const LIGHT_BLUE: Color = Color(173. / 255., 216. / 255., 230. / 255., 1.);
    pub const SKY_BLUE: Color = Color(135. / 255., 206. / 255., 235. / 255., 1.);
    pub const LIGHT_SKY_BLUE: Color = Color(135. / 255., 206. / 255., 250. / 255., 1.);
    pub const MIDNIGHT_BLUE: Color = Color(25. / 255., 25. / 255., 112. / 255., 1.);
    pub const NAVY: Color = Color(0. / 255., 0. / 255., 128. / 255., 1.);
    pub const DARK_BLUE: Color = Color(0. / 255., 0. / 255., 139. / 255., 1.);
    pub const MEDIUM_BLUE: Color = Color(0. / 255., 0. / 255., 205. / 255., 1.);
    pub const BLUE: Color = Color(0. / 255., 0. / 255., 1., 1.);
    pub const ROYAL_BLUE: Color = Color(65. / 255., 105. / 255., 225. / 255., 1.);
    pub const BLUE_VIOLET: Color = Color(138. / 255., 43. / 255., 226. / 255., 1.);
    pub const INDIGO: Color = Color(75. / 255., 0. / 255., 130. / 255., 1.);
    pub const DARK_SLATE_BLUE: Color = Color(72. / 255., 61. / 255., 139. / 255., 1.);
    pub const SLATE_BLUE: Color = Color(106. / 255., 90. / 255., 205. / 255., 1.);
    pub const MEDIUM_SLATE_BLUE: Color = Color(123. / 255., 104. / 255., 238. / 255., 1.);
    pub const MEDIUM_PURPLE: Color = Color(147. / 255., 112. / 255., 219. / 255., 1.);
    pub const DARK_MAGENTA: Color = Color(139. / 255., 0. / 255., 139. / 255., 1.);
    pub const DARK_VIOLET: Color = Color(148. / 255., 0. / 255., 211. / 255., 1.);
    pub const DARK_ORCHID: Color = Color(153. / 255., 50. / 255., 204. / 255., 1.);
    pub const MEDIUM_ORCHID: Color = Color(186. / 255., 85. / 255., 211. / 255., 1.);
    pub const PURPLE: Color = Color(128. / 255., 0. / 255., 128. / 255., 1.);
    pub const THISTLE: Color = Color(216. / 255., 191. / 255., 216. / 255., 1.);
    pub const PLUM: Color = Color(221. / 255., 160. / 255., 221. / 255., 1.);
    pub const VIOLET: Color = Color(238. / 255., 130. / 255., 238. / 255., 1.);
    pub const MAGENTA: Color = Color(1., 0. / 255., 1., 1.);
    pub const ORCHID: Color = Color(218. / 255., 112. / 255., 214. / 255., 1.);
    pub const MEDIUM_VIOLET_RED: Color = Color(199. / 255., 21. / 255., 133. / 255., 1.);
    pub const PALE_VIOLET_RED: Color = Color(219. / 255., 112. / 255., 147. / 255., 1.);
    pub const DEEP_PINK: Color = Color(1., 20. / 255., 147. / 255., 1.);
    pub const HOT_PINK: Color = Color(1., 105. / 255., 180. / 255., 1.);
    pub const LIGHT_PINK: Color = Color(1., 182. / 255., 193. / 255., 1.);
    pub const PINK: Color = Color(1., 192. / 255., 203. / 255., 1.);
    pub const ANTIQUE_WHITE: Color = Color(250. / 255., 235. / 255., 215. / 255., 1.);
    pub const BEIGE: Color = Color(245. / 255., 245. / 255., 220. / 255., 1.);
    pub const BISQUE: Color = Color(1., 228. / 255., 196. / 255., 1.);
    pub const BLANCHED_ALMOND: Color = Color(1., 235. / 255., 205. / 255., 1.);
    pub const WHEAT: Color = Color(245. / 255., 222. / 255., 179. / 255., 1.);
    pub const CORN_SILK: Color = Color(1., 248. / 255., 220. / 255., 1.);
    pub const LEMON_CHIFFON: Color = Color(1., 250. / 255., 205. / 255., 1.);
    pub const LIGHT_GOLDEN_ROD_YELLOW: Color = Color(250. / 255., 250. / 255., 210. / 255., 1.);
    pub const LIGHT_YELLOW: Color = Color(1., 1., 224. / 255., 1.);
    pub const SADDLE_BROWN: Color = Color(139. / 255., 69. / 255., 19. / 255., 1.);
    pub const SIENNA: Color = Color(160. / 255., 82. / 255., 45. / 255., 1.);
    pub const CHOCOLATE: Color = Color(210. / 255., 105. / 255., 30. / 255., 1.);
    pub const PERU: Color = Color(205. / 255., 133. / 255., 63. / 255., 1.);
    pub const SANDY_BROWN: Color = Color(244. / 255., 164. / 255., 96. / 255., 1.);
    pub const BURLY_WOOD: Color = Color(222. / 255., 184. / 255., 135. / 255., 1.);
    pub const TAN: Color = Color(210. / 255., 180. / 255., 140. / 255., 1.);
    pub const ROSY_BROWN: Color = Color(188. / 255., 143. / 255., 143. / 255., 1.);
    pub const MOCCASIN: Color = Color(1., 228. / 255., 181. / 255., 1.);
    pub const NAVAJO_WHITE: Color = Color(1., 222. / 255., 173. / 255., 1.);
    pub const PEACH_PUFF: Color = Color(1., 218. / 255., 185. / 255., 1.);
    pub const MISTY_ROSE: Color = Color(1., 228. / 255., 225. / 255., 1.);
    pub const LAVENDER_BLUSH: Color = Color(1., 240. / 255., 245. / 255., 1.);
    pub const LINEN: Color = Color(250. / 255., 240. / 255., 230. / 255., 1.);
    pub const OLD_LACE: Color = Color(253. / 255., 245. / 255., 230. / 255., 1.);
    pub const PAPAYA_WHIP: Color = Color(1., 239. / 255., 213. / 255., 1.);
    pub const SEA_SHELL: Color = Color(1., 245. / 255., 238. / 255., 1.);
    pub const MINT_CREAM: Color = Color(245. / 255., 1., 250. / 255., 1.);
    pub const SLATE_GRAY: Color = Color(112. / 255., 128. / 255., 144. / 255., 1.);
    pub const LIGHT_SLATE_GRAY: Color = Color(119. / 255., 136. / 255., 153. / 255., 1.);
    pub const LIGHT_STEEL_BLUE: Color = Color(176. / 255., 196. / 255., 222. / 255., 1.);
    pub const LAVENDER: Color = Color(230. / 255., 230. / 255., 250. / 255., 1.);
    pub const FLORAL_WHITE: Color = Color(1., 250. / 255., 240. / 255., 1.);
    pub const ALICE_BLUE: Color = Color(240. / 255., 248. / 255., 1., 1.);
    pub const GHOST_WHITE: Color = Color(248. / 255., 248. / 255., 1., 1.);
    pub const HONEYDEW: Color = Color(240. / 255., 1., 240. / 255., 1.);
    pub const IVORY: Color = Color(1., 1., 240. / 255., 1.);
    pub const AZURE: Color = Color(240. / 255., 1., 1., 1.);
    pub const SNOW: Color = Color(1., 250. / 255., 250. / 255., 1.);
    pub const BLACK: Color = Color(0. / 255., 0. / 255., 0. / 255., 1.);
    pub const DIM_GRAY: Color = Color(105. / 255., 105. / 255., 105. / 255., 1.);
    pub const GRAY: Color = Color(128. / 255., 128. / 255., 128. / 255., 1.);
    pub const DARK_GRAY: Color = Color(169. / 255., 169. / 255., 169. / 255., 1.);
    pub const SILVER: Color = Color(192. / 255., 192. / 255., 192. / 255., 1.);
    pub const LIGHT_GRAY: Color = Color(211. / 255., 211. / 255., 211. / 255., 1.);
    pub const GAINSBORO: Color = Color(220. / 255., 220. / 255., 220. / 255., 1.);
    pub const WHITE_SMOKE: Color = Color(245. / 255., 245. / 255., 245. / 255., 1.);
    pub const WHITE: Color = Color(1., 1., 1., 1.);
}

impl Color {
    pub fn to_rgb_array(self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }

    pub fn to_rgba_array(self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn with_alpha(self, alpha: f32) -> Color {
        Color(self.0, self.1, self.2, alpha)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinePoint {
    vertex: Vf2,
}
implement_vertex!(LinePoint, vertex);

impl LinePoint {
    pub fn from_point<U>(vec: Point2D<f32, U>) -> LinePoint {
        LinePoint {
            vertex: Vf2::new(vec.x, vec.y),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColoredPoint {
    pub vertex: Vf2,
    pub color: Color,
}
implement_vertex!(ColoredPoint, vertex, color);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    start: Vf2,
    end: Vf2,
}
implement_vertex!(Line, start, end);

impl Line {
    pub fn from_points<U>(start: Point2D<f32, U>, end: Point2D<f32, U>) -> Line {
        Line {
            start: Vf2::new(start.x, start.y),
            end: Vf2::new(end.x, end.y),
        }
    }
    pub fn from_line<U>(line: GLine<U>) -> Line {
        Line::from_points(line.start, line.end)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColoredLine {
    pub start: Vf2,
    pub end: Vf2,
    pub color: Color,
}
implement_vertex!(ColoredLine, start, end, color);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    pub vertex: Vf2,
    pub size: Vf2,
    pub texture_index: i32,
}
implement_vertex!(Sprite, vertex, size, texture_index);
