initSidebarItems({"constant":[["EPOCHS_IN_DAY",""],["EPOCHS_IN_HOUR",""],["EPOCHS_IN_YEAR",""],["EPOCH_DURATION_SECONDS","Duration of each tipset epoch."],["EXPECTED_LEADERS_PER_EPOCH","The expected number of block producers in each epoch."],["FIRST_NON_SINGLETON_ADDR","Defines first available ID address after builtin actors"],["SECONDS_IN_DAY",""],["SECONDS_IN_HOUR",""],["SECONDS_IN_YEAR",""],["SECTOR_QUALITY_PRECISION","Precision used for making QA power calculations"]],"enum":[["ExitCode","ExitCode defines the exit code from the VM execution"]],"fn":[["is_builtin_actor","Returns true if the code `Cid` belongs to a builtin actor."],["is_principal",""],["is_singleton_actor","Returns true if the code belongs to a singleton actor."],["make_map","Create a hamt configured with constant bit width."],["make_map_with_root","Create a map with a root cid."],["parse_uint_key",""],["u64_key",""]],"macro":[["actor_error","Convenience macro for generating Actor Errors"]],"mod":[["account",""],["chaos",""],["cron",""],["init",""],["market",""],["math",""],["miner",""],["multisig",""],["network",""],["paych",""],["power",""],["reward",""],["smooth",""],["system",""],["verifreg",""]],"struct":[["ACCOUNT_ACTOR_CODE_ID",""],["ActorError","The error type that gets returned by actor method calls."],["ActorState","State of all actor implementations"],["BURNT_FUNDS_ACTOR_ADDR","Distinguished AccountActor that is the destination of all burnt funds."],["BalanceTable","Balance table which handles getting and updating token balances specifically"],["CALLER_TYPES_SIGNABLE",""],["CHAOS_ACTOR_ADDR",""],["CHAOS_ACTOR_CODE_ID",""],["CRON_ACTOR_ADDR",""],["CRON_ACTOR_CODE_ID",""],["DEAL_WEIGHT_MULTIPLIER","Quality multiplier for unverified deals in a sector"],["INIT_ACTOR_ADDR",""],["INIT_ACTOR_CODE_ID",""],["MARKET_ACTOR_CODE_ID",""],["MINER_ACTOR_CODE_ID",""],["MULTISIG_ACTOR_CODE_ID",""],["Multimap","Multimap stores multiple values per key in a Hamt of Amts. The order of insertion of values for each key is retained."],["PAYCH_ACTOR_CODE_ID",""],["POWER_ACTOR_CODE_ID",""],["QUALITY_BASE_MULTIPLIER","Quality multiplier for committed capacity (no deals) in a sector"],["REWARD_ACTOR_ADDR",""],["REWARD_ACTOR_CODE_ID",""],["STORAGE_MARKET_ACTOR_ADDR",""],["STORAGE_POWER_ACTOR_ADDR",""],["SYSTEM_ACTOR_ADDR",""],["SYSTEM_ACTOR_CODE_ID",""],["Serialized","Serialized bytes to be used as parameters into actor methods"],["Set","Set is a Hamt with empty values for the purpose of acting as a hash set."],["SetMultimap","SetMultimap is a hamt with values that are also a hamt but are of the set variant. This allows hash sets to be indexable by an address."],["TOTAL_FILECOIN","The maximum supply of Filecoin that will ever exist (in token units)"],["VERIFIED_DEAL_WEIGHT_MULTIPLIER","Quality multiplier for verified deals in a sector"],["VERIFIED_REGISTRY_ACTOR_ADDR",""],["VERIFREG_ACTOR_CODE_ID",""]],"trait":[["ActorDowncast","Trait to allow multiple error types to be able to be downcasted into an `ActorError`."]],"type":[["DealID","Deal identifier used in market and miner actors"],["Map","Map type to be used within actors. The underlying type is a hamt."],["TokenAmount",""]]});