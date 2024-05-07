source ./keys.sh
KEY_INSERT="../../target/release/clarus-node key insert --chain clarus"
BASE_PATH="../../data"

# insert session keys
$KEY_INSERT --base-path "${BASE_PATH}/validator1" --scheme ed25519 --suri "$V1_GRAN" --key-type gran
$KEY_INSERT --base-path "${BASE_PATH}/validator1" --scheme sr25519 --suri "$V1_IMOL" --key-type imol
$KEY_INSERT --base-path "${BASE_PATH}/validator1" --scheme sr25519 --suri "$V1_BABE" --key-type babe
$KEY_INSERT --base-path "${BASE_PATH}/validator1" --scheme sr25519 --suri "$V1_AUDI" --key-type audi

$KEY_INSERT --base-path "${BASE_PATH}/validator2" --scheme ed25519 --suri "$V2_GRAN" --key-type gran
$KEY_INSERT --base-path "${BASE_PATH}/validator2" --scheme sr25519 --suri "$V2_IMOL" --key-type imol
$KEY_INSERT --base-path "${BASE_PATH}/validator2" --scheme sr25519 --suri "$V2_BABE" --key-type babe
$KEY_INSERT --base-path "${BASE_PATH}/validator2" --scheme sr25519 --suri "$V2_AUDI" --key-type audi

$KEY_INSERT --base-path "${BASE_PATH}/validator3" --scheme ed25519 --suri "$V3_GRAN" --key-type gran
$KEY_INSERT --base-path "${BASE_PATH}/validator3" --scheme sr25519 --suri "$V3_IMOL" --key-type imol
$KEY_INSERT --base-path "${BASE_PATH}/validator3" --scheme sr25519 --suri "$V3_BABE" --key-type babe
$KEY_INSERT --base-path "${BASE_PATH}/validator3" --scheme sr25519 --suri "$V3_AUDI" --key-type audi

# Insert offchain worker keys

