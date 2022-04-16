package main

import "runtime/debug"

func a(){
    debug.PrintStack()
}

func b(){
    a()
}

func main(){
    b()
}
