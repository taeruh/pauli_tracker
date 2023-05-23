#!/usr/bin/bash


PRODIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )


function check_array {
  local seeking=$1
  shift
  local in=1
  for element in "$@"; do
    if [[ $element == "$seeking" ]]; then
      in=0
      break
    fi
  done
  return $in
}


function update_build_directory {
  for d in $(find src -type d); do
    mkdir -p "$(echo $d | sed 's|src|target/build|')"
  done
  for d in $(find target/build -type d); do
    echo $d
    if [ -z "$(find src -type d -wholename $(echo $d|sed 's|target/build|src|'))" ]; then
      echo $d
      rm -rf $d
    elif ! [ -f $d/.gitkeep ]; then
      touch $d/.gitkeep
    fi
  done
}




ACTIONS=($(declare -F | awk '{ print $3}' | grep -v 'checkArray'))

if [ -z ${@+x} ]; then
  echo "require positional argument action from {${ACTIONS[@]}}"
fi


# not a really nice help message, but sufficient for completion
while getopts :h flag
  do
    case $flag in
      h) echo ${ACTIONS[@]};;
    esac
  done
shift $((OPTIND - 1))

for a in "$@"; do
  if check_array "$a" "${ACTIONS[@]}" ; then
    echo $a:
    $a
    echo 'done'
  else
    echo  "$a is no valid action (action in {${ACTIONS[@]}})"
  fi
done
