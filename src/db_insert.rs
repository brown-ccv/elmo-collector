use crate::{CpuUtilization, GpuUtilization, NodeType, NodeUtilization};
use config::{Config, File, FileFormat};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

pub async fn get_db_pool() -> PgPool {
    let builder =
        Config::builder().add_source(File::new("secrets/db_config.yml", FileFormat::Yaml));

    let db_config = match builder.build() {
        Ok(config) => config.try_deserialize::<DbConfig>().unwrap(),
        Err(e) => {
            panic!("Failed to read config file: {}", e);
        }
    };

    let pool = PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.dbname
    ))
    .await
    .unwrap();

    pool
}

pub async fn insert_cpu_info(pool: &PgPool, cpu_info: &CpuUtilization) {
    sqlx::query(
        "INSERT INTO cpu (allocated, n_idle, n_other, total) 
         VALUES ($1, $2, $3, $4)",
    )
    .bind(cpu_info.n_alloc)
    .bind(cpu_info.n_idle)
    .bind(cpu_info.n_other)
    .bind(cpu_info.n_total)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_gpu_info(pool: &PgPool, gpu_info: &GpuUtilization) {
    sqlx::query(
        "INSERT INTO gpu (allocated, total) 
         VALUES ($1, $2)",
    )
    .bind(gpu_info.n_alloc)
    .bind(gpu_info.n_total)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_powersave_node_info(
    pool: &PgPool,
    node_type: &NodeType,
    node_info: &NodeUtilization,
) {
    let table_name = match node_type {
        NodeType::Cpu => "power_save_cpu",
        NodeType::Gpu => "power_save_gpu",
        NodeType::All => "power_save",
    };

    let query = format!(
        "INSERT INTO {} (power_save, total) VALUES ($1, $2)",
        table_name
    );

    sqlx::query(&query)
        .bind(node_info.n_alloc)
        .bind(node_info.n_total)
        .execute(pool)
        .await
        .unwrap();
}
