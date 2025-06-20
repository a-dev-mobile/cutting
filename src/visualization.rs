use crate::engine::model::{Solution, CalculationResponse};
use crate::error::CuttingError;
use std::fs;

/// Генератор HTML визуализации для результатов раскроя
pub struct HtmlVisualizer;

impl HtmlVisualizer {
    /// Создает HTML визуализацию из решения
    pub fn generate_from_solution(solution: &Solution, output_path: &str) -> Result<(), CuttingError> {
        let html = Self::create_solution_html(solution);
        fs::write(output_path, html)
            .map_err(|e| CuttingError::GeneralCuttingError(format!("Failed to write HTML file: {}", e)))?;
        Ok(())
    }

    /// Создает HTML визуализацию из ответа расчета
    pub fn generate_from_response(response: &CalculationResponse, output_path: &str) -> Result<(), CuttingError> {
        let html = Self::create_response_html(response);
        fs::write(output_path, html)
            .map_err(|e| CuttingError::GeneralCuttingError(format!("Failed to write HTML file: {}", e)))?;
        Ok(())
    }

    /// Создает HTML для решения
    fn create_solution_html(solution: &Solution) -> String {
        let mut html = String::new();
        
        // HTML заголовок
        html.push_str(&Self::html_header("Визуализация раскроя"));
        
        // Информация о решении
        html.push_str(&format!(
            r#"
            <div class="info-panel">
                <h2>Информация о решении</h2>
                <div class="stats">
                    <div class="stat">
                        <span class="label">Заготовок:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Размещено деталей:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Не размещено:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Эффективность:</span>
                        <span class="value">{:.1}%</span>
                    </div>
                </div>
            </div>
            "#,
            solution.get_mosaics().len(),
            solution.get_nbr_final_tiles(),
            solution.get_no_fit_panels().len(),
            solution.get_efficiency()
        ));

        // Визуализация заготовок
        html.push_str(r#"<div class="visualization-container">"#);
        
        for (i, mosaic) in solution.get_mosaics().iter().enumerate() {
            html.push_str(&Self::create_mosaic_svg(mosaic, i + 1));
        }
        
        html.push_str(r#"</div>"#);

        // Неразмещенные детали
        if !solution.get_no_fit_panels().is_empty() {
            html.push_str(&Self::create_unplaced_panels_section(solution.get_no_fit_panels()));
        }

        // HTML подвал
        html.push_str(&Self::html_footer());
        
        html
    }

    /// Создает HTML для ответа расчета
    fn create_response_html(response: &CalculationResponse) -> String {
        let mut html = String::new();
        
        // HTML заголовок
        html.push_str(&Self::html_header("Результат оптимизации раскроя"));
        
        // Информация о результате
        html.push_str(&format!(
            r#"
            <div class="info-panel">
                <h2>Статистика</h2>
                <div class="stats">
                    <div class="stat">
                        <span class="label">Всего панелей:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Размещено:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Не поместилось:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Без материала:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Эффективность:</span>
                        <span class="value">{:.1}%</span>
                    </div>
                </div>
            </div>
            "#,
            response.statistics.total_panels,
            response.statistics.placed_panels,
            response.no_fit_panels.len(),
            response.no_material_panels.len(),
            response.statistics.efficiency_percentage
        ));

        // Группировка панелей по складским панелям
        let mut stock_panels = std::collections::HashMap::new();
        for panel in &response.panels {
            stock_panels.entry(panel.stock_panel_id.clone())
                .or_insert_with(Vec::new)
                .push(panel);
        }

        // Визуализация складских панелей
        html.push_str(r#"<div class="visualization-container">"#);
        
        for (i, (stock_id, panels)) in stock_panels.iter().enumerate() {
            html.push_str(&Self::create_stock_panel_svg(stock_id, panels, i + 1));
        }
        
        html.push_str(r#"</div>"#);

        // Неразмещенные детали
        if !response.no_fit_panels.is_empty() || !response.no_material_panels.is_empty() {
            html.push_str(&Self::create_unplaced_response_panels_section(response));
        }

        // HTML подвал
        html.push_str(&Self::html_footer());
        
        html
    }

    /// Создает SVG для мозаики
    fn create_mosaic_svg(mosaic: &crate::engine::model::Mosaic, index: usize) -> String {
        let root = mosaic.get_root_tile_node();
        let width = root.get_width();
        let height = root.get_height();
        
        // Масштабирование для отображения
        let scale = Self::calculate_scale(width, height, 400, 300);
        let svg_width = (width as f64 * scale) as i32;
        let svg_height = (height as f64 * scale) as i32;

        let mut svg = format!(
            r#"
            <div class="stock-panel">
                <h3>Заготовка {} ({}x{} мм)</h3>
                <svg width="{}" height="{}" viewBox="0 0 {} {}">
                    <!-- Контур заготовки -->
                    <rect x="0" y="0" width="{}" height="{}" 
                          fill="none" stroke="rgb(51,51,51)" stroke-width="2"/>
            "#,
            index, width, height,
            svg_width, svg_height, width, height,
            width, height
        );

        // Добавляем размещенные детали
        let final_nodes = root.get_final_tile_nodes();
        for (i, node) in final_nodes.iter().enumerate() {
            let color = Self::get_color(i);
            svg.push_str(&format!(
                r#"
                    <!-- Деталь {} -->
                    <rect x="{}" y="{}" width="{}" height="{}" 
                          fill="{}" stroke="rgb(0,0,0)" stroke-width="1" opacity="0.8"/>
                    <text x="{}" y="{}" font-family="Arial" font-size="12" 
                          text-anchor="middle" dominant-baseline="middle" fill="rgb(0,0,0)">
                        ID:{}
                    </text>
                "#,
                i + 1,
                node.get_x1(), node.get_y1(), 
                node.get_width(), node.get_height(),
                color,
                node.get_x1() + node.get_width() / 2,
                node.get_y1() + node.get_height() / 2,
                node.external_id
            ));
        }

        svg.push_str(r#"
                </svg>
            </div>
        "#);

        svg
    }

    /// Создает SVG для складской панели из ответа
    fn create_stock_panel_svg(stock_id: &str, panels: &[&crate::engine::model::OptimizedPanel], index: usize) -> String {
        // Определяем размеры складской панели
        let mut max_x = 0;
        let mut max_y = 0;
        
        for panel in panels {
            max_x = max_x.max(panel.position.right());
            max_y = max_y.max(panel.position.bottom());
        }

        // Если нет панелей, используем размеры по умолчанию
        if max_x == 0 || max_y == 0 {
            max_x = 1000;
            max_y = 600;
        }

        // Масштабирование для отображения
        let scale = Self::calculate_scale(max_x, max_y, 400, 300);
        let svg_width = (max_x as f64 * scale) as i32;
        let svg_height = (max_y as f64 * scale) as i32;

        let mut svg = format!(
            r#"
            <div class="stock-panel">
                <h3>Заготовка {} - {} ({}x{} мм)</h3>
                <svg width="{}" height="{}" viewBox="0 0 {} {}">
                    <!-- Контур заготовки -->
                    <rect x="0" y="0" width="{}" height="{}" 
                          fill="none" stroke="rgb(51,51,51)" stroke-width="2"/>
            "#,
            index, stock_id, max_x, max_y,
            svg_width, svg_height, max_x, max_y,
            max_x, max_y
        );

        // Добавляем размещенные панели
        for (i, panel) in panels.iter().enumerate() {
            let color = Self::get_color(i);
            svg.push_str(&format!(
                r#"
                    <!-- Панель {} -->
                    <rect x="{}" y="{}" width="{}" height="{}" 
                          fill="{}" stroke="rgb(0,0,0)" stroke-width="1" opacity="0.8"/>
                    <text x="{}" y="{}" font-family="Arial" font-size="12" 
                          text-anchor="middle" dominant-baseline="middle" fill="rgb(0,0,0)">
                        ID:{}{}
                    </text>
                "#,
                i + 1,
                panel.position.x, panel.position.y, 
                panel.position.width, panel.position.height,
                color,
                panel.position.x + panel.position.width / 2,
                panel.position.y + panel.position.height / 2,
                panel.tile_dimensions.id,
                if panel.position.rotated { "↻" } else { "" }
            ));
        }

        svg.push_str(r#"
                </svg>
            </div>
        "#);

        svg
    }

    /// Создает секцию неразмещенных панелей для решения
    fn create_unplaced_panels_section(panels: &[crate::engine::model::TileDimensions]) -> String {
        let mut html = String::new();
        
        html.push_str(r#"
            <div class="unplaced-section">
                <h3>Неразмещенные детали</h3>
                <div class="unplaced-panels">
        "#);

        for panel in panels {
            html.push_str(&format!(
                r#"
                <div class="unplaced-panel">
                    <div class="panel-info">
                        <span class="panel-id">ID: {}</span>
                        <span class="panel-size">{}x{} мм</span>
                        <span class="panel-material">{}</span>
                    </div>
                </div>
                "#,
                panel.id,
                panel.width, panel.height,
                panel.material
            ));
        }

        html.push_str(r#"
                </div>
            </div>
        "#);

        html
    }

    /// Создает секцию неразмещенных панелей для ответа
    fn create_unplaced_response_panels_section(response: &CalculationResponse) -> String {
        let mut html = String::new();
        
        html.push_str(r#"
            <div class="unplaced-section">
                <h3>Неразмещенные детали</h3>
        "#);

        if !response.no_fit_panels.is_empty() {
            html.push_str(r#"
                <h4>Не поместились:</h4>
                <div class="unplaced-panels">
            "#);

            for panel in &response.no_fit_panels {
                html.push_str(&format!(
                    r#"
                    <div class="unplaced-panel no-fit">
                        <div class="panel-info">
                            <span class="panel-id">ID: {}</span>
                            <span class="panel-size">{}x{} мм</span>
                            <span class="panel-material">{}</span>
                        </div>
                    </div>
                    "#,
                    panel.id,
                    panel.width, panel.height,
                    panel.material
                ));
            }

            html.push_str(r#"</div>"#);
        }

        if !response.no_material_panels.is_empty() {
            html.push_str(r#"
                <h4>Нет подходящего материала:</h4>
                <div class="unplaced-panels">
            "#);

            for panel in &response.no_material_panels {
                html.push_str(&format!(
                    r#"
                    <div class="unplaced-panel no-material">
                        <div class="panel-info">
                            <span class="panel-id">ID: {}</span>
                            <span class="panel-size">{}x{} мм</span>
                            <span class="panel-material">{}</span>
                        </div>
                    </div>
                    "#,
                    panel.id,
                    panel.width, panel.height,
                    panel.material
                ));
            }

            html.push_str(r#"</div>"#);
        }

        html.push_str(r#"</div>"#);

        html
    }

    /// Вычисляет масштаб для отображения
    fn calculate_scale(width: i32, height: i32, max_width: i32, max_height: i32) -> f64 {
        let scale_x = max_width as f64 / width as f64;
        let scale_y = max_height as f64 / height as f64;
        scale_x.min(scale_y).min(1.0)
    }

    /// Получает цвет для детали по индексу
    fn get_color(index: usize) -> &'static str {
        const COLORS: &[&str] = &[
            "rgb(255, 107, 107)", "rgb(78, 205, 196)", "rgb(69, 183, 209)", 
            "rgb(150, 206, 180)", "rgb(255, 234, 167)", "rgb(221, 160, 221)", 
            "rgb(152, 216, 200)", "rgb(247, 220, 111)", "rgb(187, 143, 206)", 
            "rgb(133, 193, 233)", "rgb(248, 196, 113)", "rgb(130, 224, 170)", 
            "rgb(241, 148, 138)", "rgb(133, 193, 233)", "rgb(215, 189, 226)"
        ];
        COLORS[index % COLORS.len()]
    }

    /// Создает HTML заголовок
    fn html_header(title: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }}
        
        h1 {{
            text-align: center;
            color: #333;
            margin-bottom: 30px;
        }}
        
        .info-panel {{
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            margin-bottom: 30px;
        }}
        
        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
        }}
        
        .stat {{
            display: flex;
            justify-content: space-between;
            padding: 10px;
            background: #f8f9fa;
            border-radius: 4px;
        }}
        
        .label {{
            font-weight: bold;
            color: #666;
        }}
        
        .value {{
            color: #333;
            font-weight: bold;
        }}
        
        .visualization-container {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
            gap: 30px;
            margin-bottom: 30px;
        }}
        
        .stock-panel {{
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            text-align: center;
        }}
        
        .stock-panel h3 {{
            margin-top: 0;
            color: #333;
            border-bottom: 2px solid #eee;
            padding-bottom: 10px;
        }}
        
        .stock-panel svg {{
            border: 1px solid #ddd;
            border-radius: 4px;
            background: white;
        }}
        
        .unplaced-section {{
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            margin-top: 30px;
        }}
        
        .unplaced-section h3 {{
            color: #d32f2f;
            margin-top: 0;
            border-bottom: 2px solid #ffebee;
            padding-bottom: 10px;
        }}
        
        .unplaced-section h4 {{
            color: #666;
            margin-bottom: 15px;
        }}
        
        .unplaced-panels {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 15px;
        }}
        
        .unplaced-panel {{
            padding: 15px;
            border-radius: 4px;
            border-left: 4px solid #d32f2f;
        }}
        
        .unplaced-panel.no-fit {{
            background: #ffebee;
            border-left-color: #d32f2f;
        }}
        
        .unplaced-panel.no-material {{
            background: #fff3e0;
            border-left-color: #f57c00;
        }}
        
        .panel-info {{
            display: flex;
            flex-direction: column;
            gap: 5px;
        }}
        
        .panel-id {{
            font-weight: bold;
            color: #333;
        }}
        
        .panel-size {{
            color: #666;
            font-size: 14px;
        }}
        
        .panel-material {{
            color: #888;
            font-size: 12px;
        }}
        
        @media (max-width: 768px) {{
            .visualization-container {{
                grid-template-columns: 1fr;
            }}
            
            .stats {{
                grid-template-columns: 1fr;
            }}
            
            .unplaced-panels {{
                grid-template-columns: 1fr;
            }}
        }}
    </style>
</head>
<body>
    <h1>{}</h1>
"#,
            title, title
        )
    }

    /// Создает HTML подвал
    fn html_footer() -> String {
        r#"
</body>
</html>"#.to_string()
    }
}
