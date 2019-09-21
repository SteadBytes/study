package main

import "fmt"

func main() {
	var a int32 = 2147483647
	fmt.Printf("a = %d\n", a)
	fmt.Printf("(a + 1) <= a = false, got %t\n", (a+1) <= a)
	fmt.Printf("(%d + 1) = 21474836478, got %d\n", a, a+1)
}
