#!/bin/bash

bin=glitter
targetbin=$bin

if path=$(command -v glitter); then
    target=$(dirname "$path")
else
    target=~/.local/bin
fi

while getopts "t:n:hu" opt; do
    case $opt in
        t) target=$OPTARG;;
        n) targetbin=$OPTARG;;
        u)
            echo 'Uninstalling...'
            rm $(which $targetbin)
            echo 'Done'
            exit 0
            ;;
        h)
            echo 'Glitter install script'
            echo -e "-t\tTarget of the installation (default: $target)"
            echo -e "-n\tName of the installed binary (default: $targetbin)"
            echo -e '-u\tUninstall glitter'
            echo -e '-h\tShow this message'
            exit 0
            ;;
    esac
done

echo 'Compiling...'
CGO_ENABLED=0 go build -ldflags="-s -w" -trimpath

installdir=${target%/}/$targetbin
echo "Installing to $installdir..."
install -Dm755 glitter $installdir

echo 'Done!'
