use std::env;
use std::error::Error;
use crate::commands::run::seeder_tracker::SeederTracker;
use crate::utils;

pub async fn list_seeders_status() -> Result<(), Box<dyn Error>> {
    let seeders = utils::list_seeders().await?;
    
    if seeders.is_empty() {
        println!("No seeders available in the seeders directory.");
        return Ok(());
    }

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "Please, be sure to set the `DATABASE_URL` environment variable.")?;
    
    let tracker = SeederTracker::new(database_url)?;
    tracker.ensure_seeds_table().await?;

    println!("\n{:<30} {:<10}", "Seeder Name", "Status");
    println!("{}", "-".repeat(42));

    for seeder in seeders {
        let is_executed = tracker.is_seeder_executed(&seeder).await?;
        let status = if is_executed {
            "✅ Executed"
        } else {
            "⏳ Pending"
        };
        
        println!("{:<30} {:<10}", seeder, status);
    }
    
    println!();
    Ok(())
}