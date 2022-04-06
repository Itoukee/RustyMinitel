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
    mkdir ../../../$name/binary
    
    mv target/release/rustyminitel ../../../$name/binary/
    touch ../../../$name/.bashrc
    echo "export PATH=\"${HOME}/bin:${PATH}" >> ../../../$name/.bashrc
    
    
    
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