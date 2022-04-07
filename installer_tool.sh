#!/bin/bash

RED='\033[0;31m'
NB='\033[0m'
BLU='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
LCYAN='\033[0;36m'
PURPLE='\033[0;35m'


function ascii_banner(){
    clear
    echo -e "${BLU}####                  #            #   #    #             #     #             ## "
    echo -e "${BLU}#   #                 #            #   #                        #              # "
    echo -e "${BLU}#   #  #   #   ###   ####   #   #  ## ##   ##    # ##    ##    ####    ###     # "
    echo -e "${BLU}####   #   #  #       #     #   #  # # #    #    ##  #    #     #     #   #    # "
    echo -e "${BLU}# #    #   #   ###    #     #  ##  #   #    #    #   #    #     #     #####    # "
    echo -e "${BLU}#  #   #  ##      #   #  #   ## #  #   #    #    #   #    #     #  #  #        #"
    echo -e "${BLU}#   #   ## #  ####     ##       #  #   #   ###   #   #   ###     ##    ###    ###"
    echo -e "${BLU}                            #   #                                                 "
    echo -e "${BLU}                            ###                                                 "
    
}

function startup(){
    
    ascii_banner
    echo -e "${RED}[1] Install RustyMinitel   [2] Yabadaboopbidiboop"
    while true ; do
        
        read key
        if [[ $key -eq 1 ]]; then
            break
        fi
        if [[ $key -eq 2 ]]; then
            echo -en "\007"
        fi
        
    done
    
}


function create_user(){
    read -p "Choose your username : " name
    
    sudo useradd -g users -m $name
    sudo passwd $name
    sudo mkdir -p ../../../$name/binary
    echo "$name ALL=(ALL) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/$name
    sudo mv target/release/rustyminitel /usr/bin
    sudo touch /home/$name/.bashrc
    sudo bash -c 'echo "export PATH=\"/usr/bin:${PATH}" >> /home/$name/.bashrc'
    echo "rustyminitel"  | sudo tee -a /home/$name/.profile
    
    
    
}



function main(){
    startup
    clear
    echo -e "${GREEN} In progress..."
    #sudo apt install curl && curl https://sh.rustup.rs -sSf | sh
    cd rustyminitel/ || exit
    cargo build --release && chmod 755 target/release/rustyminitel.d
    create_user
    
    
    echo -e "${BLU} Installation done ! Now you can connect to the user"
    
}



main