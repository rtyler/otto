#!/bin/sh

err() {
    printf '\e[48;5;%dm' 196
    echo -n ">> ${1}"
    printf '\e[0m \n'
}
ok() {
    printf '\e[48;5;%dm' 034
    echo -n ">> ${1}"
    printf '\e[0m \n'
}

check_bin() {
    which $1 &> /dev/null
    if [ $? -ne 0 ]; then
        err "Please ensure that \`${1}\` is in your PATH"
        exit 1;
    else
        ok "${1} is present"
    fi;
}

check_bin 'wget'
check_bin 'java'
check_bin 'rustc'
check_bin 'cargo'
