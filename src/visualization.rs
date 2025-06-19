//! Модуль для создания HTML визуализации размещения панелей
//! 
//! Этот модуль содержит функции для генерации интерактивной HTML страницы,
//! которая показывает как панели размещены на листе материала.

use crate::engine::model::response::{OptimizedPanel, ResponseStatistics};

/// Создает HTML визуализацию размещения панелей
/// 
/// # Параметры
/// * `panels` - Список оптимизированных панелей с их позициями
/// * `stats` - Статистика оптимизации
/// * `sheet_width` - Ширина листа материала (по умолчанию 1000)
/// * `sheet_height` - Высота листа материала (по умолчанию 600)
/// 
/// # Возвращает
/// Строку с HTML кодом для визуализации
pub fn create_html_visualization(
    panels: &[OptimizedPanel], 
    stats: &ResponseStatistics,
    sheet_width: Option<i32>,
    sheet_height: Option<i32>
) -> String {
    let sheet_w = sheet_width.unwrap_or(1000);
    let sheet_h = sheet_height.unwrap_or(600);
    
    let mut html = String::new();
    
    // HTML заголовок и стили
    html.push_str(&create_html_header());
    
    // Статистика
    html.push_str(&create_stats_section(stats));
    
    // Визуализация
    html.push_str(&create_visualization_section(panels, sheet_w, sheet_h));
    
    // Легенда
    html.push_str(&create_legend_section());
    
    // JavaScript для интерактивности
    html.push_str(&create_javascript_section());
    
    // Закрывающие теги
    html.push_str("</body>\n</html>");
    
    html
}

fn create_html_header() -> String {
    r#"<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Визуализация размещения панелей</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #333;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        
        .header {
            background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }
        
        .header h1 {
            margin: 0;
            font-size: 2.5em;
            font-weight: 300;
        }
        
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            padding: 30px;
            background: #f8f9fa;
        }
        
        .stat-card {
            background: white;
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 5px 15px rgba(0,0,0,0.08);
            text-align: center;
        }
        
        .stat-value {
            font-size: 2em;
            font-weight: bold;
            color: #4facfe;
            margin-bottom: 5px;
        }
        
        .stat-label {
            color: #666;
            font-size: 0.9em;
        }
        
        .visualization {
            padding: 30px;
            text-align: center;
        }
        
        .sheet {
            position: relative;
            border: 3px solid #333;
            margin: 20px auto;
            background: #f0f0f0;
            border-radius: 5px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
        }
        
        .panel {
            position: absolute;
            border: 2px solid #333;
            border-radius: 3px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: bold;
            font-size: 12px;
            color: white;
            text-shadow: 1px 1px 2px rgba(0,0,0,0.7);
            cursor: pointer;
            transition: all 0.3s ease;
        }
        
        .panel:hover {
            transform: scale(1.05);
            z-index: 10;
            box-shadow: 0 5px 15px rgba(0,0,0,0.3);
        }
        
        .panel-300x200 { background: linear-gradient(45deg, #ff6b6b, #ee5a52); }
        .panel-150x100 { background: linear-gradient(45deg, #4ecdc4, #44a08d); }
        .panel-200x250 { background: linear-gradient(45deg, #45b7d1, #96c93d); }
        .panel-100x80 { background: linear-gradient(45deg, #f9ca24, #f0932b); }
        .panel-50x50 { background: linear-gradient(45deg, #eb4d4b, #6c5ce7); }
        .panel-other { background: linear-gradient(45deg, #a55eea, #26de81); }
        
        .legend {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin: 30px 0;
            padding: 0 30px;
        }
        
        .legend-item {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 10px;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }
        
        .legend-color {
            width: 30px;
            height: 20px;
            border-radius: 3px;
            border: 1px solid #333;
        }
        
        .tooltip {
            position: absolute;
            background: rgba(0,0,0,0.9);
            color: white;
            padding: 8px 12px;
            border-radius: 5px;
            font-size: 12px;
            pointer-events: none;
            z-index: 1000;
            opacity: 0;
            transition: opacity 0.3s ease;
        }
        
        .efficiency-bar {
            width: 100%;
            height: 20px;
            background: #e0e0e0;
            border-radius: 10px;
            overflow: hidden;
            margin-top: 10px;
        }
        
        .efficiency-fill {
            height: 100%;
            background: linear-gradient(90deg, #4facfe, #00f2fe);
            transition: width 1s ease;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🎨 Визуализация размещения панелей</h1>
            <p>Интерактивная схема оптимального размещения</p>
        </div>
"#.to_string()
}

fn create_stats_section(stats: &ResponseStatistics) -> String {
    format!(r#"        <div class="stats">
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Размещено панелей</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{:.1}%</div>
                <div class="stat-label">Эффективность</div>
                <div class="efficiency-bar">
                    <div class="efficiency-fill" style="width: {:.1}%"></div>
                </div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Использованная площадь</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Потерянная площадь</div>
            </div>
        </div>
"#, 
        stats.placed_panels,
        stats.efficiency_percentage,
        stats.efficiency_percentage,
        stats.used_area as i32,
        stats.wasted_area as i32
    )
}

fn create_visualization_section(panels: &[OptimizedPanel], sheet_width: i32, sheet_height: i32) -> String {
    let mut section = format!(r#"        <div class="visualization">
            <h2>Лист материала {}×{} мм</h2>
            <div class="sheet" id="sheet" style="width: {}px; height: {}px;">
"#, 
        sheet_width, sheet_height,
        (sheet_width as f64 * 0.8) as i32,  // Масштаб для отображения
        (sheet_height as f64 * 0.8) as i32
    );
    
    // Вычисляем масштаб для отображения
    let scale_x = (sheet_width as f64 * 0.8) / sheet_width as f64;
    let scale_y = (sheet_height as f64 * 0.8) / sheet_height as f64;
    
    // Добавляем панели
    for (i, panel) in panels.iter().enumerate() {
        let x = (panel.position.x as f64 * scale_x) as i32;
        let y = (panel.position.y as f64 * scale_y) as i32;
        let width = (panel.position.width as f64 * scale_x) as i32;
        let height = (panel.position.height as f64 * scale_y) as i32;
        
        let panel_class = get_panel_css_class(panel.tile_dimensions.width, panel.tile_dimensions.height);
        let panel_text = format!("{}×{}", panel.tile_dimensions.width, panel.tile_dimensions.height);
        
        section.push_str(&format!(
            r#"                <div class="panel {}" 
                     style="left: {}px; top: {}px; width: {}px; height: {}px;"
                     data-panel="{}"
                     data-size="{}×{}"
                     data-position="({}, {})"
                     data-area="{}">
                    {}
                </div>
"#,
            panel_class, x, y, width, height, i + 1,
            panel.tile_dimensions.width, panel.tile_dimensions.height,
            panel.position.x, panel.position.y,
            panel.position.area(),
            panel_text
        ));
    }
    
    section.push_str("            </div>\n");
    section
}

fn create_legend_section() -> String {
    r#"            <div class="legend">
                <div class="legend-item">
                    <div class="legend-color panel-300x200"></div>
                    <span>Панель 300×200 мм</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color panel-150x100"></div>
                    <span>Панель 150×100 мм</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color panel-200x250"></div>
                    <span>Панель 200×250 мм</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color panel-100x80"></div>
                    <span>Панель 100×80 мм</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color panel-50x50"></div>
                    <span>Панель 50×50 мм</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color panel-other"></div>
                    <span>Другие панели</span>
                </div>
            </div>
        </div>
    </div>
    
    <div class="tooltip" id="tooltip"></div>
"#.to_string()
}

fn create_javascript_section() -> String {
    r#"    <script>
        // Интерактивность
        const panels = document.querySelectorAll('.panel');
        const tooltip = document.getElementById('tooltip');
        
        panels.forEach(panel => {
            panel.addEventListener('mouseenter', (e) => {
                const panelNum = e.target.dataset.panel;
                const size = e.target.dataset.size;
                const position = e.target.dataset.position;
                const area = e.target.dataset.area;
                
                tooltip.innerHTML = `
                    <strong>Панель ${panelNum}</strong><br>
                    Размер: ${size} мм<br>
                    Позиция: ${position}<br>
                    Площадь: ${area} мм²
                `;
                tooltip.style.opacity = '1';
            });
            
            panel.addEventListener('mouseleave', () => {
                tooltip.style.opacity = '0';
            });
            
            panel.addEventListener('mousemove', (e) => {
                tooltip.style.left = e.pageX + 10 + 'px';
                tooltip.style.top = e.pageY - 10 + 'px';
            });
        });
        
        // Анимация загрузки
        setTimeout(() => {
            panels.forEach((panel, index) => {
                setTimeout(() => {
                    panel.style.opacity = '0';
                    panel.style.transform = 'scale(0)';
                    setTimeout(() => {
                        panel.style.opacity = '1';
                        panel.style.transform = 'scale(1)';
                        panel.style.transition = 'all 0.5s ease';
                    }, 50);
                }, index * 100);
            });
        }, 500);
    </script>
"#.to_string()
}

/// Создает JSON данные для экспорта
pub fn create_json_data(panels: &[OptimizedPanel], stats: &ResponseStatistics) -> String {
    let mut json = String::from("{\n");
    json.push_str(&format!("  \"statistics\": {{\n"));
    json.push_str(&format!("    \"placed_panels\": {},\n", stats.placed_panels));
    json.push_str(&format!("    \"total_panels\": {},\n", stats.total_panels));
    json.push_str(&format!("    \"efficiency_percentage\": {:.2},\n", stats.efficiency_percentage));
    json.push_str(&format!("    \"used_area\": {:.0},\n", stats.used_area));
    json.push_str(&format!("    \"wasted_area\": {:.0}\n", stats.wasted_area));
    json.push_str("  },\n");
    
    json.push_str("  \"panels\": [\n");
    for (i, panel) in panels.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"id\": {},\n", i + 1));
        json.push_str(&format!("      \"width\": {},\n", panel.tile_dimensions.width));
        json.push_str(&format!("      \"height\": {},\n", panel.tile_dimensions.height));
        json.push_str(&format!("      \"x\": {},\n", panel.position.x));
        json.push_str(&format!("      \"y\": {},\n", panel.position.y));
        json.push_str(&format!("      \"area\": {},\n", panel.position.area()));
        json.push_str(&format!("      \"rotated\": {}\n", panel.position.rotated));
        json.push_str("    }");
        if i < panels.len() - 1 {
            json.push_str(",");
        }
        json.push_str("\n");
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    
    json
}

/// Определяет CSS класс для панели на основе её размеров
fn get_panel_css_class(width: i32, height: i32) -> &'static str {
    match (width, height) {
        (300, 200) | (200, 300) => "panel-300x200",
        (150, 100) | (100, 150) => "panel-150x100",
        (200, 250) | (250, 200) => "panel-200x250",
        (100, 80) | (80, 100) => "panel-100x80",
        (50, 50) => "panel-50x50",
        _ => "panel-other",
    }
}
