from iota_wallet import IotaWallet, StrongholdSecretManager, init_logger
import json

# This example creates a new database and account and write debug logs in `wallet.log`.

log_config = {
    "name": './wallet.log',
    "levelFilter": 'debug',
    "targetExclusions": ["h2", "hyper", "rustls"]
}

# Init the logger
init_logger(json.dumps(log_config))

client_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}

# Shimmer coin type
coin_type = 4219

secret_manager = StrongholdSecretManager(
    "wallet.stronghold", "some_hopefully_secure_password")

wallet = IotaWallet('./alice-database', client_options,
                    coin_type, secret_manager)

# Store the mnemonic in the Stronghold snapshot, this only needs to be done once
account = wallet.store_mnemonic("flame fever pig forward exact dash body idea link scrub tennis minute " +
                                "surge unaware prosper over waste kitten ceiling human knife arch situate civil")

account = wallet.create_account('Alice')
print(account)
