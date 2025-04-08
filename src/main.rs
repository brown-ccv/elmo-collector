use elmo_collector::{NodeType, db_insert, query_slurm};

#[tokio::main]
async fn main() {
    let pool = db_insert::get_db_pool().await;

    let cpu_info = query_slurm::get_cpu_info();
    let gpu_info = query_slurm::get_gpu_info();

    let powersave_cpu_info = query_slurm::get_powersave_node_info(NodeType::Cpu);
    let powersave_gpu_info = query_slurm::get_powersave_node_info(NodeType::Gpu);
    let powersave_all_info = query_slurm::get_powersave_node_info(NodeType::All);

    db_insert::insert_cpu_info(&pool, &cpu_info).await;
    db_insert::insert_gpu_info(&pool, &gpu_info).await;
    db_insert::insert_powersave_node_info(&pool, &NodeType::Cpu, &powersave_cpu_info).await;
    db_insert::insert_powersave_node_info(&pool, &NodeType::Gpu, &powersave_gpu_info).await;
    db_insert::insert_powersave_node_info(&pool, &NodeType::All, &powersave_all_info).await;

    println!("CPU Info: {:?}", cpu_info);
    println!("GPU Info: {:?}", gpu_info);
}
