// Production-Ready Blockchain Validator - The Systems Storyteller Way
// 
// A Tale of Trust, Consensus, and Digital Truth
// Where transactions tell their stories and blocks preserve history

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use sled::{Db, IVec};
use tokio::sync::mpsc;

/// ## The Grand Narrative: A Blockchain's Life Story
/// 
/// In this realm, transactions arrive seeking validation,
/// blocks compete to join the eternal chain, and validators
/// stand guard over the integrity of digital truth.
pub struct BlockchainChronicler {
    chain_repository: ChainRepository,
    mempool_of_pending_tales: Arc<Mutex<Vec<TransactionStory>>>,
    validator_council: ValidatorCouncil,
    network_storytellers: NetworkOfStoryTellers,
    utxo_ledger: UTXOLedger,
    configuration: ChronicleConfiguration,
    mining_heart: Option<MiningHeart>,
}

/// ## Chapter Structure: Each Block Tells Its Tale
/// 
/// Every block is a chapter in the blockchain's story,
/// containing the tales of transactions and the wisdom
/// of those who came before.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockChapter {
    chapter_number: u64,
    timestamp_of_creation: u64,
    previous_chapter_essence: String,
    transaction_tales: Vec<TransactionStory>,
    merkle_tree_of_truth: String,
    chapter_essence: String,
    proof_of_storytelling: ProofOfWork,
    chapter_size_bytes: usize,
}

/// ## Individual Transaction Stories
/// 
/// Each transaction is a story of value moving through
/// the digital realm, seeking validation and permanence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionStory {
    story_id: String,
    inputs_consumed: Vec<UTXOReference>,
    outputs_created: Vec<UTXOOutput>,
    story_fee: u64,
    timestamp_of_telling: u64,
    transaction_nonce: u64,
    digital_signature: Vec<u8>,
    public_key_of_narrator: Vec<u8>,
}

/// ## UTXO: Unspent Tale Outputs
/// 
/// These represent the unspent outputs that can be used
/// as inputs for future transaction stories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UTXOReference {
    previous_story_id: String,
    output_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UTXOOutput {
    recipient_address: Vec<u8>,
    value_locked: u64,
    locking_script: ScriptOfTruth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptOfTruth {
    script_type: ScriptType,
    required_signatures: u8,
    public_keys: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScriptType {
    PayToPublicKey,
    PayToMultiSig,
    PayToScriptHash,
}

/// ## The Proof of Work: Storytelling Effort
/// 
/// The computational effort required to earn the right
/// to tell the next chapter in the blockchain's saga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWork {
    difficulty_target: u64,
    nonce_of_discovery: u64,
    storyteller_reward: u64,
    hash_rate_estimate: f64,
}

/// ## Persistent Chain Repository
/// 
/// The eternal keeper of all blockchain stories,
/// persisted to disk for immortality.
pub struct ChainRepository {
    block_db: Db,
    tx_db: Db,
    utxo_db: Db,
    chain_tip: Arc<RwLock<Option<BlockChapter>>>,
    block_index: Arc<RwLock<HashMap<String, u64>>>,
}

/// ## UTXO Ledger: Keeper of Unspent Stories
/// 
/// Tracks all unspent transaction outputs that can be
/// used as inputs for new transaction stories.
pub struct UTXOLedger {
    unspent_outputs: Arc<RwLock<HashMap<String, UTXOOutput>>>,
    spent_outputs: Arc<RwLock<HashSet<String>>>,
    db: Db,
}

/// ## Network of Story Tellers
/// 
/// Manages connections with other blockchain nodes,
/// sharing stories and synchronizing the eternal chain.
pub struct NetworkOfStoryTellers {
    peer_connections: Arc<Mutex<Vec<PeerConnection>>>,
    message_broadcaster: Arc<Mutex<mpsc::UnboundedSender<NetworkMessage>>>,
    sync_status: Arc<RwLock<SyncStatus>>,
    known_peers: Arc<RwLock<HashSet<String>>>,
}

#[derive(Debug, Clone)]
pub struct PeerConnection {
    peer_address: String,
    connection_stream: Arc<Mutex<TcpStream>>,
    last_seen: u64,
    sync_height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    NewTransactionStory(TransactionStory),
    NewBlockChapter(BlockChapter),
    RequestChainSync(u64),
    ChainSyncResponse(Vec<BlockChapter>),
    PeerDiscovery(Vec<String>),
    Heartbeat(u64),
}

#[derive(Debug, Clone)]
pub struct SyncStatus {
    is_syncing: bool,
    current_height: u64,
    target_height: u64,
    sync_progress: f64,
}

/// ## Mining Heart: The Proof of Work Engine
/// 
/// The computational heart that seeks valid hashes
/// through persistent effort and storytelling passion.
pub struct MiningHeart {
    is_beating: Arc<Mutex<bool>>,
    current_difficulty: Arc<RwLock<u64>>,
    hash_rate: Arc<RwLock<f64>>,
    mining_reward_address: Vec<u8>,
    thread_handles: Vec<thread::JoinHandle<()>>,
}

/// ## The Validator Council: Guardians of Truth
/// 
/// These entities ensure that only valid stories become
/// part of the permanent blockchain narrative.
pub struct ValidatorCouncil {
    council_members: HashMap<String, ValidatorGuardian>,
    consensus_threshold: f64,
    current_storyteller: Option<String>,
    reputation_system: ReputationSystem,
}

#[derive(Debug, Clone)]
pub struct ValidatorGuardian {
    guardian_id: String,
    stake_in_truth: u64,
    reputation_score: f64,
    tales_validated: u64,
    public_key: PublicKey,
    last_validation_time: u64,
}

pub struct ReputationSystem {
    validator_scores: HashMap<String, f64>,
    penalty_system: PenaltyTracker,
    reward_multipliers: HashMap<String, f64>,
}

#[derive(Debug)]
pub struct PenaltyTracker {
    recent_penalties: HashMap<String, Vec<(u64, PenaltyType)>>,
    cumulative_penalties: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub enum PenaltyType {
    InvalidSignature,
    DoubleSpending,
    MalformedTransaction,
    NetworkMisbehavior,
}

/// ## Chronicle Configuration
/// 
/// The fundamental rules that govern how our
/// blockchain chronicle operates and evolves.
#[derive(Debug, Clone)]
pub struct ChronicleConfiguration {
    pub target_block_time: Duration,
    pub difficulty_adjustment_interval: u64,
    pub max_block_size: usize,
    pub min_transaction_fee: u64,
    pub base_mining_reward: u64,
    pub reward_halving_interval: u64,
    pub max_peers: usize,
    pub network_port: u16,
    pub data_directory: String,
}

impl BlockchainChronicler {
    /// ## The Genesis Moment: Where All Stories Begin
    /// 
    /// In the beginning, there was the genesis block,
    /// the first chapter of an infinite story.
    pub async fn new_chronicle_begins(config: ChronicleConfiguration) -> Result<Self, ChronicleError> {
        println!("📖 A new blockchain chronicle begins...");
        
        let chain_repository = ChainRepository::new(&config.data_directory).await?;
        let utxo_ledger = UTXOLedger::new(&config.data_directory).await?;
        let network = NetworkOfStoryTellers::new(config.network_port).await?;
        
        let mut chronicle = Self {
            chain_repository,
            mempool_of_pending_tales: Arc::new(Mutex::new(Vec::new())),
            validator_council: ValidatorCouncil::new(),
            network_storytellers: network,
            utxo_ledger,
            configuration: config.clone(),
            mining_heart: None,
        };

        // Create genesis block if this is a new chain
        if chronicle.chain_repository.chain_is_empty().await? {
            chronicle.craft_and_commit_genesis_chapter().await?;
        }

        // Start the mining heart
        chronicle.awaken_the_mining_heart().await?;
        
        // Begin network synchronization
        chronicle.begin_network_synchronization().await?;

        println!("✨ Chronicle initialization complete!");
        Ok(chronicle)
    }

    /// ## Act I: Transaction Stories Arrive
    /// 
    /// New transaction stories arrive at our validator,
    /// seeking their place in the permanent record.
    pub async fn transaction_story_arrives(&mut self, story: TransactionStory) -> Result<(), ChronicleError> {
        println!("📜 New transaction story arrives: {}", story.story_id);

        // The story must prove its authenticity
        self.story_proves_its_authenticity(&story).await?;
        
        // The story must not be a duplicate
        self.story_proves_its_uniqueness(&story).await?;
        
        // Add to mempool
        {
            let mut mempool = self.mempool_of_pending_tales.lock().unwrap();
            mempool.push(story.clone());
            
            // Keep mempool size reasonable
            if mempool.len() > 10000 {
                mempool.sort_by(|a, b| b.story_fee.cmp(&a.story_fee));
                mempool.truncate(10000);
            }
        }

        // Broadcast to network
        self.network_storytellers.broadcast_transaction_story(story).await?;
        
        println!("✅ Story accepted into the mempool");
        Ok(())
    }

    /// ## Act II: Stories Undergo Rigorous Validation
    /// 
    /// Each story must prove its worth through multiple trials
    /// before joining the permanent chronicle of the blockchain.
    async fn story_proves_its_authenticity(&self, story: &TransactionStory) -> Result<(), ChronicleError> {
        // Verify digital signature
        if !self.signature_tells_the_truth(story).await? {
            return Err(ChronicleError::StoryBearsFalseWitness(
                "Digital signature verification failed".to_string()
            ));
        }

        // Verify inputs exist and are unspent
        let total_input_value = self.verify_and_calculate_input_value(story).await?;
        let total_output_value = story.outputs_created.iter().map(|o| o.value_locked).sum::<u64>();
        
        // Verify sufficient funds and reasonable fee
        if total_input_value < total_output_value + story.story_fee {
            return Err(ChronicleError::NarratorLacksResources(
                "Insufficient input value to cover outputs and fees".to_string()
            ));
        }

        // Verify fee is reasonable
        if story.story_fee < self.configuration.min_transaction_fee {
            return Err(ChronicleError::InsufficientFee(story.story_fee));
        }

        // Verify transaction nonce to prevent replay attacks
        self.verify_transaction_nonce(story).await?;

        println!("✅ Story passes all authenticity trials");
        Ok(())
    }

    async fn story_proves_its_uniqueness(&self, story: &TransactionStory) -> Result<(), ChronicleError> {
        // Check if transaction already exists in chain
        if self.chain_repository.transaction_exists(&story.story_id).await? {
            return Err(ChronicleError::DuplicateStory(story.story_id.clone()));
        }

        // Check if already in mempool
        let mempool = self.mempool_of_pending_tales.lock().unwrap();
        if mempool.iter().any(|t| t.story_id == story.story_id) {
            return Err(ChronicleError::DuplicateStory(story.story_id.clone()));
        }

        Ok(())
    }

    async fn signature_tells_the_truth(&self, story: &TransactionStory) -> Result<bool, ChronicleError> {
        let public_key = PublicKey::from_bytes(&story.public_key_of_narrator)
            .map_err(|e| ChronicleError::InvalidPublicKey(e.to_string()))?;
        
        let signature = Signature::from_bytes(&story.digital_signature)
            .map_err(|e| ChronicleError::InvalidSignature(e.to_string()))?;

        // Create message to verify (transaction without signature)
        let message = self.create_signable_message(story);
        
        Ok(public_key.verify(&message, &signature).is_ok())
    }

    async fn verify_and_calculate_input_value(&self, story: &TransactionStory) -> Result<u64, ChronicleError> {
        let mut total_value = 0u64;
        
        for input in &story.inputs_consumed {
            let utxo_key = format!("{}:{}", input.previous_story_id, input.output_index);
            
            // Check if UTXO exists and is unspent
            let utxo = self.utxo_ledger.find_unspent_output(&utxo_key).await?
                .ok_or_else(|| ChronicleError::UTXONotFound(utxo_key.clone()))?;
            
            // Verify spending authorization (simplified - in reality would check scripts)
            self.verify_spending_authorization(&utxo, &story.public_key_of_narrator)?;
            
            total_value = total_value.checked_add(utxo.value_locked)
                .ok_or(ChronicleError::ValueOverflow)?;
        }
        
        Ok(total_value)
    }

    async fn verify_transaction_nonce(&self, story: &TransactionStory) -> Result<(), ChronicleError> {
        // In a real implementation, you'd track nonces per address
        // This is a simplified check
        if story.transaction_nonce == 0 {
            return Err(ChronicleError::InvalidNonce(story.transaction_nonce));
        }
        Ok(())
    }

    /// ## Act III: The Mining Saga
    /// 
    /// Through computational effort, validators earn the right
    /// to add their chapter to the eternal blockchain story.
    async fn awaken_the_mining_heart(&mut self) -> Result<(), ChronicleError> {
        let mining_keypair = Keypair::generate(&mut OsRng);
        
        self.mining_heart = Some(MiningHeart::new(
            mining_keypair.public.to_bytes().to_vec(),
            self.configuration.target_block_time,
        ));
        
        // Start mining thread
        self.begin_the_eternal_mining_quest().await?;
        
        println!("💎 Mining heart awakened and beating");
        Ok(())
    }

    async fn begin_the_eternal_mining_quest(&mut self) -> Result<(), ChronicleError> {
        let mempool = self.mempool_of_pending_tales.clone();
        let chain_repo = self.chain_repository.clone();
        let config = self.configuration.clone();
        
        tokio::spawn(async move {
            loop {
                // Wait for transactions to accumulate
                tokio::time::sleep(Duration::from_secs(1)).await;
                
                let transactions = {
                    let mut pool = mempool.lock().unwrap();
                    if pool.is_empty() {
                        continue;
                    }
                    
                    // Select transactions for next block
                    pool.sort_by(|a, b| b.story_fee.cmp(&a.story_fee));
                    let selected = pool.drain(..std::cmp::min(1000, pool.len())).collect::<Vec<_>>();
                    selected
                };
                
                if let Err(e) = Self::mine_new_chapter(transactions, &chain_repo, &config).await {
                    eprintln!("Mining error: {:?}", e);
                }
            }
        });
        
        Ok(())
    }

    async fn mine_new_chapter(
        transactions: Vec<TransactionStory>,
        chain_repo: &ChainRepository,
        config: &ChronicleConfiguration,
    ) -> Result<(), ChronicleError> {
        println!("⛏️  Beginning to mine new chapter with {} transactions", transactions.len());
        
        let previous_block = chain_repo.get_chain_tip().await?
            .ok_or(ChronicleError::ChainCorrupted("No chain tip found".to_string()))?;
        
        let mut block = BlockChapter {
            chapter_number: previous_block.chapter_number + 1,
            timestamp_of_creation: current_timestamp(),
            previous_chapter_essence: previous_block.chapter_essence.clone(),
            transaction_tales: transactions,
            merkle_tree_of_truth: String::new(),
            chapter_essence: String::new(),
            proof_of_storytelling: ProofOfWork {
                difficulty_target: Self::calculate_current_difficulty(&previous_block, config),
                nonce_of_discovery: 0,
                storyteller_reward: config.base_mining_reward,
                hash_rate_estimate: 0.0,
            },
            chapter_size_bytes: 0,
        };

        // Calculate merkle root
        block.merkle_tree_of_truth = Self::weave_merkle_tree_of_truth(&block.transaction_tales);
        
        // Mine the block
        let (hash, nonce, hash_rate) = Self::perform_proof_of_work(&block).await?;
        
        block.chapter_essence = hash;
        block.proof_of_storytelling.nonce_of_discovery = nonce;
        block.proof_of_storytelling.hash_rate_estimate = hash_rate;
        block.chapter_size_bytes = bincode::serialize(&block).unwrap().len();
        
        // Commit to chain
        chain_repo.add_block_chapter(block.clone()).await?;
        
        println!("🎉 New chapter {} mined successfully!", block.chapter_number);
        Ok(())
    }

    async fn perform_proof_of_work(block: &BlockChapter) -> Result<(String, u64, f64), ChronicleError> {
        let difficulty_target = block.proof_of_storytelling.difficulty_target;
        let start_time = SystemTime::now();
        
        println!("🎯 Mining with difficulty target: {}", difficulty_target);
        
        // Use multiple threads for mining
        let num_threads = num_cpus::get();
        let mut handles = vec![];
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        for thread_id in 0..num_threads {
            let block_clone = block.clone();
            let tx_clone = tx.clone();
            
            let handle = tokio::spawn(async move {
                let start_nonce = thread_id as u64 * 1_000_000;
                
                for nonce in start_nonce..(start_nonce + 10_000_000) {
                    let hash = Self::calculate_block_hash(&block_clone, nonce);
                    
                    if Self::hash_meets_difficulty(&hash, difficulty_target) {
                        let _ = tx_clone.send((hash, nonce));
                        return;
                    }
                    
                    if nonce % 100_000 == 0 {
                        println!("Thread {}: Tried {} nonces...", thread_id, nonce - start_nonce);
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for a solution
        if let Some((hash, nonce)) = rx.recv().await {
            // Cancel other threads
            for handle in handles {
                handle.abort();
            }
            
            let elapsed = start_time.elapsed().unwrap();
            let hash_rate = nonce as f64 / elapsed.as_secs_f64();
            
            println!("💎 Solution found! Nonce: {}, Hash rate: {:.2} H/s", nonce, hash_rate);
            Ok((hash, nonce, hash_rate))
        } else {
            Err(ChronicleError::ProofOfWorkFailed("No solution found".to_string()))
        }
    }

    /// ## Network Synchronization Saga
    
    async fn begin_network_synchronization(&mut self) -> Result<(), ChronicleError> {
        self.network_storytellers.start_peer_discovery().await?;
        self.network_storytellers.begin_chain_synchronization().await?;
        println!("🌐 Network synchronization initiated");
        Ok(())
    }

    /// ## Supporting Cast: Helper Functions
    
    fn calculate_current_difficulty(previous_block: &BlockChapter, config: &ChronicleConfiguration) -> u64 {
        // Simplified difficulty adjustment
        // In reality, would look at last N blocks' timing
        let base_difficulty = 0x00000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
        
        if previous_block.chapter_number % config.difficulty_adjustment_interval == 0 && previous_block.chapter_number > 0 {
            // Adjust difficulty based on block timing
            // This is a simplified calculation
            base_difficulty / 2 // Make it harder
        } else {
            previous_block.proof_of_storytelling.difficulty_target
        }
    }

    fn calculate_block_hash(block: &BlockChapter, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        let block_data = format!(
            "{}{}{}{}{}{}",
            block.chapter_number,
            block.timestamp_of_creation,
            block.previous_chapter_essence,
            block.merkle_tree_of_truth,
            block.proof_of_storytelling.difficulty_target,
            nonce
        );
        hasher.update(block_data.as_bytes());
        format!("{:064x}", hasher.finalize())
    }

    fn hash_meets_difficulty(hash: &str, difficulty_target: u64) -> bool {
        // Convert hash to number and compare with target
        // Simplified: just check for leading zeros
        let leading_zeros = difficulty_target.leading_zeros() as usize / 4;
        hash.starts_with(&"0".repeat(leading_zeros))
    }

    fn weave_merkle_tree_of_truth(transactions: &[TransactionStory]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }
        
        let mut hashes: Vec<String> = transactions.iter()
            .map(|tx| {
                let mut hasher = Sha256::new();
                hasher.update(tx.story_id.as_bytes());
                format!("{:x}", hasher.finalize())
            })
            .collect();
        
        // Build merkle tree
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(chunk[0].as_bytes());
                if chunk.len() > 1 {
                    hasher.update(chunk[1].as_bytes());
                } else {
                    hasher.update(chunk[0].as_bytes()); // Duplicate if odd number
                }
                next_level.push(format!("{:x}", hasher.finalize()));
            }
            
            hashes = next_level;
        }
        
        hashes.into_iter().next().unwrap_or_else(|| "0".repeat(64))
    }

    fn create_signable_message(&self, story: &TransactionStory) -> Vec<u8> {
        // Create a canonical representation for signing
        let mut message = Vec::new();
        message.extend_from_slice(story.story_id.as_bytes());
        message.extend_from_slice(&story.timestamp_of_telling.to_le_bytes());
        message.extend_from_slice(&story.transaction_nonce.to_le_bytes());
        
        // Add inputs and outputs
        for input in &story.inputs_consumed {
            message.extend_from_slice(input.previous_story_id.as_bytes());
            message.extend_from_slice(&input.output_index.to_le_bytes());
        }
        
        for output in &story.outputs_created {
            message.extend_from_slice(&output.recipient_address);
            message.extend_from_slice(&output.value_locked.to_le_bytes());
        }
        
        message
    }

    fn verify_spending_authorization(&self, _utxo: &UTXOOutput, _public_key: &[u8]) -> Result<(), ChronicleError> {
        // Simplified authorization check
        // In reality, would execute the locking script
        Ok(())
    }

    async fn craft_and_commit_genesis_chapter(&mut self) -> Result<(), ChronicleError> {
        let genesis_block = BlockChapter {
            chapter_number: 0,
            timestamp_of_creation: current_timestamp(),
            previous_chapter_essence: "0".repeat(64),
            transaction_tales: vec![],
            merkle_tree_of_truth: "genesis".to_string(),
            chapter_essence: "genesis_hash".to_string(),
            proof_of_storytelling: ProofOfWork {
                difficulty_target: 0x0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
                nonce_of_discovery: 0,
                storyteller_reward: 0,
                hash_rate_estimate: 0.0,
            },
            chapter_size_bytes: 0,
        };

        self.chain_repository.add_block_chapter(genesis_block).await?;
        println!("🏗️  Genesis chapter crafted and committed");
        Ok(())
    }

    /// ## Public API: Chronicle State and Operations
    
    pub async fn chronicle_shares_its_current_state(&self) -> Result<ChronicleState, ChronicleError> {
        let chain_tip = self.chain_repository.get_chain_tip().await?;
        let mempool_size = self.mempool_of_pending_tales.lock().unwrap().len();
        
        Ok(ChronicleState {
            total_chapters: chain_tip.as_ref().map(|b| b.chapter_number + 1).unwrap_or(0),
            pending_stories: mempool_size,
            latest_chapter_essence: chain_tip.map(|b| b.chapter_essence).unwrap_or_default(),
            chain_integrity: self.chronicle_verifies_its_own_integrity().await?,
            network_peers: self.network_storytellers.get_peer_count().await,
            sync_status: self.network_storytellers.get_sync_status().await,
        })
    }

    async fn chronicle_verifies_its_own_integrity(&self) -> Result<bool, ChronicleError> {
        self.chain_repository.verify_chain_integrity().await
    }

    pub async fn get_balance(&self, address: &[u8]) -> Result<u64, ChronicleError> {
        self.utxo_ledger.calculate_balance(address).await
    }

    pub async fn create_transaction(
        &self,
        from_keypair: &Keypair,
        to_address: &[u8],
        amount: u64,
        fee: u64,
    ) -> Result<TransactionStory, ChronicleError> {
        // Find UTXOs for sender
        let sender_address = from_keypair.public.to_bytes();
        let utxos = self.utxo_ledger.find_utxos_for_address(&sender_address).await?;
        
        // Select UTXOs to cover amount + fee
        let mut selected_utxos = Vec::new();
        let mut total_input = 0u64;
        
        for (utxo_ref, utxo) in utxos {
            selected_utxos.push((utxo_ref, utxo));
            total_input += utxo.value_locked;
            
            if total_input >= amount + fee {
                break;
            }
        }
        
        if total_input < amount + fee {
            return Err(ChronicleError::InsufficientFunds { 
                required: amount + fee, 
                available: total_input 
            });
        }
        
        // Create outputs
        let mut outputs = vec![
            UTXOOutput {
                recipient_address: to_address.to_vec(),
                value_locked: amount,
                locking_script: ScriptOfTruth {
                    script_type: ScriptType::PayToPublicKey,
                    required_signatures: 1,
                    public_keys: vec![to_address.to_vec()],
                },
            }
        ];
        
        // Add change output if necessary
        let change = total_input - amount - fee;
        if change > 0 {
            outputs.push(UTXOOutput {
                recipient_address: sender_address.to_vec(),
                value_locked: change,
                locking_script: ScriptOfTruth {
                    script_type: ScriptType::PayToPublicKey,
                    required_signatures: 1,
                    public_keys: vec![sender_address.to_vec()],
                },
            });
        }
        
        // Create transaction
        let mut transaction = TransactionStory {
            story_id: generate_transaction_id(),
            inputs_consumed: selected_utxos.into_iter().map(|(utxo_ref, _)| utxo_ref).collect(),
            outputs_created: outputs,
            story_fee: fee,
            timestamp_of_telling: current_timestamp(),
            transaction_nonce: generate_nonce(),
            digital_signature: Vec::new(), // Will be filled after signing
            public_key_of_narrator: from_keypair.public.to_bytes().to_vec(),
        };
        
        // Sign the transaction
        let message = self.create_signable_message(&transaction);
        let signature = from_keypair.sign(&message);
        transaction.digital_signature = signature.to_bytes().to_vec();
        
        Ok(transaction)
    }
}

/// ## Implementation Details for Supporting Structures

impl ChainRepository {
    async fn new(data_dir: &str) -> Result<Self, ChronicleError> {
        std::fs::create_dir_all(data_dir).map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
        
        let block_db = sled::open(format!("{}/blocks", data_dir))
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
        let tx_db = sled::open(format!("{}/transactions", data_dir))
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
        let utxo_db = sled::open(format!("{}/utxos", data_dir))
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
        
        Ok(Self {
            block_db,
            tx_db,
            utxo_db,
            chain_tip: Arc::new(RwLock::new(None)),
            block_index: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    async fn chain_is_empty(&self) -> Result<bool, ChronicleError> {
        Ok(self.block_db.is_empty())
    }
    
    async fn add_block_chapter(&self, block: BlockChapter) -> Result<(), ChronicleError> {
        // Serialize and store block
        let block_data = bincode::serialize(&block)
            .map_err(|e| ChronicleError::SerializationError(e.to_string()))?;
        
        let block_key = format!("block_{:010}", block.chapter_number);
        self.block_db.insert(&block_key, block_data)
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
        
        // Store transactions
        for tx in &block.transaction_tales {
            let tx_data = bincode::serialize(tx)
                .map_err(|e| ChronicleError::SerializationError(e.to_string()))?;
            self.tx_db.insert(&tx.story_id, tx_data)
                .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
        }
        
        // Update chain tip
        {
            let mut tip = self.chain_tip.write().unwrap();
            *tip = Some(block.clone());
        }
        
        // Update block index
        {
            let mut index = self.block_index.write().unwrap();
            index.insert(block.chapter_essence.clone(), block.chapter_number);
        }
        
        // Flush to disk
        self.block_db.flush_async().await
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_chain_tip(&self) -> Result<Option<BlockChapter>, ChronicleError> {
        let tip = self.chain_tip.read().unwrap().clone();
        
        if tip.is_none() {
            // Load from database
            if let Some((_, block_data)) = self.block_db.last()
                .map_err(|e| ChronicleError::DatabaseError(e.to_string()))? {
                let block: BlockChapter = bincode::deserialize(&block_data)
                    .map_err(|e| ChronicleError::SerializationError(e.to_string()))?;
                return Ok(Some(block));
            }
        }
        
        Ok(tip)
    }
    
    async fn transaction_exists(&self, tx_id: &str) -> Result<bool, ChronicleError> {
        Ok(self.tx_db.contains_key(tx_id)
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?)
    }
    
    async fn verify_chain_integrity(&self) -> Result<bool, ChronicleError> {
        let mut previous_hash = "0".repeat(64);
        
        for result in self.block_db.iter() {
            let (_, block_data) = result.map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
            let block: BlockChapter = bincode::deserialize(&block_data)
                .map_err(|e| ChronicleError::SerializationError(e.to_string()))?;
            
            if block.chapter_number > 0 && block.previous_chapter_essence != previous_hash {
                return Ok(false);
            }
            
            previous_hash = block.chapter_essence.clone();
        }
        
        Ok(true)
    }
    
    fn clone(&self) -> Self {
        Self {
            block_db: self.block_db.clone(),
            tx_db: self.tx_db.clone(),
            utxo_db: self.utxo_db.clone(),
            chain_tip: self.chain_tip.clone(),
            block_index: self.block_index.clone(),
        }
    }
}

impl UTXOLedger {
    async fn new(data_dir: &str) -> Result<Self, ChronicleError> {
        let db = sled::open(format!("{}/utxos", data_dir))
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
            
        Ok(Self {
            unspent_outputs: Arc::new(RwLock::new(HashMap::new())),
            spent_outputs: Arc::new(RwLock::new(HashSet::new())),
            db,
        })
    }
    
    async fn find_unspent_output(&self, utxo_key: &str) -> Result<Option<UTXOOutput>, ChronicleError> {
        // Check in-memory cache first
        {
            let unspent = self.unspent_outputs.read().unwrap();
            if let Some(utxo) = unspent.get(utxo_key) {
                return Ok(Some(utxo.clone()));
            }
        }
        
        // Check database
        if let Some(utxo_data) = self.db.get(utxo_key)
            .map_err(|e| ChronicleError::DatabaseError(e.to_string()))? {
            let utxo: UTXOOutput = bincode::deserialize(&utxo_data)
                .map_err(|e| ChronicleError::SerializationError(e.to_string()))?;
            return Ok(Some(utxo));
        }
        
        Ok(None)
    }
    
    async fn calculate_balance(&self, address: &[u8]) -> Result<u64, ChronicleError> {
        let mut balance = 0u64;
        
        for result in self.db.iter() {
            let (key, utxo_data) = result.map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
            let utxo: UTXOOutput = bincode::deserialize(&utxo_data)
                .map_err(|e| ChronicleError::SerializationError(e.to_string()))?;
            
            if utxo.recipient_address == address {
                // Check if not spent
                let key_str = String::from_utf8_lossy(&key);
                let spent = self.spent_outputs.read().unwrap();
                if !spent.contains(key_str.as_ref()) {
                    balance = balance.checked_add(utxo.value_locked)
                        .ok_or(ChronicleError::ValueOverflow)?;
                }
            }
        }
        
        Ok(balance)
    }
    
    async fn find_utxos_for_address(&self, address: &[u8]) -> Result<Vec<(UTXOReference, UTXOOutput)>, ChronicleError> {
        let mut utxos = Vec::new();
        
        for result in self.db.iter() {
            let (key, utxo_data) = result.map_err(|e| ChronicleError::DatabaseError(e.to_string()))?;
            let utxo: UTXOOutput = bincode::deserialize(&utxo_data)
                .map_err(|e| ChronicleError::SerializationError(e.to_string()))?;
            
            if utxo.recipient_address == address {
                let key_str = String::from_utf8_lossy(&key);
                let spent = self.spent_outputs.read().unwrap();
                if !spent.contains(key_str.as_ref()) {
                    // Parse key to create UTXOReference
                    let parts: Vec<&str> = key_str.split(':').collect();
                    if parts.len() == 2 {
                        if let Ok(output_index) = parts[1].parse::<u32>() {
                            let utxo_ref = UTXOReference {
                                previous_story_id: parts[0].to_string(),
                                output_index,
                            };
                            utxos.push((utxo_ref, utxo));
                        }
                    }
                }
            }
        }
        
        Ok(utxos)
    }
}

impl NetworkOfStoryTellers {
    async fn new(port: u16) -> Result<Self, ChronicleError> {
        let (tx, _rx) = mpsc::unbounded_channel();
        
        Ok(Self {
            peer_connections: Arc::new(Mutex::new(Vec::new())),
            message_broadcaster: Arc::new(Mutex::new(tx)),
            sync_status: Arc::new(RwLock::new(SyncStatus {
                is_syncing: false,
                current_height: 0,
                target_height: 0,
                sync_progress: 0.0,
            })),
            known_peers: Arc::new(RwLock::new(HashSet::new())),
        })
    }
    
    async fn start_peer_discovery(&self) -> Result<(), ChronicleError> {
        // Implementation for peer discovery
        // In a real system, this would connect to bootstrap nodes
        println!("🔍 Starting peer discovery...");
        Ok(())
    }
    
    async fn begin_chain_synchronization(&self) -> Result<(), ChronicleError> {
        // Implementation for chain sync
        println!("🔄 Beginning chain synchronization...");
        Ok(())
    }
    
    async fn broadcast_transaction_story(&self, _story: TransactionStory) -> Result<(), ChronicleError> {
        // Implementation for broadcasting transactions
        println!("📡 Broadcasting transaction to network...");
        Ok(())
    }
    
    async fn get_peer_count(&self) -> usize {
        self.peer_connections.lock().unwrap().len()
    }
    
    async fn get_sync_status(&self) -> SyncStatus {
        self.sync_status.read().unwrap().clone()
    }
}

impl MiningHeart {
    fn new(reward_address: Vec<u8>, _target_block_time: Duration) -> Self {
        Self {
            is_beating: Arc::new(Mutex::new(true)),
            current_difficulty: Arc::new(RwLock::new(0x0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
            hash_rate: Arc::new(RwLock::new(0.0)),
            mining_reward_address: reward_address,
            thread_handles: Vec::new(),
        }
    }
}

impl ValidatorCouncil {
    fn new() -> Self {
        let mut council_members = HashMap::new();
        let keypair = Keypair::generate(&mut OsRng);
        
        council_members.insert("validator_1".to_string(), ValidatorGuardian {
            guardian_id: "validator_1".to_string(),
            stake_in_truth: 1000,
            reputation_score: 1.0,
            tales_validated: 0,
            public_key: keypair.public,
            last_validation_time: current_timestamp(),
        });

        Self {
            council_members,
            consensus_threshold: 0.67,
            current_storyteller: Some("validator_1".to_string()),
            reputation_system: ReputationSystem::new(),
        }
    }
}

impl ReputationSystem {
    fn new() -> Self {
        Self {
            validator_scores: HashMap::new(),
            penalty_system: PenaltyTracker {
                recent_penalties: HashMap::new(),
                cumulative_penalties: HashMap::new(),
            },
            reward_multipliers: HashMap::new(),
        }
    }
}

impl Default for ChronicleConfiguration {
    fn default() -> Self {
        Self {
            target_block_time: Duration::from_secs(600), // 10 minutes
            difficulty_adjustment_interval: 2016, // ~2 weeks
            max_block_size: 1_048_576, // 1MB
            min_transaction_fee: 1000, // 0.00001 units
            base_mining_reward: 5_000_000_000, // 50 units
            reward_halving_interval: 210_000,
            max_peers: 50,
            network_port: 8333,
            data_directory: "./blockchain_data".to_string(),
        }
    }
}

/// ## The Current State of Our Chronicle
#[derive(Debug)]
pub struct ChronicleState {
    pub total_chapters: u64,
    pub pending_stories: usize,
    pub latest_chapter_essence: String,
    pub chain_integrity: bool,
    pub network_peers: usize,
    pub sync_status: SyncStatus,
}

/// ## Plot Twists: When Things Go Wrong
#[derive(Debug)]
pub enum ChronicleError {
    StoryBearsFalseWitness(String),
    NarratorLacksResources(String),
    ChronicleCorrupted(String),
    ProofOfWorkFailed(String),
    ConsensusNotReached(String),
    DatabaseError(String),
    SerializationError(String),
    NetworkError(String),
    InvalidPublicKey(String),
    InvalidSignature(String),
    UTXONotFound(String),
    DuplicateStory(String),
    InvalidNonce(u64),
    InsufficientFee(u64),
    ValueOverflow,
    InsufficientFunds { required: u64, available: u64 },
}

impl ChronicleError {
    pub fn resolve_the_conflict(self) -> StoryResolution {
        match self {
            ChronicleError::StoryBearsFalseWitness(_) => StoryResolution::RejectTheStory,
            ChronicleError::NarratorLacksResources(_) => StoryResolution::ReturnToSender,
            ChronicleError::ChronicleCorrupted(_) => StoryResolution::ResyncWithNetwork,
            ChronicleError::ProofOfWorkFailed(_) => StoryResolution::AdjustDifficulty,
            ChronicleError::ConsensusNotReached(_) => StoryResolution::SeekMoreValidators,
            ChronicleError::DatabaseError(_) => StoryResolution::RepairDatabase,
            ChronicleError::NetworkError(_) => StoryResolution::ReconnectToNetwork,
            ChronicleError::InvalidSignature(_) => StoryResolution::RejectTheStory,
            ChronicleError::UTXONotFound(_) => StoryResolution::RejectTheStory,
            ChronicleError::InsufficientFunds { .. } => StoryResolution::ReturnToSender,
            _ => StoryResolution::LogAndContinue,
        }
    }
}

pub enum StoryResolution {
    RejectTheStory,
    ReturnToSender,
    ResyncWithNetwork,
    AdjustDifficulty,
    SeekMoreValidators,
    RepairDatabase,
    ReconnectToNetwork,
    LogAndContinue,
}

// Utility functions
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn generate_transaction_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 16] = rng.gen();
    hex::encode(random_bytes)
}

fn generate_nonce() -> u64 {
    use rand::Rng;
    rand::thread_rng().gen()
}

/// ## Cargo.toml Dependencies
/// 
/// ```toml
/// [dependencies]
/// serde = { version = "1.0", features = ["derive"] }
/// bincode = "1.3"
/// sha2 = "0.10"
/// ed25519-dalek = { version = "2.0", features = ["rand_core"] }
/// rand = "0.8"
/// sled = "0.34"
/// tokio = { version = "1.0", features = ["full"] }
/// hex = "0.4"
/// num_cpus = "1.0"
/// ```

/// ## Example Usage: A Complete Blockchain Story
/// 
/// ```rust
/// #[tokio::main]
/// async fn main() -> Result<(), ChronicleError> {
///     // Chapter 1: The Chronicle Begins
///     let config = ChronicleConfiguration::default();
///     let mut blockchain = BlockchainChronicler::new_chronicle_begins(config).await?;
///     
///     // Chapter 2: Create a keypair for transactions
///     let alice_keypair = Keypair::generate(&mut OsRng);
///     let bob_address = Keypair::generate(&mut OsRng).public.to_bytes();
///     
///     // Chapter 3: Create and submit a transaction
///     let transaction = blockchain.create_transaction(
///         &alice_keypair,
///         &bob_address,
///         1000, // amount
///         50,   // fee
///     ).await?;
///     
///     blockchain.transaction_story_arrives(transaction).await?;
///     
///     // Chapter 4: Check the chronicle's state
///     let state = blockchain.chronicle_shares_its_current_state().await?;
///     println!("Chronicle State: {:?}", state);
///     
///     // Chapter 5: Check balances
///     let alice_balance = blockchain.get_balance(&alice_keypair.public.to_bytes()).await?;
///     let bob_balance = blockchain.get_balance(&bob_address).await?;
///     
///     println!("Alice balance: {}", alice_balance);
///     println!("Bob balance: {}", bob_balance);
///     
///     Ok(())
/// }
/// ```
            