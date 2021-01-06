#!/bin/bash

function read_dir() {
   for file in `ls $1`
   do
       if [ "`ls -A $1"/"$file`" != "" -a -d $1"/"$file ]
       then
           read_dir $1"/"$file
       else
           echo rustfmt $1"/"$file
           rustfmt $1"/"$file
           cargo clean
       fi
   done
}
read_dir $1
