pub mod db_insert;
pub mod query_slurm;

use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CpuUtilization {
    pub n_alloc: i32,
    pub n_idle: i32,
    pub n_other: i32,
    pub n_total: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct GpuUtilization {
    pub n_alloc: i32,
    pub n_total: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct NodeUtilization {
    pub n_alloc: i32,
    pub n_total: i32,
}

pub enum NodeType {
    All,
    Cpu,
    Gpu,
}
