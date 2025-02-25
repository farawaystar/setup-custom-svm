# Clone the repository with --filter=blob:none to avoid downloading large files
git clone --no-local --filter=blob:none https://github.com/anza-xyz/agave.git agave-filtered
cd agave-filtered

# Create a new branch
git checkout -b filtered-branch

# Remove all files and folders
git rm -rf .

# Restore only the folders you want
git checkout HEAD -- \
    faucet \
    curve25519 \
    core \
    runtime \
    svm-conformance \
    storage-bigtable \
    account-decoder-client-types \
    inline-spl \
    metrics \
    rpc \
    system-program \
    loader-v4-program \
    zk-token-sdk \
    poseidon \
    quic-client \
    zk-sdk \
    unified-scheduler-logic \
    programs/zk-token-proof \
    udp-client \
    program-runtime \
    config-program \
    unified-scheduler-pool \
    geyser-plugin-manager \
    cli-config \
    runtime-transaction \
    rpc-client-api \
    ledger \
    poh \
    agave-thread-manager \
    tpu-client \
    stake-program \
    bucket-map \
    connection-cache \
    agave-transaction-view \
    zk-token-proof-program \
    cost-model \
    builtins-default-costs \
    banking-stage-ingress-types \
    transaction-metrics-tracker \
    log-collector \
    programs/address-lookup-table \
    bloom \
    rpc-client-nonce-utils \
    curves/curve25519 \
    lattice-hash \
    timings \
    bucket_map \
    fee \
    compute-budget-program \
    entry \
    wen-restart \
    perf \
    programs/config \
    accounts-db \
    type-overrides \
    compute-budget \
    builtins \
    zk-elgamal-proof-program \
    svm-transaction \
    programs/system \
    svm-rent-collector \
    merkle-tree \
    programs/vote \
    net-utils \
    account-decoder \
    transaction-status \
    thin-client \
    send-transaction-service \
    vote \
    programs/bpf_loader \
    programs/compute-budget \
    streamer \
    version \
    geyser-plugin-interface \
    turbine \
    rpc-client \
    rayon-threadlimit \
    storage-proto \
    tls-utils \
    svm \
    client \
    vote-program \
    compute-budget-instruction \
    programs/stake \
    agave-banking-stage-ingress-types \
    transaction-status-client-types \
    programs/loader-v4 \
    address-lookup-table-program \
    gossip \
    transaction-view \
    programs/zk-elgamal-proof \
    remote-wallet \
    agave-geyser-plugin-interface \
    measure \
    pubsub-client \
    clap-utils \
    bpf-loader-program \
    thread-manager \
    tpu-client-next

# Commit these changes
git add .
git commit -m "Filter repository to include only specific folders"

# Create a new independent repository
cd ..
mkdir agave-independent-f
cd agave-independent-f
git init

# Pull in the changes from the filtered content
git remote add origin ../agave-filtered
git pull origin master

# Remove the remote to detach from the original repository
git remote remove origin

# Clean up: Remove the temporary agave-filtered directory
cd ..
# rm -rf agave-filtered

echo "Independent repository created in 'agave-independent-f' directory."
