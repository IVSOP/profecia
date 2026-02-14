mkdir -p blockchain_program/target/deploy
cp PRIVATE_KEY/*.json /workspace/target/deploy/blockchain_program-keypair.json

cargo build-sbf

rm -rf .surfpool && surpool start
