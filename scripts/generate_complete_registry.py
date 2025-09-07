#!/usr/bin/env python3
"""
Script to generate complete token registry from the unified token registry
This extracts all tokens and filters them to only include chains we support
"""

# Supported chains in our Rust multichain wallet
SUPPORTED_CHAINS = {
    'Bitcoin': ['BitcoinLegacy', 'BitcoinSegwit', 'BitcoinTaproot'],
    'Ethereum': ['Ethereum'],
    'Solana': ['Solana'],
    'Tron': ['Tron'],
    'Dogecoin': ['Dogecoin'],
    'Filecoin': ['Filecoin'],
    'Cosmos': ['Cosmos'],
    'Osmosis': ['Osmosis'],
    'Secret': ['Secret'],
    'Juno': ['Juno'],
    'Akash': ['Akash'],
    'Celestia': ['Celestia'],
    'Sei': ['Sei'],
    'Injective': ['Injective'],
    'Near': ['Near'],
    'Sui': ['Sui'],
    'Ripple': ['Ripple'],
    'Tezos': ['Tezos'],
}

# Additional tokens to add (from unified registry that have deployments on our supported chains)
ADDITIONAL_TOKENS = {
    'LDO': {
        'name': 'Lido DAO',
        'cmc_id': 8000,
        'category': 'DeFi',
        'ethereum_address': '0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32'
    },
    'IMX': {
        'name': 'Immutable X',
        'cmc_id': 10603,
        'category': 'Layer 2',
        'ethereum_address': '0xF57e7e7C23978C3cAEC3C3548E3D615c346e79fF'
    },
    'ENA': {
        'name': 'Ethena',
        'cmc_id': 30171,
        'category': 'DeFi',
        'ethereum_address': '0x57e114B691Db790C35207b2e685D4A43181e6061'
    },
    'ONDO': {
        'name': 'Ondo Finance',
        'cmc_id': 21159,
        'category': 'RWA',
        'ethereum_address': '0xfAbA6f8e4a5E8Ab82F62fe7C39859FA577269BE3'
    },
    'HYPE': {
        'name': 'Hyperliquid',
        'cmc_id': 33021,
        'category': 'DEX',
        'ethereum_address': '0xEa66501Df1a00261e3bB79D1e90444fc6C7104e7'
    },
    'MNT': {
        'name': 'Mantle',
        'cmc_id': 27075,
        'category': 'Layer 2',
        'ethereum_address': '0x3c3a81e81dc49A522A592e7622A7E711c06bf354'
    },
}

def generate_rust_token_code():
    """Generate the Rust code for additional tokens"""
    
    code = """
        // ============ ADDITIONAL ERC-20 TOKENS ============
        
"""
    
    for symbol, info in ADDITIONAL_TOKENS.items():
        code += f"""        // {info['name']}
        self.add_token(UnifiedToken {{
            base_symbol: "{symbol}".to_string(),
            name: "{info['name']}".to_string(),
            logo: "https://s2.coinmarketcap.com/static/img/coins/64x64/{info['cmc_id']}.png".to_string(),
            cmc_id: Some({info['cmc_id']}),
            coingecko_id: None,
            category: "{info['category']}".to_string(),
            is_multi_chain: false,
            asset_type: AssetType::Token,
            deployments: vec![
                ChainDeployment {{
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("{info['ethereum_address']}".to_string()),
                    decimals: 18,
                    symbol: "{symbol}".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                    chain_logo: Some("https://s2.coinmarketcap.com/static/img/coins/64x64/1027.png".to_string()),
                }},
            ],
        }});

"""
    
    return code

if __name__ == "__main__":
    print("Generating additional tokens for Rust token registry...")
    code = generate_rust_token_code()
    print(code)
    print("\nAdd this code to the initialize_defi_tokens() function in token_registry.rs")