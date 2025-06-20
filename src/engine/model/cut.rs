use serde::{Deserialize, Serialize};

/// Тип разреза
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CutType {
    Horizontal,
    Vertical,
}

/// Информация о разрезе
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cut {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub original_width: i32,
    pub original_height: i32,
    pub is_horizontal: bool,
    pub cut_coord: i32,
    pub original_tile_id: i32,
    pub child1_tile_id: i32,
    pub child2_tile_id: i32,
}

impl Cut {
    /// Создать новый разрез
    pub fn new(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        original_width: i32,
        original_height: i32,
        is_horizontal: bool,
        cut_coord: i32,
        original_tile_id: i32,
        child1_tile_id: i32,
        child2_tile_id: i32,
    ) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            original_width,
            original_height,
            is_horizontal,
            cut_coord,
            original_tile_id,
            child1_tile_id,
            child2_tile_id,
        }
    }

    /// Получить длину разреза
    pub fn get_length(&self) -> i64 {
        ((self.x2 - self.x1).abs() + (self.y2 - self.y1).abs()) as i64
    }

    /// Проверить, является ли горизонтальным
    pub fn get_is_horizontal(&self) -> bool {
        self.is_horizontal
    }

    /// Получить координату разреза
    pub fn get_cut_coords(&self) -> i32 {
        self.cut_coord
    }

    /// Получить координату разреза (альтернативное название)
    pub fn cut_coord(&self) -> i32 {
        self.cut_coord
    }
}

/// Строитель для создания разрезов
#[derive(Debug, Default)]
pub struct CutBuilder {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    original_width: i32,
    original_height: i32,
    is_horizontal: bool,
    cut_coord: i32,
    original_tile_id: i32,
    child1_tile_id: i32,
    child2_tile_id: i32,
}

impl CutBuilder {
    /// Создать новый строитель
    pub fn new() -> Self {
        Self::default()
    }

    /// Установить X1
    pub fn set_x1(mut self, x1: i32) -> Self {
        self.x1 = x1;
        self
    }

    /// Установить Y1
    pub fn set_y1(mut self, y1: i32) -> Self {
        self.y1 = y1;
        self
    }

    /// Установить X2
    pub fn set_x2(mut self, x2: i32) -> Self {
        self.x2 = x2;
        self
    }

    /// Установить Y2
    pub fn set_y2(mut self, y2: i32) -> Self {
        self.y2 = y2;
        self
    }

    /// Установить оригинальную ширину
    pub fn set_original_width(mut self, width: i32) -> Self {
        self.original_width = width;
        self
    }

    /// Установить оригинальную высоту
    pub fn set_original_height(mut self, height: i32) -> Self {
        self.original_height = height;
        self
    }

    /// Установить горизонтальность
    pub fn set_horizontal(mut self, is_horizontal: bool) -> Self {
        self.is_horizontal = is_horizontal;
        self
    }

    /// Установить координату разреза
    pub fn set_cut_coords(mut self, cut_coord: i32) -> Self {
        self.cut_coord = cut_coord;
        self
    }

    /// Установить ID оригинального узла
    pub fn set_original_tile_id(mut self, id: i32) -> Self {
        self.original_tile_id = id;
        self
    }

    /// Установить ID первого дочернего узла
    pub fn set_child1_tile_id(mut self, id: i32) -> Self {
        self.child1_tile_id = id;
        self
    }

    /// Установить ID второго дочернего узла
    pub fn set_child2_tile_id(mut self, id: i32) -> Self {
        self.child2_tile_id = id;
        self
    }

    /// Построить разрез
    pub fn build(self) -> Cut {
        Cut::new(
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.original_width,
            self.original_height,
            self.is_horizontal,
            self.cut_coord,
            self.original_tile_id,
            self.child1_tile_id,
            self.child2_tile_id,
        )
    }
}
