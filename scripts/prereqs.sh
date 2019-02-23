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

which wget &> /dev/null
if [ $? -ne 0 ]; then
    err "Please ensure that \`wget\` is in your PATH"
    exit 1;
else
    ok "wget is present"
fi;

which java &> /dev/null
if [ $? -ne 0 ]; then
    err "Please ensure that \`java\` is in your PATH"
    exit 1;
else
    ok "Java is present"
fi;

