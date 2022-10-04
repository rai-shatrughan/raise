 #!/bin/bash
 
root_path=/data
data_folders=("kafka" "zookeeper" "etcd1")

1create_network() {
    docker network create \
        --driver=bridge \
        --subnet=172.18.0.0/23 \
        sr_cluster_network
}

2down_containers() {
    docker-compose --env-file .env -f docker-compose.yml down
}

remove_dir() {
    wd=$1    
    if [[ -d $wd ]]; then
        echo "Removing " $wd
        sudo rm -rf $wd
    fi
}

crate_dir(){
    wd=$1
    sudo mkdir -p $wd
    sudo chown nobody:nogroup $wd
    sudo chmod 777 $wd -R
    echo "Created " $wd
}

3recreate_dir(){
    for dir in ${data_folders[@]}; do
        wd=$root_path/$dir    
        remove_dir $wd
        crate_dir  $wd  
    done
}

4build_rust(){
    cd ..
    echo "building rust binary"

    cd ws
    cargo install --target x86_64-unknown-linux-musl --path .
    cd ..
    
    cd cons
    cargo install --target x86_64-unknown-linux-musl --path .
    cd ..
    
    mkdir -p 0docker/app/bin/    
    cp target/x86_64-unknown-linux-musl/release/ws 0docker/app/bin/ws  
    cp target/x86_64-unknown-linux-musl/release/cons 0docker/app/bin/cons  

    cd ui
    trunk build --release
    cd ../0docker
}

5recreate_containers() {
    docker-compose --env-file .env -f docker-compose.yml build 
    docker-compose --env-file .env -f docker-compose.yml up -d
}

6setup_db() {
    #configure for 1st time usage.
    echo "sleeping for 5s"
    sleep 5
    echo "create Topic*****"
    echo
    bash kafka/createTopic.sh
}

7clean_rust_bin(){
    echo "removing app/bin/"
    rm -rf app/bin/
}

1create_network
2down_containers
3recreate_dir
4build_rust
5recreate_containers
6setup_db
7clean_rust_bin