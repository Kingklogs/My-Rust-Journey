// examples/complex_protection_saga.rs
//! # The Complex Protection Saga
//! 
//! This example demonstrates advanced MEV protection scenarios including:
//! - Multiple concurrent threats
//! - Custom protection strategies  
//! - Real-time monitoring
//! - Batch protection of multiple transactions

use mev_protection_guardian::*;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();
    
    println!("🏛️  Advanced MEV Protection Guardian Demo");
    println!("     'A Tale of Multiple Transactions Under Siege'\n");
    
    // Create a batch of transactions with varying risk profiles
    let transaction_batch = create_diverse_transaction_batch();
    
    println!("📊 Created batch of {} transactions with varying risk profiles", transaction_batch.len());
    
    // Run protection sagas concurrently
    let protection_results = run_concurrent_protection_sagas(transaction_batch).await;
    
    // Analyze the results
    analyze_protection_outcomes(protection_results).await;
    
    Ok(())
}

/// Creates a diverse set of transactions to demonstrate different protection scenarios
fn create_diverse_transaction_batch() -> Vec<InnocentTransaction> {
    vec![
        // High-value Uniswap trade - attracts sandwich bots
        InnocentTransaction {
            id: Uuid::new_v4(),
            user_address: "0x742d35Cc6064C2532C4a2e3cE4285b8b4f267Db8".to_string(),
            target_contract: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(), // Uniswap V2
            value: 500000, // Very high value
            gas_price: 200, // Premium gas
            data: vec![0x38, 0xed, 0x17, 0x39, 0x00, 0x00, 0x00, 0x64], // swapExactTokensForTokens
            vulnerability_score: 0.0,
        },
        
        // Medium-value SushiSwap trade
        InnocentTransaction {
            id: Uuid::new_v4(),
            user_address: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_string(),
            target_contract: "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F".to_string(), // SushiSwap
            value: 50000,
            gas_price: 80,
            data: vec![0x38, 0xed, 0x17, 0x39, 0x00, 0x00, 0x01, 0x00],
            vulnerability_score: 0.0,
        },
        
        // Low-value transaction to unknown contract
        InnocentTransaction {
            id: Uuid::new_v4(),
            user_address: "0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string(),
            target_contract: "0x1234567890123456789012345678901234567890".to_string(), // Unknown contract
            value: 1000,
            gas_price: 20,
            data: vec![0xa9, 0x05, 0x9c, 0xbb], // transfer function
            vulnerability_score: 0.0,
        },
        
        // NFT marketplace transaction - different risk profile
        InnocentTransaction {
            id: Uuid::new_v4(),
            user_address: "0xA0b86a33E6411011495ec33fCb61F4A4c2b0F4e4".to_string(),
            target_contract: "0x7Be8076f4EA4A4AD08075C2508e481d6C946D12b".to_string(), // OpenSea
            value: 75000,
            gas_price: 150,
            data: vec![0xfb, 0x0f, 0x3e, 0xe1], // atomicMatch_
            vulnerability_score: 0.0,
        },
        
        // DeFi lending transaction
        InnocentTransaction {
            id: Uuid::new_v4(),
            user_address: "0x514910771AF9Ca656af840dff83E8264EcF986CA".to_string(),
            target_contract: "0x7d2768dE32b0b80b7a3454c06BdAc94A69DDc7A9".to_string(), // Aave
            value: 200000,
            gas_price: 100,
            data: vec![0xe8, 0xed, 0xa9, 0xdf], // deposit
            vulnerability_score: 0.0,
        },
    ]
}

/// Runs protection sagas for multiple transactions concurrently
async fn run_concurrent_protection_sagas(
    transactions: Vec<InnocentTransaction>
) -> Vec<(Uuid, Result<SafeSanctuary, supporting_cast::PlotTwist>)> {
    println!("🚀 Launching concurrent protection sagas...\n");
    
    let mut handles = Vec::new();
    
    for transaction in transactions {
        let tx_id = transaction.id;
        let handle = tokio::spawn(async move {
            println!("🎭 Starting protection saga for transaction {}", tx_id);
            
            // Add some realistic delays to simulate blockchain conditions
            sleep(Duration::from_millis(rand::random::<u64>() % 100)).await;
            
            let result = complete_mev_protection_saga(transaction).await;
            
            match &result {
                Ok(sanctuary) => {
                    println!("✅ Transaction {} reached {:?} sanctuary", tx_id, sanctuary.security_level);
                },
                Err(plot_twist) => {
                    println!("⚡ Transaction {} encountered plot twist: {}", tx_id, plot_twist);
                }
            }
            
            (tx_id, result)
        });
        
        handles.push(handle);
    }
    
    // Wait for all sagas to complete
    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => println!("❌ Task failed: {}", e),
        }
    }
    
    results
}

/// Analyzes the outcomes of multiple protection sagas
async fn analyze_protection_outcomes(
    results: Vec<(Uuid, Result<SafeSanctuary, supporting_cast::PlotTwist>)>
) {
    println!("\n📊 PROTECTION SAGA ANALYSIS");
    println!("═══════════════════════════════════════\n");
    
    let mut security_distribution = HashMap::new();
    let mut success_count = 0;
    let mut plot_twist_count = 0;
    let mut total_guardians = 0;
    
    for (tx_id, result) in results {
        match result {
            Ok(sanctuary) => {
                success_count += 1;
                total_guardians += sanctuary.guardian_count;
                
                let count = security_distribution.entry(sanctuary.security_level.clone()).or_insert(0);
                *count += 1;
                
                println!("🏛️  Transaction {}: SUCCESS", tx_id);
                println!("    Security Level: {:?}", sanctuary.security_level);
                println!("    Guardian Count: {}", sanctuary.guardian_count);
                println!();
            },
            Err(plot_twist) => {
                plot_twist_count += 1;
                
                println!("⚡ Transaction {}: PLOT TWIST", tx_id);
                println!("    Challenge: {}", plot_twist);
                
                let resolution = plot_twist.resolve_the_conflict();
                println!("    Resolution: {:?}", resolution);
                println!();
            }
        }
    }
    
    // Print summary statistics
    println!("📈 SUMMARY STATISTICS");
    println!("────────────────────────────────────────");
    println!("Total Transactions: {}", success_count + plot_twist_count);
    println!("Successful Protections: {} ({:.1}%)", 
             success_count, 
             (success_count as f64 / (success_count + plot_twist_count) as f64) * 100.0);
    println!("Plot Twists Encountered: {}", plot_twist_count);
    println!("Average Guardians per Transaction: {:.1}", 
             total_guardians as f64 / success_count.max(1) as f64);
    println!();
    
    println!("🛡️  SECURITY LEVEL DISTRIBUTION");
    println!("────────────────────────────────────────");
    for (level, count) in security_distribution {
        println!("{:?}: {} transactions", level, count);
    }
    
    // Simulate ongoing monitoring
    println!("\n🔍 ONGOING PROTECTION MONITORING");
    println!("────────────────────────────────────────");
    simulate_ongoing_monitoring().await;
}

/// Simulates ongoing monitoring of the protection system
async fn simulate_ongoing_monitoring() {
    println!("Starting continuous threat monitoring...");
    
    for i in 1..=5 {
        sleep(Duration::from_secs(1)).await;
        
        let mempool_activity = rand::random::<f64>();
        let threat_level = if mempool_activity > 0.8 {
            "🔴 HIGH"
        } else if mempool_activity > 0.5 {
            "🟡 MEDIUM"  
        } else {
            "🟢 LOW"
        };
        
        let active_guardians = rand::random::<usize>() % 10 + 1;
        let protected_transactions = rand::random::<usize>() % 50 + 1;
        
        println!("Monitor Cycle {}: Threat Level: {} | Active Guardians: {} | Protected Txs: {}", 
                 i, threat_level, active_guardians, protected_transactions);
    }
    
    println!("✅ Monitoring complete - All systems operational");
}

// Additional example functions for specific scenarios

/// Demonstrates custom protection strategy selection
pub async fn custom_protection_strategy_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Custom Protection Strategy Example");
    
    let high_risk_transaction = InnocentTransaction {
        id: Uuid::new_v4(),
        user_address: "0x742d35Cc6064C2532C4a2e3cE4285b8b4f267Db8".to_string(),
        target_contract: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
        value: 1000000, // Very high value
        gas_price: 300, // Maximum gas
        data: vec![0x38, 0xed, 0x17, 0x39], // Swap signature
        vulnerability_score: 0.95, // Manually set high vulnerability
    };
    
    println!("💎 Processing ultra-high-risk transaction...");
    
    match complete_mev_protection_saga(high_risk_transaction).await {
        Ok(sanctuary) => {
            println!("🏆 Ultra-high-risk transaction successfully protected!");
            println!("   Final Security Level: {:?}", sanctuary.security_level);
            
            // Should reach Sacred Sanctuary level
            assert!(matches!(sanctuary.security_level, SecurityLevel::SacredSanctuary));
        },
        Err(e) => {
            println!("❌ Failed to protect ultra-high-risk transaction: {}", e);
        }
    }
    
    Ok(())
}

/// Demonstrates batch processing with different risk profiles
pub async fn risk_profile_analysis_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Risk Profile Analysis Example");
    
    // Create transactions with known risk profiles
    let risk_scenarios = vec![
        ("Low Risk", create_low_risk_transaction()),
        ("Medium Risk", create_medium_risk_transaction()),
        ("High Risk", create_high_risk_transaction()),
        ("Ultra High Risk", create_ultra_high_risk_transaction()),
    ];
    
    for (risk_name, transaction) in risk_scenarios {
        println!("\n🎯 Testing {} scenario...", risk_name);
        
        match complete_mev_protection_saga(transaction).await {
            Ok(sanctuary) => {
                println!("   ✅ Protection Level: {:?}", sanctuary.security_level);
                println!("   🛡️  Guardian Count: {}", sanctuary.guardian_count);
            },
            Err(plot_twist) => {
                println!("   ⚡ Plot Twist: {}", plot_twist);
            }
        }
    }
    
    Ok(())
}

// Helper functions for creating test transactions

fn create_low_risk_transaction() -> InnocentTransaction {
    InnocentTransaction {
        id: Uuid::new_v4(),
        user_address: "0x1234567890123456789012345678901234567890".to_string(),
        target_contract: "0x0000000000000000000000000000000000000001".to_string(),
        value: 100,
        gas_price: 20,
        data: vec![0xa9, 0x05, 0x9c, 0xbb], // Simple transfer
        vulnerability_score: 0.0,
    }
}

fn create_medium_risk_transaction() -> InnocentTransaction {
    InnocentTransaction {
        id: Uuid::new_v4(),
        user_address: "0x742d35Cc6064C2532C4a2e3cE4285b8b4f267Db8".to_string(),
        target_contract: "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F".to_string(), // SushiSwap
        value: 25000,
        gas_price: 60,
        data: vec![0x38, 0xed, 0x17, 0x39],
        vulnerability_score: 0.0,
    }
}

fn create_high_risk_transaction() -> InnocentTransaction {
    InnocentTransaction {
        id: Uuid::new_v4(),
        user_address: "0x742d35Cc6064C2532C4a2e3cE4285b8b4f267Db8".to_string(),
        target_contract: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(), // Uniswap V2
        value: 200000,
        gas_price: 150,
        data: vec![0x38, 0xed, 0x17, 0x39],
        vulnerability_score: 0.0,
    }
}

fn create_ultra_high_risk_transaction() -> InnocentTransaction {
    InnocentTransaction {
        id: Uuid::new_v4(),
        user_address: "0x742d35Cc6064C2532C4a2e3cE4285b8b4f267Db8".to_string(),
        target_contract: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(), // Uniswap V2
        value: 1000000, // 1M value
        gas_price: 400, // Extreme gas price
        data: vec![0x38, 0xed, 0x17, 0x39, 0xFF, 0xFF, 0xFF, 0xFF], // Large swap
        vulnerability_score: 0.0,
    }
}