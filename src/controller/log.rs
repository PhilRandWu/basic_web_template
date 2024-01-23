
pub fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"); // 配置日志等级为 INFO 级别

    env_logger::Builder::from_env(env) // 创建日志记录器构建器
        .format(|buf, record| { // 配置日志格式
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // 当前时间
                buf.default_styled_level(record.level()), // 日志等级
                record.module_path().unwrap_or("<unnamed>"), // 记录器名称
                &record.args() // 日志正文
            )
        })
        .init();
    // 启动日志记录器
    info!("env_logger initialized."); // 输出一条日志以表明已成功初始化日志记录器
}