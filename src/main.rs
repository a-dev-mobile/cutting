use clap::{Parser, Subcommand};
use cutting_cli::engine::service::CutListOptimizerService;
use cutting_cli::engine::logger::CutListLoggerImpl;
use cutting_cli::engine::model::{
    CalculationRequest, ClientInfo, Configuration, Panel, PerformanceThresholds, CalculationResponse
};
use cutting_cli::error::CuttingError;
use cutting_cli::visualization::HtmlVisualizer;
use serde_json;
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::fs;
use std::path::Path;

/// CLI для системы оптимизации раскроя
#[derive(Parser)]
#[command(name = "cutting-cli")]
#[command(about = "Система оптимизации раскроя материалов")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Запустить оптимизацию раскроя
    Optimize {
        /// Путь к файлу с входными данными (JSON)
        #[arg(short, long)]
        input: String,
        
        /// Путь к файлу для сохранения результата (JSON)
        #[arg(short, long)]
        output: Option<String>,
        
        /// Количество потоков для обработки
        #[arg(short, long, default_value = "4")]
        threads: usize,
        
        /// Таймаут в секундах
        #[arg(short = 'T', long, default_value = "300")]
        timeout: u64,
        
        /// Показать подробный вывод
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Отправить задачу на выполнение (асинхронно)
    Submit {
        /// Путь к файлу с входными данными (JSON)
        #[arg(short, long)]
        input: String,
        
        /// Количество потоков для обработки
        #[arg(short, long, default_value = "4")]
        threads: usize,
        
        /// Показать подробный вывод
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Проверить статус задачи
    Status {
        /// Идентификатор задачи
        #[arg(short, long)]
        task_id: String,
        
        /// Показать подробную информацию
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Остановить задачу
    Stop {
        /// Идентификатор задачи
        #[arg(short, long)]
        task_id: String,
    },
    
    /// Принудительно завершить задачу
    Terminate {
        /// Идентификатор задачи
        #[arg(short, long)]
        task_id: String,
    },
    
    /// Получить список задач клиента
    List {
        /// Идентификатор клиента
        #[arg(short, long)]
        client_id: String,
        
        /// Фильтр по статусу
        #[arg(short, long)]
        status: Option<String>,
    },
    
    /// Получить статистику системы
    Stats,
    
    /// Демонстрация базовых алгоритмов
    Demo,
    
    /// Создать пример входного файла
    Example {
        /// Путь для сохранения примера
        #[arg(short, long, default_value = "example_input.json")]
        output: String,
    },
    
    /// Создать HTML визуализацию результатов
    Visualize {
        /// Путь к файлу с результатами оптимизации (JSON)
        #[arg(short, long)]
        input: String,
        
        /// Путь для сохранения HTML файла
        #[arg(short, long, default_value = "visualization.html")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Optimize { input, output, threads, timeout, verbose } => {
            if let Err(e) = run_optimization(input, output, threads, timeout, verbose) {
                eprintln!("❌ Ошибка оптимизации: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Submit { input, threads, verbose } => {
            if let Err(e) = submit_task(input, threads, verbose) {
                eprintln!("❌ Ошибка отправки задачи: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Status { task_id, verbose } => {
            if let Err(e) = check_status(task_id, verbose) {
                eprintln!("❌ Ошибка проверки статуса: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Stop { task_id } => {
            if let Err(e) = stop_task(task_id) {
                eprintln!("❌ Ошибка остановки задачи: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Terminate { task_id } => {
            if let Err(e) = terminate_task(task_id) {
                eprintln!("❌ Ошибка завершения задачи: {}", e);
                std::process::exit(1);
            }
        }
        Commands::List { client_id, status } => {
            if let Err(e) = list_tasks(client_id, status) {
                eprintln!("❌ Ошибка получения списка задач: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Stats => {
            if let Err(e) = show_stats() {
                eprintln!("❌ Ошибка получения статистики: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Demo => {
            run_demo();
        }
        Commands::Example { output } => {
            if let Err(e) = create_example(output) {
                eprintln!("❌ Ошибка создания примера: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Visualize { input, output } => {
            if let Err(e) = create_visualization(input, output) {
                eprintln!("❌ Ошибка создания визуализации: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_optimization(
    input_path: String,
    output_path: Option<String>,
    threads: usize,
    timeout: u64,
    verbose: bool,
) -> Result<(), CuttingError> {
    println!("🔧 Запуск оптимизации раскроя");
    println!("================================");
    
    // Загрузка входных данных
    let request = load_request_from_file(&input_path)?;
    
    if verbose {
        println!("📋 Входные данные:");
        println!("  - Панелей: {}", request.panels.len());
        println!("  - Стоковых панелей: {}", request.stock_panels.len());
        println!("  - Клиент: {}", request.client_info.id);
        println!("  - Потоков: {}", threads);
        println!();
    }
    
    // Создание и инициализация оптимизатора
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut optimizer = cutting_cli::engine::service::CutListOptimizerServiceImpl::new(logger.clone());
    
    optimizer.init(threads)?;
    
    println!("⚙️  Выполнение оптимизации...");
    
    // Запуск оптимизации с таймаутом
    let start_time = std::time::Instant::now();
    let result = run_with_timeout(
        move || optimizer.optimize(request),
        Duration::from_secs(timeout),
    )?;
    
    let elapsed = start_time.elapsed();
    
    println!("✅ Оптимизация завершена за {:.2} сек", elapsed.as_secs_f64());
    
    if verbose {
        println!("📊 Результаты:");
        println!("  - Размещено панелей: {}", result.statistics.placed_panels);
        println!("  - Не поместилось: {}", result.statistics.unplaced_panels);
        println!("  - Без материала: {}", result.no_material_panels.len());
        println!("  - Эффективность: {:.2}%", result.statistics.efficiency_percentage);
        println!("  - Использовано стоковых панелей: {}", result.statistics.stock_panels_used);
        println!();
    }
    
    // Сохранение результата
    if let Some(output_path) = output_path {
        save_result_to_file(&result, &output_path)?;
        println!("💾 Результат сохранен в: {}", output_path);
    } else {
        // Вывод результата в консоль
        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| CuttingError::GeneralCuttingError(format!("JSON serialization error: {}", e)))?;
        println!("📄 Результат:");
        println!("{}", json);
    }
    
    Ok(())
}

fn submit_task(input_path: String, threads: usize, verbose: bool) -> Result<(), CuttingError> {
    println!("📤 Отправка задачи на выполнение");
    println!("================================");
    
    let request = load_request_from_file(&input_path)?;
    
    if verbose {
        println!("📋 Входные данные:");
        println!("  - Панелей: {}", request.panels.len());
        println!("  - Стоковых панелей: {}", request.stock_panels.len());
        println!("  - Клиент: {}", request.client_info.id);
        println!();
    }
    
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = cutting_cli::engine::service::CutListOptimizerServiceImpl::new(logger);
    service.init(threads)?;
    
    let result = service.submit_task(request)?;
    
    if result.is_success() {
        println!("✅ Задача отправлена успешно");
        if let Some(task_id) = result.task_id {
            println!("🆔 ID задачи: {}", task_id);
            println!("💡 Используйте 'cutting-cli status --task-id {}' для проверки статуса", task_id);
        }
    } else {
        println!("❌ Ошибка отправки задачи");
        println!("   Код ошибки: {}", result.status_code);
    }
    
    Ok(())
}

fn check_status(task_id: String, verbose: bool) -> Result<(), CuttingError> {
    println!("📊 Проверка статуса задачи: {}", task_id);
    println!("=====================================");
    
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = cutting_cli::engine::service::CutListOptimizerServiceImpl::new(logger);
    service.init(1)?;
    
    let status_option = service.get_task_status(&task_id)?;
    
    if let Some(status) = status_option {
        println!("📋 Статус: {}", status.status);
        println!("📈 Прогресс инициализации: {}%", status.init_percentage);
        println!("📈 Прогресс выполнения: {}%", status.percentage_done);
        
        if verbose {
            if let Some(details) = status.details {
                println!("📝 Детали: {}", details);
            }
            
            if let Some(solution) = status.solution {
                println!("📊 Решение найдено:");
                println!("  - Размещено панелей: {}", solution.panels.len());
                println!("  - Не поместилось: {}", solution.no_fit_panels.len());
                println!("  - Эффективность: {:.1}%", solution.statistics.efficiency_percentage);
            }
        }
    } else {
        println!("❌ Задача с ID {} не найдена", task_id);
    }
    
    Ok(())
}

fn stop_task(task_id: String) -> Result<(), CuttingError> {
    println!("⏹️  Остановка задачи: {}", task_id);
    
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = cutting_cli::engine::service::CutListOptimizerServiceImpl::new(logger);
    service.init(1)?;
    
    service.stop_task(&task_id)?;
    
    println!("✅ Задача остановлена");
    Ok(())
}

fn terminate_task(task_id: String) -> Result<(), CuttingError> {
    println!("🛑 Принудительное завершение задачи: {}", task_id);
    
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = cutting_cli::engine::service::CutListOptimizerServiceImpl::new(logger);
    service.init(1)?;
    
    service.terminate_task(&task_id)?;
    
    println!("✅ Задача завершена принудительно");
    Ok(())
}

fn list_tasks(client_id: String, _status_filter: Option<String>) -> Result<(), CuttingError> {
    println!("📋 Список задач клиента: {}", client_id);
    println!("===============================");
    
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = cutting_cli::engine::service::CutListOptimizerServiceImpl::new(logger);
    service.init(1)?;
    
    // TODO: Реализовать парсинг статуса из строки
    let status = None; // status_filter.and_then(|s| parse_task_status(&s));
    
    let tasks = service.get_tasks(&client_id, status)?;
    
    if tasks.is_empty() {
        println!("📭 Задач не найдено");
    } else {
        println!("📊 Найдено задач: {}", tasks.len());
        println!();
        
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. ID: {}", i + 1, task.id);
            println!("   Статус: {:?}", task.status);
            if let Some(start_time) = task.start_time {
                println!("   Время запуска: {}", start_time.format("%Y-%m-%d %H:%M:%S"));
            }
            if let Some(end_time) = task.end_time {
                println!("   Время завершения: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
            }
            println!();
        }
    }
    
    Ok(())
}

fn show_stats() -> Result<(), CuttingError> {
    println!("📊 Статистика системы");
    println!("====================");
    
    let logger = Arc::new(CutListLoggerImpl::new());
    let mut service = cutting_cli::engine::service::CutListOptimizerServiceImpl::new(logger);
    service.init(1)?;
    
    let stats = service.get_stats()?;
    
    println!("🔄 Активные задачи: {}", stats.nbr_running_tasks);
    println!("⏸️  Задачи в ожидании: {}", stats.nbr_idle_tasks);
    println!("✅ Завершенные задачи: {}", stats.nbr_finished_tasks);
    println!("⏹️  Остановленные задачи: {}", stats.nbr_stopped_tasks);
    println!("🛑 Принудительно завершенные: {}", stats.nbr_terminated_tasks);
    println!("❌ Задачи с ошибками: {}", stats.nbr_error_tasks);
    println!();
    println!("🧵 Выполняющиеся потоки: {}", stats.nbr_running_threads);
    println!("📋 Потоки в очереди: {}", stats.nbr_queued_threads);
    println!("✅ Завершенные потоки: {}", stats.nbr_finished_threads);
    
    Ok(())
}

fn run_demo() {
    use cutting_cli::engine::model::tile::{TileNode, TileDimensions};
    use cutting_cli::engine::cutting::CuttingEngine;

    println!("🎯 Демонстрация базовых алгоритмов");
    println!("==================================");
    println!();

    // Демонстрация Этапа 1: Базовые структуры данных
    println!("📋 Этап 1: Базовые структуры данных");
    println!("-----------------------------------");
    
    // Создаем лист материала
    let mut sheet = TileNode::new(0, 1000, 0, 600);
    println!("Создан лист материала: {}x{} мм", sheet.get_width(), sheet.get_height());
    
    // Создаем список плиток для размещения
    let tiles = vec![
        TileDimensions::new(1, 300, 200, "Фанера".to_string(), 0, Some("Столешница".to_string())),
        TileDimensions::new(2, 150, 100, "Фанера".to_string(), 0, Some("Полка".to_string())),
        TileDimensions::new(3, 200, 250, "Фанера".to_string(), 0, Some("Дверца".to_string())),
        TileDimensions::new(4, 100, 80, "Фанера".to_string(), 0, Some("Ящик".to_string())),
        TileDimensions::new(5, 50, 50, "Фанера".to_string(), 0, Some("Квадрат".to_string())),
    ];
    
    println!("Плитки для размещения:");
    for (i, tile) in tiles.iter().enumerate() {
        println!("  - ID {}: {}x{} мм ({})", 
            i + 1, tile.width, tile.height, 
            tile.label.as_ref().unwrap_or(&"Без названия".to_string()));
    }
    println!();

    // Демонстрация Этапа 2: Алгоритмы разрезания
    println!("⚙️  Этап 2: Алгоритмы разрезания");
    println!("-------------------------------");
    
    let mut placed_count = 0;
    
    for (i, tile) in tiles.iter().enumerate() {
        println!("Размещаем плитку ID {}: {}x{} мм...", i + 1, tile.width, tile.height);
        
        // Создаем TileDimensions с правильным ID
        let tile_with_id = TileDimensions::new(
            i as i32 + 1,
            tile.width,
            tile.height,
            tile.material.clone(),
            tile.orientation,
            tile.label.clone(),
        );
        
        match CuttingEngine::try_place_tile(&mut sheet, &tile_with_id) {
            Ok(true) => {
                placed_count += 1;
                println!("  ✅ Успешно размещена{}", if sheet.get_final_tile_nodes().last().map_or(false, |n| n.is_rotated) { " (повернута)" } else { "" });
            }
            Ok(false) => {
                println!("  ❌ Не удалось разместить - не помещается");
            }
            Err(e) => {
                println!("  ❌ Ошибка: {:?}", e);
            }
        }
    }
    
    println!();
    println!("📊 Результаты размещения");
    println!("------------------------");
    println!("Размещено плиток: {}/{}", placed_count, tiles.len());
    println!("Использованная площадь: {} мм²", sheet.get_used_area());
    println!("Общая площадь листа: {} мм²", sheet.get_area());
    println!("Эффективность использования: {:.1}%", sheet.get_used_area_ratio() * 100.0);
    println!("Количество разрезов: {}", sheet.get_nbr_final_tiles());
    
    // Показываем финальные плитки
    println!();
    println!("🎯 Размещенные плитки:");
    let final_tiles = sheet.get_final_tile_nodes();
    for (i, final_tile) in final_tiles.iter().enumerate() {
        let original_tile = tiles.iter().find(|t| t.id == final_tile.external_id);
        if let Some(tile) = original_tile {
            println!("  {}. {} - позиция ({}, {}) размер {}x{}{}", 
                i + 1,
                tile.label.as_ref().unwrap_or(&format!("ID {}", tile.id)),
                final_tile.get_x1(), final_tile.get_y1(),
                final_tile.get_width(), final_tile.get_height(),
                if final_tile.is_rotated { " (повернута)" } else { "" }
            );
        }
    }
    
    // Демонстрация тестирования алгоритмов
    println!();
    println!("🧪 Тестирование алгоритмов");
    println!("--------------------------");
    
    // Тест горизонтального разреза
    let test_node = TileNode::new(0, 200, 0, 100);
    match CuttingEngine::split_horizontally(&test_node, 60, 3) {
        Ok(result) => {
            println!("✅ Горизонтальный разрез: {}x{} → {}x{} + {}x{}", 
                test_node.get_width(), test_node.get_height(),
                result.left_node.get_width(), result.left_node.get_height(),
                result.right_node.get_width(), result.right_node.get_height());
        }
        Err(e) => println!("❌ Ошибка горизонтального разреза: {:?}", e),
    }
    
    // Тест вертикального разреза
    match CuttingEngine::split_vertically(&test_node, 120, 3) {
        Ok(result) => {
            println!("✅ Вертикальный разрез: {}x{} → {}x{} + {}x{}", 
                test_node.get_width(), test_node.get_height(),
                result.left_node.get_width(), result.left_node.get_height(),
                result.right_node.get_width(), result.right_node.get_height());
        }
        Err(e) => println!("❌ Ошибка вертикального разреза: {:?}", e),
    }
    
    println!();
    println!("🎉 Демонстрация завершена!");
    println!("Этапы 1 и 2 успешно реализованы и протестированы.");
}

fn create_example(output_path: String) -> Result<(), CuttingError> {
    println!("📝 Создание примера входного файла");
    println!("==================================");
    
    let example_request = CalculationRequest {
        client_info: ClientInfo::with_details(
            "example_client".to_string(),
            Some("Example Client".to_string()),
            Some("1.0.0".to_string()),
            Some("CLI".to_string()),
        ),
        configuration: Configuration {
            cut_thickness: "3.0".to_string(),
            min_trim_dimension: "10.0".to_string(),
            optimization_factor: 1.0,
            use_single_stock_unit: false,
            cut_orientation_preference: 0,
            performance_thresholds: Some(PerformanceThresholds::default()),
        },
        panels: vec![
            Panel::new(1, "300.0".to_string(), "200.0".to_string(), 2, Some("Фанера".to_string())),
            Panel::new(2, "150.0".to_string(), "100.0".to_string(), 4, Some("Фанера".to_string())),
            Panel::new(3, "200.0".to_string(), "250.0".to_string(), 1, Some("Фанера".to_string())),
            Panel::new(4, "100.0".to_string(), "80.0".to_string(), 3, Some("Фанера".to_string())),
        ],
        stock_panels: vec![
            Panel::new(101, "1000.0".to_string(), "600.0".to_string(), 5, Some("Фанера".to_string())),
            Panel::new(102, "800.0".to_string(), "400.0".to_string(), 3, Some("Фанера".to_string())),
        ],
    };
    
    let json = serde_json::to_string_pretty(&example_request)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("JSON serialization error: {}", e)))?;
    
    fs::write(&output_path, json)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("File write error: {}", e)))?;
    
    println!("✅ Пример создан: {}", output_path);
    println!("💡 Используйте 'cutting-cli optimize --input {}' для запуска оптимизации", output_path);
    
    Ok(())
}

fn load_request_from_file(path: &str) -> Result<CalculationRequest, CuttingError> {
    if !Path::new(path).exists() {
        return Err(CuttingError::GeneralCuttingError(format!("File not found: {}", path)));
    }
    
    let content = fs::read_to_string(path)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("File read error: {}", e)))?;
    
    let request: CalculationRequest = serde_json::from_str(&content)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("JSON parse error: {}", e)))?;
    
    Ok(request)
}

fn save_result_to_file(result: &cutting_cli::engine::model::CalculationResponse, path: &str) -> Result<(), CuttingError> {
    let json = serde_json::to_string_pretty(result)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("JSON serialization error: {}", e)))?;
    
    fs::write(path, json)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("File write error: {}", e)))?;
    
    Ok(())
}

fn create_visualization(input_path: String, output_path: String) -> Result<(), CuttingError> {
    println!("🎨 Создание HTML визуализации");
    println!("=============================");
    
    // Загрузка результатов оптимизации
    if !Path::new(&input_path).exists() {
        return Err(CuttingError::GeneralCuttingError(format!("File not found: {}", input_path)));
    }
    
    let content = fs::read_to_string(&input_path)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("File read error: {}", e)))?;
    
    let response: CalculationResponse = serde_json::from_str(&content)
        .map_err(|e| CuttingError::GeneralCuttingError(format!("JSON parse error: {}", e)))?;
    
    println!("📊 Данные загружены:");
    println!("  - Размещено панелей: {}", response.statistics.placed_panels);
    println!("  - Не поместилось: {}", response.no_fit_panels.len());
    println!("  - Без материала: {}", response.no_material_panels.len());
    println!("  - Эффективность: {:.1}%", response.statistics.efficiency_percentage);
    println!();
    
    // Создание HTML визуализации
    HtmlVisualizer::generate_from_response(&response, &output_path)?;
    
    println!("✅ HTML визуализация создана: {}", output_path);
    println!("💡 Откройте файл в браузере для просмотра результатов");
    
    Ok(())
}

fn run_with_timeout<F, T>(f: F, timeout: Duration) -> Result<T, CuttingError>
where
    F: FnOnce() -> Result<T, CuttingError> + Send + 'static,
    T: Send + 'static,
{
    use std::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });
    
    match rx.recv_timeout(timeout) {
        Ok(result) => result,
        Err(_) => Err(CuttingError::Timeout(format!("Operation timed out after {} seconds", timeout.as_secs()))),
    }
}
